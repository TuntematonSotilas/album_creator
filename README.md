# album_creator

Web app to create albums

## Framework

Seed [Website](https://seed-rs.org)

## Installation

### Install Rustup + Cargo
https://www.rust-lang.org/learn/get-started

### Install Cargo Make

	cargo install cargo-make

# Configure

Create a `config/config.rs` file with :

```Rust
pub const LOGIN: &str = "your_login";
pub const PWD: &str = "your_password";
```

## Run

Open a new terminal and run :

	cargo make serve

Open a second terminal and run :

	cargo make watch