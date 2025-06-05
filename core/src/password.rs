use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid password length at line {0}")]
    InvalidLength(usize),
    #[error("Thread pool error: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
    #[error("Invalid password count")]
    InvalidCount,
}

/// Generates a specified number of passwords of a given length using multithreading.
///
/// # Arguments
/// * `count` - The number of passwords to generate.
/// * `length` - The length of each generated password.
/// * `threads` - The number of threads to use for parallel processing.
/// * `output` - Optional path to save the generated passwords. If None, prints to standard output.
///
/// # Errors
/// Returns an error if:
/// * `count` is zero.
/// * `length` is zero.
/// * There is a failure in creating a thread pool.
/// * There is a failure in file creation or writing.
///
/// # Example
/// ```
/// generate_passwords(10, 12, 4, Some("passwords.txt".into()))?;
/// ```
pub fn generate_passwords(
    count: usize,
    length: usize,
    threads: usize,
    output: Option<std::path::PathBuf>,
) -> Result<(), PasswordError> {
    if count == 0 {
        return Err(PasswordError::InvalidCount);
    }
    if length == 0 {
        return Err(PasswordError::InvalidLength(0));
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()?;

    let passwords = pool.install(|| {
        (0..count)
            .into_par_iter()
            .map(|_| generate_password(length))
            .collect::<Vec<_>>()
    });

    let mut writer: Box<dyn Write> = match output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    for password in passwords {
        writeln!(writer, "{}", password)?;
    }

    Ok(())
}

/// Generates a random password of the specified length using printable ASCII characters (32–126).
///
/// # Arguments
/// * `length` - The length of the generated password.
///
/// # Returns
/// A `String` representing the randomly generated password.
///
/// # Example
/// ```
/// let password = generate_password(12);
/// ```
fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(32..=126) as u8 as char)
        .collect()
}

/// Reads passwords from a file and ensures all passwords have the same length.
///
/// # Arguments
/// * `path` - The path to the file containing passwords.
///
/// # Returns
/// A `Result<Vec<String>, PasswordError>` containing:
/// * `Ok(Vec<String>)` with the list of passwords if reading is successful.
/// * `Err(PasswordError::InvalidLength)` if passwords have inconsistent lengths.
///
/// # Errors
/// Returns an error if:
/// * There's an I/O error when reading the file.
/// * Passwords in the file have varying lengths.
///
/// # Example
/// ```
/// let passwords = read_passwords("passwords.txt")?;
/// ```
pub fn read_passwords(path: &std::path::Path) -> Result<Vec<String>, PasswordError> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut passwords = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            passwords.push(line.clone());
        } else if line.len() != passwords[0].len() {
            return Err(PasswordError::InvalidLength(i + 1));
        } else {
            passwords.push(line);
        }
    }

    Ok(passwords)
}
