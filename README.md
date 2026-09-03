# QRCode generator

This is my own QR Code generator implementation based on the ISO specifications.

The project is primarily an exploration of how QR Codes work internally. It
implements the data encoding, padding, Reed–Solomon error correction, masking,
format information, and matrix assembly without third-party dependencies.

## Running the application

The command-line binary currently generates a QR Code for the example message
`HELLO WORLD` with medium error correction:

```bash
cargo run
```

Run the command from the project root. The application prints intermediate
encoding/debug information as well as the assembled 21 × 21 QR matrix.

The binary is currently an example program rather than a full command-line
interface. To generate another message or choose another correction level,
change the values in `src/main.rs`.

## Current features and roadmap

- Version 1 QR Code matrices (21 × 21 modules).
- Alphanumeric encoding using the QR Code character set.
- All four error-correction levels: L, M, Q, and H.
- Data and error-correction codeword generation.
- Mask pattern application.
- Format information generation and placement.
- Terminal rendering of the QR Code matrix.
- Unit and integration tests for encoding, correction, and complete matrices.

- [ ] Command-line arguments for the message and correction level
- [ ] Numeric, byte, and other QR Code encoding modes
- [ ] QR Code versions greater than 1
- [ ] Automatic mask selection based on the QR Code penalty rules
- [ ] Image/SVG export
- [ ] Cleaner library and command-line APIs

## Using the library

The crate exposes the QR Code implementation as a library. A basic QR Code can
be assembled and rendered as text with:

```rust
use qrcodegen::correction::Correction;
use qrcodegen::encoding::Encoding;
use qrcodegen::qrcode::QRCode;

let mut qr_code = QRCode::new(
    "HELLO WORLD".to_string(),
    Encoding::ALPHANUMERIC,
    Correction::M,
);
qr_code.assemble();

println!("{}", qr_code.as_string());
```

At the moment, only version 1 alphanumeric QR Codes are supported. Messages
must fit the selected version and correction level, and must use characters
from the QR Code alphanumeric set:

```text
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:
```

## Project structure

```text
qrcode-generator/
├── src/
│   ├── main.rs                 Example binary entrypoint
│   ├── lib.rs                  Public library modules
│   ├── qrcode.rs               QR Code assembly and version handling
│   ├── encoding/
│   │   ├── mod.rs              Encoding abstraction and bitstream creation
│   │   └── alphanumeric.rs     Alphanumeric mode implementation
│   ├── correction.rs           Reed–Solomon and format information handling
│   ├── masking.rs              QR mask patterns
│   ├── patterns.rs             Finder, separator, timing, and dark modules
│   └── utils.rs                Matrix, binary, and padding utilities
├── tests/
│   └── integration_tests.rs    Complete version 1 QR Code tests
├── log-antilog.csv             GF(256) logarithm/antilogarithm table
└── Cargo.toml                  Rust package manifest
```

## Technical choices

### Rust with no third-party dependencies

The implementation uses the Rust standard library only. This keeps the QR
Code algorithms visible and makes the project easy to build and test with
Cargo.

### QR Code data pipeline

The generator follows the main QR Code construction stages:

```text
message
  → mode and character-count indicators
  → alphanumeric bitstream
  → terminator and padding
  → data codewords and error-correction codewords
  → QR mask and format information
  → terminal matrix
```

### Finite-field arithmetic

Reed–Solomon operations use a lookup table stored in `log-antilog.csv`. The
current implementation uses the table to calculate generator polynomials and
error-correction codewords for version 1 QR Codes.

## Testing

Run all unit and integration tests with:

```bash
cargo test
```

The integration tests compare generated version 1 matrices for the L, M, Q,
and H correction levels using known `HELLO WORLD` examples.
