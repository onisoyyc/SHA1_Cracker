# SHA-1 Hash Cracker

A Rust-based tool to crack SHA-1 hashes using a dictionary (wordlist) attack. This program takes a SHA-1 hash and a wordlist file path as input, then iterates through the wordlist to find a matching hash.

## Features

- **Dictionary Attack**: Uses a wordlist to crack SHA-1 hashes.
- **Command-Line Interface**: Easy-to-use CLI for specifying hash and wordlist path.
- **Error Handling**: Provides clear error messages for incorrect input formats.

## Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.56 or later)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/sha1-cracker.git
   cd sha1-cracker
   ```

2. Add dependencies in `Cargo.toml`:
   ```toml
   [dependencies]
   sha1 = "0.10"
   hex = "0.4"
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

To run the SHA-1 hash cracker:

```bash
cargo run -- <path/to/wordlist.txt> <sha1_hash>
```

### Example

```bash
cargo run -- rockyou.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
```

- `rockyou.txt` is the wordlist file containing potential passwords.
- `7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53` is the SHA-1 hash to crack.

### Output

- If a matching hash is found:
  ```
  Password found: password123
  ```
- If no match is found:
  ```
  Password not found
  ```

## Code Structure

- **Command-Line Arguments**: Reads the file path and target hash.
- **Hash Validation**: Checks if the SHA-1 hash is in the correct 40-character format.
- **Wordlist Processing**: Reads each line in the wordlist, hashes it, and compares it to the target.



## Error Handling

- **Invalid Arguments**: Prompts the correct usage if arguments are missing or incorrect.
- **Invalid SHA-1 Hash Length**: Checks if the SHA-1 hash is the correct length (40 characters).
