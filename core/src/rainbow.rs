use crate::{
    hashing::{Algorithm, HashError},
    password::PasswordError,
};
use anyhow::Result;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};
use thiserror::Error;
use tracing::{debug, error, info, instrument, warn};

const MAGIC_WORD: &[u8] = b"rainbowtable";
const VERSION: u8 = 1;

type RainbowTableData = (
    Algorithm,
    usize,
    usize,
    u128,
    u128,
    HashMap<Vec<u8>, Vec<u8>>,
);

#[derive(Error, Debug)]
pub enum RainbowError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Password error: {0}")]
    Password(#[from] PasswordError),
    #[error("Thread pool error: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
    #[error("Empty input file")]
    EmptyInput,
    #[error("Passwords have varying lengths")]
    VaryingLengths,
    #[error("Invalid magic number")]
    InvalidMagic,
    #[error("Invalid header")]
    InvalidHeader,
    #[error("Invalid algorithm")]
    InvalidAlgorithm,
    #[error("Invalid chain data")]
    InvalidChainData,
    #[error("Algorithm mismatch")]
    AlgorithmMismatch,
    #[error("Password length mismatch")]
    PasswordLengthMismatch,
    #[error("No passwords found")]
    NoPasswordsFound,
    #[error("Unicode conversion error: {0}")]
    UnicodeError(String),
    #[error("Invalid numeric value: {0}")]
    NumericError(String),
}

#[instrument(skip_all)]
pub fn generate_rainbow_table(
    input_path: &Path,
    output_path: &Path,
    algorithm: Algorithm,
    num_links: usize,
    threads: usize,
    charset_size: u128,
    unicode_offset: u128,
) -> Result<(), RainbowError> {
    info!("Generating rainbow table from: {}", input_path.display());
    debug!(
        "Algorithm: {:?}, Links: {}, Threads: {}, Charset: {}, Offset: {}",
        algorithm, num_links, threads, charset_size, unicode_offset
    );

    let passwords = crate::password::read_passwords(input_path)?;
    if passwords.is_empty() {
        error!("Empty input file: {}", input_path.display());
        return Err(RainbowError::EmptyInput);
    }

    let password_length = passwords[0].len();
    for (i, pwd) in passwords.iter().enumerate() {
        if pwd.len() != password_length {
            warn!(
                "Password at index {} has invalid length: expected {}, got {}",
                i,
                password_length,
                pwd.len()
            );
            return Err(RainbowError::VaryingLengths);
        }
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?;

    let chains: Vec<(Vec<u8>, Vec<u8>)> = pool.install(|| {
        passwords
            .par_iter()
            .map(|start| -> Result<_, RainbowError> {
                debug!("Processing chain for password: {}", start);
                let mut current = start.clone();
                for _ in 0..num_links {
                    let hash = compute_hash_rainbow(&current, algorithm)?;
                    current = reduce(&hash, password_length, charset_size, unicode_offset)?;
                }
                Ok((start.as_bytes().to_vec(), current.into_bytes()))
            })
            .collect::<Result<Vec<_>, _>>()
    })?;

    let mut file = File::create(output_path)?;
    write_header(
        &mut file,
        algorithm,
        password_length,
        charset_size,
        num_links,
        unicode_offset,
    )?;

    let num_chains = chains.len();

    for (start, end) in &chains {
        file.write_all(start)?;
        file.write_all(end)?;
    }

    info!(
        "Successfully generated rainbow table with {} chains",
        num_chains
    );

    Ok(())
}

fn compute_hash_rainbow(password: &str, algorithm: Algorithm) -> Result<Vec<u8>, RainbowError> {
    crate::hashing::compute_hash(password, algorithm).map_err(|e| match e {
        HashError::Scrypt(_) => RainbowError::InvalidAlgorithm,
        HashError::InvalidAlgorithm => RainbowError::InvalidAlgorithm,
        HashError::Io(e) => RainbowError::Io(e),
        HashError::Password(e) => RainbowError::Password(e),
        HashError::ThreadPool(e) => RainbowError::ThreadPool(e),
    })
}

fn reduce(
    hash: &[u8],
    password_length: usize,
    charset_size: u128,
    offset: u128,
) -> Result<String, RainbowError> {
    let mut password = String::with_capacity(password_length);
    let mut hash_cycle = hash.iter().cycle();

    for _ in 0..password_length {
        let mut code_point = offset;

        for _ in 0..4 {
            code_point = code_point
                .wrapping_shl(8)
                .wrapping_add(*hash_cycle.next().unwrap_or(&0) as u128);
        }

        code_point = offset + (code_point % charset_size);

        let c = char::from_u32(code_point as u32).ok_or_else(|| {
            RainbowError::UnicodeError(format!("Invalid Unicode code point: {}", code_point))
        })?;

        if !c.is_ascii() {
            return Err(RainbowError::UnicodeError(format!(
                "Non-ASCII character generated: {}",
                c
            )));
        }

        password.push(c);
    }

    Ok(password)
}

fn write_header(
    file: &mut File,
    algorithm: Algorithm,
    password_length: usize,
    charset_size: u128,
    num_links: usize,
    offset: u128,
) -> Result<(), RainbowError> {
    if password_length > u8::MAX as usize {
        return Err(RainbowError::InvalidHeader);
    }

    file.write_all(MAGIC_WORD)?;
    file.write_all(&[VERSION])?;

    let algorithm_str = algorithm.to_string();
    file.write_all(&[algorithm_str.len() as u8])?;
    file.write_all(algorithm_str.as_bytes())?;

    file.write_all(&[password_length as u8])?;
    file.write_all(&charset_size.to_be_bytes())?;
    file.write_all(&(num_links as u128).to_be_bytes())?;
    file.write_all(&offset.to_be_bytes())?;

    Ok(())
}

#[instrument(skip_all)]
pub fn dump_rainbow_table(path: &Path) -> Result<(), RainbowError> {
    info!("Dumping rainbow table from: {}", path.display());
    let (algorithm, pw_len, num_links, charset_size, offset, _) = parse_rainbow(path)?;

    println!("Hashassin Rainbow Table");
    println!("VERSION: {}", VERSION);
    println!("ALGORITHM: {}", algorithm);
    println!("PASSWORD LENGTH: {}", pw_len);
    println!("CHAR SET SIZE: {}", charset_size);
    println!("NUM LINKS: {}", num_links);
    println!("ASCII OFFSET: {}", offset);

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let header_size = MAGIC_WORD.len() + 1 + 1 + algorithm.to_string().len() + 1 + 16 + 16 + 16;
    let chain_size = 2 * pw_len;

    for chunk in buffer[header_size..].chunks(chain_size) {
        if chunk.len() != chain_size {
            warn!("Invalid chain data length: {}", chunk.len());
            break;
        }
        let (start, end) = chunk.split_at(pw_len);
        println!(
            "{}\t{}",
            String::from_utf8_lossy(start),
            String::from_utf8_lossy(end)
        );
    }

    info!("Successfully dumped rainbow table");
    Ok(())
}

#[instrument(skip_all)]
pub fn crack(
    rainbow_path: &Path,
    hash_path: &Path,
    output_path: Option<&Path>,
    threads: usize,
) -> Result<(), RainbowError> {
    info!(
        "Cracking hashes from {} using rainbow table {}",
        hash_path.display(),
        rainbow_path.display()
    );

    let (rainbow_algorithm, pw_len, num_links, charset_size, offset, end_map) =
        parse_rainbow(rainbow_path)?;
    let (hash_algorithm, hashes) = parse_hash_file(hash_path)?;

    if rainbow_algorithm != hash_algorithm {
        return Err(RainbowError::AlgorithmMismatch);
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?;

    let results: Vec<_> = pool.install(|| {
        hashes
            .par_iter()
            .filter_map(|target_hash| {
                (0..num_links).find_map(|i| {
                    let mut current_hash = target_hash.clone();
                    for _ in 0..i {
                        let password = match reduce(&current_hash, pw_len, charset_size, offset) {
                            Ok(p) => p,
                            Err(_) => return None,
                        };

                        current_hash = match compute_hash_rainbow(&password, rainbow_algorithm) {
                            Ok(h) => h,
                            Err(_) => return None,
                        };
                    }

                    let possible_end = match reduce(&current_hash, pw_len, charset_size, offset) {
                        Ok(p) => p,
                        Err(_) => return None,
                    };

                    end_map.get(possible_end.as_bytes()).and_then(|start| {
                        match regenerate_chain(
                            start,
                            target_hash,
                            num_links - i,
                            rainbow_algorithm,
                            pw_len,
                            charset_size,
                            offset,
                        ) {
                            Ok(Some(res)) => Some(res),
                            _ => None,
                        }
                    })
                })
            })
            .collect()
    });

    if results.is_empty() {
        return Err(RainbowError::NoPasswordsFound);
    }

    let mut writer: Box<dyn Write> = match output_path {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    for (hash, password) in &results {
        writeln!(writer, "{}\t{}", hex::encode(hash), password)?;
    }

    Ok(())
}

fn parse_rainbow(path: &Path) -> Result<RainbowTableData, RainbowError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut cursor = 0;

    if !buffer.starts_with(MAGIC_WORD) {
        return Err(RainbowError::InvalidMagic);
    }
    cursor += MAGIC_WORD.len();

    let version = buffer[cursor];
    cursor += 1;
    if version != VERSION {
        return Err(RainbowError::InvalidHeader);
    }

    let algorithm_len = buffer[cursor] as usize;
    cursor += 1;

    let algorithm_str = std::str::from_utf8(&buffer[cursor..cursor + algorithm_len])
        .map_err(|_| RainbowError::InvalidAlgorithm)?;
    cursor += algorithm_len;

    let password_length = buffer[cursor] as usize;
    cursor += 1;

    let charset_size = u128::from_be_bytes(
        buffer[cursor..cursor + 16]
            .try_into()
            .map_err(|_| RainbowError::InvalidHeader)?,
    );
    cursor += 16;

    let num_links = u128::from_be_bytes(
        buffer[cursor..cursor + 16]
            .try_into()
            .map_err(|_| RainbowError::InvalidHeader)?,
    ) as usize;
    cursor += 16;

    let offset = u128::from_be_bytes(
        buffer[cursor..cursor + 16]
            .try_into()
            .map_err(|_| RainbowError::InvalidHeader)?,
    );
    cursor += 16;

    let mut end_map = HashMap::new();
    let chain_size = 2 * password_length;
    for chunk in buffer[cursor..].chunks(chain_size) {
        if chunk.len() != chain_size {
            break;
        }
        let (start, end) = chunk.split_at(password_length);
        end_map.insert(end.to_vec(), start.to_vec());
    }

    let algorithm = Algorithm::from_str(algorithm_str).ok_or(RainbowError::InvalidAlgorithm)?;

    Ok((
        algorithm,
        password_length,
        num_links,
        charset_size,
        offset,
        end_map,
    ))
}

fn regenerate_chain(
    start: &[u8],
    target_hash: &[u8],
    steps: usize,
    algorithm: Algorithm,
    password_length: usize,
    charset_size: u128,
    offset: u128,
) -> Result<Option<(Vec<u8>, String)>, RainbowError> {
    let mut current =
        String::from_utf8(start.to_vec()).map_err(|e| RainbowError::UnicodeError(e.to_string()))?;

    for _ in 0..steps {
        let hash = compute_hash_rainbow(&current, algorithm)?;
        if hash == target_hash {
            return Ok(Some((target_hash.to_vec(), current)));
        }
        current = reduce(&hash, password_length, charset_size, offset)?;
    }

    Ok(None)
}

fn parse_hash_file(path: &Path) -> Result<(Algorithm, Vec<Vec<u8>>), RainbowError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if buffer.len() < 3 {
        return Err(RainbowError::InvalidHeader);
    }

    let version = buffer[0];
    if version != 1 {
        return Err(RainbowError::InvalidHeader);
    }

    let algorithm_len = buffer[1] as usize;
    let algorithm_str = std::str::from_utf8(&buffer[2..2 + algorithm_len])
        .map_err(|_| RainbowError::InvalidAlgorithm)?;

    let algorithm = Algorithm::from_str(algorithm_str).ok_or(RainbowError::InvalidAlgorithm)?;

    let hash_size = algorithm.hash_size();
    let data_start = 2 + algorithm_len + 1; // +1 for password length byte

    let hashes: Result<Vec<_>, _> = buffer[data_start..]
        .chunks(hash_size)
        .map(|chunk| {
            if chunk.len() != hash_size {
                Err(RainbowError::InvalidHeader)
            } else {
                Ok(chunk.to_vec())
            }
        })
        .collect();

    Ok((algorithm, hashes?))
}

// Helper trait for Algorithm
trait AlgorithmExt {
    fn hash_size(&self) -> usize;
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
}

impl AlgorithmExt for Algorithm {
    fn hash_size(&self) -> usize {
        match self {
            Algorithm::Md5 => 16,
            Algorithm::Sha256 => 32,
            Algorithm::Sha3_512 => 64,
            Algorithm::Scrypt => 91,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "md5" => Some(Self::Md5),
            "sha256" => Some(Self::Sha256),
            "sha3-512" => Some(Self::Sha3_512),
            "scrypt" => Some(Self::Scrypt),
            _ => None,
        }
    }
}
