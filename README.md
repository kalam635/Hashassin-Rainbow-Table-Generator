# Project-1 CS 551 Group 17

## 📌 Project Overview
This repository contains the implementation for **Project-1** as part of **CS 551** coursework. 

This project generates random passwords concurrently using multiple threads, allowing efficient password creation in bulk. It also reads, validates, and processes passwords from files, ensuring proper format and handling errors such as invalid lengths or counts.



## 🚀 Setup & Installation
To clone and set up the project locally, run:
```sh
git clone https://github.com/2025-Spring-CS-551/project-1-cs-551-group-17.git
cd project-1-cs-551-group-17

🤝 Contributors
Kalam Shaik


##💻 Features
Password Generation: Generates random passwords based on user-defined length and count.

Hash Generation: Supports multiple hashing algorithms (MD5, SHA256, SHA3-512, Scrypt).

Dumping Hashes : This will print the output in the required format to std out 

Multithreading: Utilizes Rayon for parallel processing to improve performance when handling large datasets.

File Handling: Allows for reading passwords from files and outputting results to files.

Error Handling: Comprehensive error handling with custom error types for various failure scenarios.





## 🛠️ Code Functions
generate_passwords: Generates a specified number of random passwords of a given length using multiple threads for concurrency, and writes the passwords to a file or outputs them to the console.

generate_hashes: (Assumed function) Would take the generated passwords and compute their cryptographic hashes (e.g., using SHA-256 or bcrypt) for storage or verification purposes.

dump_hashes: (Assumed function) Would write the computed hashes to a specified file or console, similar to how the passwords are dumped in generate_passwords.

## The project also uses the following libraries:

base64: For encoding and decoding data in base64 format, typically used for handling hashes or encoded passwords.

digest: A trait and common functionality for cryptographic hash functions, enabling the use of different hashing algorithms.

md5: A library for computing MD5 hashes, used for generating a hash of the password.

scrypt: For the scrypt password hashing algorithm, a key derivation function designed to make brute-force attacks more difficult.

sha2: Provides the SHA-256 hashing algorithm, widely used for cryptographic purposes.

sha3: Implements the SHA-3 family of hash functions, including SHA3-512, for secure cryptographic hashing.

These libraries are used to enhance the cryptographic capabilities of the project, including password hashing and encoding.

## Additional Files
passwords.txt = this file consists of 5 sample passwords each of length 8

hashes.bin = this file consists of hashed sha256 binary output 



## 🛠️ How to Run
cargo fmt
cargo build
cargo clippy 
cargo check

cargo run --bin hashassin gen-passwords --chars 8 --num 5 --threads 4 --out-file passwords.txt

cargo run --bin hashassin gen-hashes --in-file passwords.txt --out-file hashes.bin --threads 4 --algorithm sha256

cargo run --bin hashassin dump-hashes --in-file hashes.bin


## 🔐 Supported Hashing Algorithms
MD5 (128-bit)

SHA256 (256-bit)

SHA3-512 (512-bit)

Scrypt (custom parameters)

## 🛡️ Error Handling
The tool uses comprehensive error handling and provides descriptive error messages for various failure scenarios. Errors are categorized into:

Password Errors

Hash Errors

File I/O Errors

Thread Pool Errors

