# Nand2Tetris Assembler in Rust

A Nand2Tetris Assembler written in Rust! This assembler is designed to translate assembly language from the Nand2Tetris course (`.asm` files) into binary code (`.hack` files), allowing you to run your Hack assembly programs on the Hack hardware platform.

## Getting Started

### Prerequisites

Before you can use this assembler, you need to have Rust installed on your machine. If you don't have Rust installed, you can download it from [the Rust website](https://www.rust-lang.org/tools/install).

### Building the Assembler

To build the assembler, you will use Cargo, Rust's package manager and build system. Simply follow these steps:

1. Clone this repository to your local machine using Git:
   ```sh
   git clone https://github.com/yourusername/nand2tetris-assembler-rust.git
   ```
   Replace `yourusername` with your actual GitHub username.

2. Navigate into the project directory:
   ```sh
   cd nand2tetris-assembler-rust
   ```

3. Build the project using Cargo:
   ```sh
   cargo build --release
   ```
   The `--release` flag builds the assembler in release mode, which optimizes the binary for performance.

### Running the Assembler

After building the assembler, you can use it to convert `.asm` files to `.hack` binary files.

To run the assembler, use the following command:

```sh
./rust-assembler <filepath>
```

Replace `<filepath>` with the path to your `.asm` file. The assembler will generate a `.hack` file in the same directory as your `.asm` file.

## Examples

Assuming you have an assembly file named `Max.asm` in your current directory, you can assemble it into a binary file named `Max.hack` with the following command:

```sh
./rust-assembler Max.asm
```

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- This project is inspired by the Nand2Tetris course, and it's dedicated to everyone involved in creating and maintaining this resource.