# ISO8583 parser 

This Rust program gets iso messages in hex string format and it extracts and processes specific fields from an input message.

**Important Note:** As the PHP Web server use a rust program to parse the message, you will need it. You can achieve this program by compiling the rust written program from source or you can downlaod the executable file with **iso_parser_downloader**.
## Usage
### Run it as a PHP Web Server
1. Download the source code and go to the root directory of your source code
2. Run below command inside **PowerShell**:
   ```
    .\iso_parser_downloader.bat
   ```
3. Run PHP Web Server using below command:
   ```
   php -S localhost:12345
   ```
4. Open your browser and go to the link below:
   ```
   localhost:12345
   ```

### Compile and Utilize as a Command Line Interface (CLI).

1. Clone the repository:

    ```bash
    git clone https://github.com/HosseinAssaran/ISO8583-Parser
    cd ISO8583-Parser
    ```

2. Build and run the program:

    ```bash
    cargo build
    cargo run
    ```

3. Follow the prompts to enter an iso message for parsing.

### Compile and Integrate the Rust Parser CLI with a PHP Web Server.

1. Clone the repository:

    ```bash
    git clone https://github.com/HosseinAssaran/ISO8583-Parser
    cd ISO8583-Parser
    ```

2. Build relaese:
   
    ```bash
    cargo build --release
    ````

3. Run PHP Server:
   
    ```bash
    php -S localhost:12345
    ```` 

4. Open you browser and go to `localhost:12345`

## Testing

To run tests, use the following command:

```bash
cargo test
```

## Usage As a Library
1. Add the iso8583_parser package to your project using Cargo:

```bash
cargo add iso8583_parser
```

2. Import the necessary modules where you want to use the library functions:

```bash
use iso8583_parser::{StringManipulation, positions_of_set_bits, LTV};

```
