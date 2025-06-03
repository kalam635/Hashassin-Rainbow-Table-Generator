use crate::hashing::HashError;
use std::fs::File;
use std::io::Read;
use std::str;

/// Dumps hashes from a file and prints the details of the algorithm, version, and password length.
/// It processes different hash algorithms and prints their corresponding hash or string values.
///
/// # Arguments
/// * `path` - The path to the file containing the hashed data.
///
/// # Returns
/// A `Result<(), HashError>`:
/// * `Ok(())` if the hashes were successfully processed and printed.
/// * `Err(HashError)` if there was an issue with reading the file, invalid data, or unsupported algorithm.
///
/// # Errors
/// This function will return an error if:
/// * The file format is invalid (e.g., missing or incorrect header).
/// * The algorithm is unknown or unsupported.
/// * There is an issue while converting hash data to string (e.g., invalid UTF-8).
///
/// # Example
/// ```
/// dump_hashes("path/to/file")?;
/// ```
pub fn dump_hashes(path: &std::path::Path) -> Result<(), HashError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse header
    if buffer.len() < 3 {
        return Err(HashError::InvalidAlgorithm);
    }

    let version = buffer[0];
    let algorithm_len = buffer[1] as usize;
    let algorithm_start = 2;
    let algorithm_end = algorithm_start + algorithm_len;

    if algorithm_end >= buffer.len() {
        return Err(HashError::InvalidAlgorithm);
    }

    let algorithm_str = str::from_utf8(&buffer[algorithm_start..algorithm_end])
        .map_err(|_| HashError::InvalidAlgorithm)?;

    let password_len = buffer[algorithm_end];
    let data_start = algorithm_end + 1;

    // Determine hash size based on the algorithm
    let hash_size = match algorithm_str {
        "md5" => 16,
        "sha256" => 32,
        "sha3-512" => 64,
        "scrypt" => 91,
        _ => return Err(HashError::InvalidAlgorithm),
    };

    // Process hashes
    let data = &buffer[data_start..];
    if data.len() % hash_size != 0 {
        return Err(HashError::InvalidAlgorithm);
    }

    println!("VERSION: {}", version);
    println!("ALGORITHM: {}", algorithm_str);
    println!("PASSWORD LENGTH: {}", password_len);

    // Print each hash or string
    for chunk in data.chunks(hash_size) {
        match algorithm_str {
            "md5" | "sha256" | "sha3-512" => {
                println!("{}", hex::encode(chunk));
            }
            "scrypt" => {
                let s = str::from_utf8(chunk).map_err(|_| HashError::InvalidAlgorithm)?;
                println!("{}", s);
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
