# EMV Parser

This Rust program gets emv messagesv in hex string format and it extracts and processes specific fields from an input message.

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/HosseinAssaran/EMV-Parser
    ```

2. Build and run the program:

    ```bash
    cargo build
    cargo run
    ```

3. Follow the prompts to enter an EMV message for parsing.

## Testing

To run tests, use the following command:

```bash
cargo test
```

## Usage As a Library
1. Add the emv_parser package to your project using Cargo:

```bash
cargo add emv_parser
```

2. Import the necessary modules where you want to use the library functions:

```bash
use emv_parser::{StringManipulation, positions_of_set_bits, LTV};

```