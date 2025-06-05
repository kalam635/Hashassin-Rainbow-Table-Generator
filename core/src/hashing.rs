use crate::password;
use base64::Engine;
use digest::Digest;
use md5::Md5;
use rand::Rng;
use rayon::prelude::*;
use scrypt::{scrypt, Params};
use sha2::Sha256;
use sha3::Sha3_512;
use std::fs::File;
use std::io::{self, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Password error: {0}")]
    Password(#[from] password::PasswordError),
    #[error("Thread pool error: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
    #[error("Invalid algorithm")]
    InvalidAlgorithm,
    #[error("Scrypt error: {0}")]
    Scrypt(String),
}

#[derive(Clone, Copy, Debug, clap::ValueEnum, PartialEq)]
#[non_exhaustive]
pub enum Algorithm {
    Md5,
    Sha256,
    Sha3_512,
    Scrypt,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Md5 => write!(f, "md5"),
            Algorithm::Sha256 => write!(f, "sha256"),
            Algorithm::Sha3_512 => write!(f, "sha3-512"),
            Algorithm::Scrypt => write!(f, "scrypt"),
        }
    }
}

pub fn generate_hashes(
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    algorithm: Algorithm,
    threads: usize,
) -> Result<(), HashError> {
    let passwords = password::read_passwords(input_path)?;
    if passwords.is_empty() {
        return Err(HashError::InvalidAlgorithm);
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?;

    let hashes = pool.install(|| {
        passwords
            .par_iter()
            .map(|pwd| compute_hash(pwd, algorithm))
            .collect::<Result<Vec<_>, _>>()
    })?;

    let algorithm_str = algorithm.to_string();
    let hash_size = match algorithm {
        Algorithm::Md5 => 16,
        Algorithm::Sha256 => 32,
        Algorithm::Sha3_512 => 64,
        Algorithm::Scrypt => 91,
    };

    let mut file = File::create(output_path)?;
    file.write_all(&[1])?;
    file.write_all(&[algorithm_str.len() as u8])?;
    file.write_all(algorithm_str.as_bytes())?;
    file.write_all(&[passwords[0].len() as u8])?;

    for hash in hashes {
        if hash.len() != hash_size {
            return Err(HashError::InvalidAlgorithm);
        }
        file.write_all(&hash)?;
    }

    Ok(())
}

pub fn compute_hash(password: &str, algorithm: Algorithm) -> Result<Vec<u8>, HashError> {
    Ok(match algorithm {
        Algorithm::Md5 => {
            let mut hasher = Md5::new();
            hasher.update(password.as_bytes());
            hasher.finalize().to_vec()
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            hasher.finalize().to_vec()
        }
        Algorithm::Sha3_512 => {
            let mut hasher = Sha3_512::new();
            hasher.update(password.as_bytes());
            hasher.finalize().to_vec()
        }
        Algorithm::Scrypt => {
            let mut rng = rand::thread_rng();
            let salt: [u8; 16] = rng.gen();
            let params = Params::new(14, 8, 1, 32)
                .map_err(|e| HashError::Scrypt(format!("Invalid Scrypt parameters: {e:?}")))?;
            let mut output = [0u8; 32];
            scrypt(password.as_bytes(), &salt, &params, &mut output)
                .map_err(|e| HashError::Scrypt(format!("Scrypt computation failed: {e:?}")))?;
            format!(
                "$scrypt$ln=14,r=8,p=1${}${}",
                base64::engine::general_purpose::STANDARD.encode(salt),
                base64::engine::general_purpose::STANDARD.encode(output)
            )
            .into_bytes()
        }
    })
}
