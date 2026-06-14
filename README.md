# 📄 Read-CSV

A simple Rust project that reads and displays records from a CSV file using the `csv` crate.

## Features

* Read CSV files
* Parse records efficiently
* Handle errors gracefully
* Built with Rust and the `csv` dependency

## Usage

```bash
cargo run
```

## Example Output

```text
StringRecord(["1", "John", "john@example.com"])
StringRecord(["2", "Alice", "alice@example.com"])
```

## Dependencies

* `csv`

## Project Structure

```
.
├── src/
│   └── main.rs
├── customers.csv
├── Cargo.toml
└── Cargo.lock
```
