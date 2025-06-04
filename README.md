# 🔐 Hashassin - Rainbow Table Generator and Cracker

**Hashassin** is a secure, high-performance Rust-based application for generating and using rainbow tables to crack password hashes. This project supports multithreaded generation, advanced hashing algorithms, and a binary-compliant file format.

---

## 📌 Table of Contents
- [Description](#-project-description)
- [Features](#-features)
- [Hashing Algorithms](#-supported-hashing-algorithms)
- [Build & Format Instructions](#-build--format-instructions)
- [How to Run](#-how-to-run)
- [Performance](#-performance)
- [Credits](#-credits)

---

## 📄 Project Description

This project extends functionality from Project 1 by introducing a fully-featured rainbow table pipeline, including:
- Rainbow table generation using custom seed passwords.
- Binary file output with a strict spec-compliant format.
- Dumping rainbow table contents to human-readable form.
- Cracking hashed passwords using rainbow tables via parallel search.

---

## 🚀 Features

- 📂 Rainbow Table Generator (`gen-rainbow-table`)
- 📤 Table Dumper (`dump-rainbow-table`)
- 🔓 Password Cracker (`crack`)
- 🧵 Multithreaded execution using `rayon`
- 🔍 Full support for ASCII and Unicode charset generation
- 🔒 Hashing with `md5`, `sha256`, `sha3-512`, and `scrypt`
- 🧪 Error-safe, fully documented, and formatted Rust code

---

## 🔐 Supported Hashing Algorithms
- `md5` (default)
- `sha256`
- `sha3-512`
- `scrypt`

---

## 🧱 Build & Format Instructions

```bash
# Clone the project
git clone <your_repo_url>
cd hashassin

# Navigate to cli folder
cd cli

# Build binary
cargo build --release

# Format code
cargo fmt --all

# Run lint checks (should produce no warnings)
cargo clippy

# Build docs
cargo doc --document-private-items --no-deps --open
```

---

## 🧪 How to Run

### 1️⃣ Generate Rainbow Table

```bash
cargo run --release -- gen-rainbow-table \
  --in-file passwords.txt \
  --out-file rainbow.bin \
  --algorithm sha256 \
  --threads 4 \
  --num-links 5
```

### 2️⃣ Dump Rainbow Table

```bash
cargo run --release -- dump-rainbow-table \
  --in-file rainbow.bin
```

### 3️⃣ Crack Hashes

```bash
cargo run --release -- crack \
  --hashes hashes.bin \
  --in-file rainbow.bin \
  --threads 4 \
  --out-file cracked.txt
```

> 💡 All CLI options include sensible defaults. `md5` and 1 thread are used if not specified.

---


## 📈 Performance

For performance metrics (hashing times, thread scaling, etc.), refer to [`PERFORMANCE.md`](./PERFORMANCE.md). Highlights include:
- Thread scalability benchmarks
- Algorithm speed comparisons
- Unicode charset performance impact

---

## 👥 Credits

See [`CREDITS.md`](./CREDITS.md) for group member contributions and work breakdown.

---

## ✅ Academic Honesty

This project abides by the course’s academic integrity policy. See [`HONESTY.md`](./HONESTY.md) for details.

---