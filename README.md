# Rusty Ripper CLI

`Rusty Ripper` is a command-line interface (CLI) application written in Rust designed to handle dictionary attacks using hash functions and password files. The tool allows users to load dictionaries, apply hashing algorithms, and validate hashed password-login pairs from a CSV file.

## Features

- **Dictionary Loading**: Load and display a dictionary from a file.
- **Hash Function Application**: Apply different hash functions (e.g., `md5`, `sha256`, `sha512`) to tokens from the dictionary.
- **Password Verification**: Load login-password pairs from a CSV file and verify them against hashed tokens.
- **Combination Mode**: Combine dictionary, hash, and password operations to check for valid login-password pairs.

## Installation

To get started, you need to have Rust installed. If Rust is not installed, you can do so by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and build the project:

```bash
git clone https://github.com/LeweLotki/rusty-ripper.git
cd rusty-ripper
cargo build --release
```

## Usage

The CLI application provides several flags to run various operations. Below is a detailed guide on how to use each flag and combination.

### Commands

1. **Load and Display a Dictionary**
   ```bash
   cargo run -- -d <dictionary_file>
   ```
   - `-d`, `--dictionary`: Specify the path to a dictionary file.
   - This command loads the dictionary file and displays information about its contents.

2. **Apply a Hash Function to the Dictionary Tokens**
   ```bash
   cargo run -- --hash <hash_function>
   ```
   - `--hash`: Specify the hash function (`md5`, `sha256`, `sha512`).
   - This command applies the selected hash function to the dictionary tokens and displays a summary of the hashes.

3. **Load and Display Password-Login Pairs from a CSV File**
   ```bash
   cargo run -- -p <passwords_file>
   ```
   - `-p`, `--passwords`: Specify the path to the CSV file containing login-password pairs.
   - This command loads the CSV file and displays information about the login-password pairs.

4. **Run in Full Combination Mode**
   ```bash
   cargo run -- -d <dictionary_file> --hash <hash_function> -p <passwords_file>
   ```
   - Combines the dictionary, hash, and password functionalities.
   - This command checks if the hashed tokens from the dictionary match any hashed passwords from the CSV file and displays the associated login and token.

### Error Handling

- If an incorrect combination of flags is used, the program displays an error message and shows the help message.
- Only valid flag combinations are accepted:
  - `--dictionary` alone
  - `--hash` alone
  - `--passwords` alone
  - All three flags together (`--dictionary`, `--hash`, `--passwords`)

## Example

### Example 1: Loading a Dictionary

```bash
cargo run -- -d dictionary/dictionary.txt
```
This will load the specified dictionary file and display information such as the number of words in the dictionary.

### Example 2: Applying a Hash Function

```bash
cargo run -- --hash sha256
```
This will apply the `sha256` hash function to the dictionary tokens (a dummy dictionary is used if no dictionary is provided).

### Example 3: Loading Passwords

```bash
cargo run -- -p passwords/passwords.csv
```
This will load the specified CSV file and display the number of login-password pairs.

### Example 4: Full Combination Mode

```bash
cargo run -- -d dictionary/dictionary.txt --hash sha256 -p passwords/passwords.csv
```
This will load the dictionary, apply the `sha256` hash function, and verify the login-password pairs.

## Dependencies

The project uses the following dependencies:
- **[clap](https://crates.io/crates/clap)**: For parsing command-line arguments.
- **[sha2](https://crates.io/crates/sha2)**: For hashing with SHA-256 and SHA-512.
- **[md-5](https://crates.io/crates/md-5)**: For MD5 hashing.
- **[csv](https://crates.io/crates/csv)**: For reading CSV files.
- **[hex](https://crates.io/crates/hex)**: For encoding hash results in hexadecimal format.

## Contributing

Feel free to open issues or submit pull requests if you want to contribute or encounter any bugs.
