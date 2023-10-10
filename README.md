# hash-brute-force
This project is a Rust-based utility for generating hashes for numbers and filtering them based on the number of trailing zeros and the number of first hashes found. It provides an efficient way to generate and filter hashes, making it useful for various applications that require hash generation and filtering.

## Installation

To use this utility, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from the official Rust website: https://www.rust-lang.org/

Once Rust is installed, you can clone this repository and build the project using the following commands:
```bash
git clone https://github.com/9i5BCrUcNX5NmT/hash-brute-force.git
cd hash-brute-force
cargo build --release
```
## Usage

After building the project, you can run the utility using the following command:
```bash
cargo run --release -- <arguments>
```
Replace <arguments> with the desired command-line arguments to specify the range of numbers to generate hashes for, the number of trailing zeros to filter, and the number of first hashes to find.

For example, to generate hashes and filter them for at least 3 trailing zeros and find the first 1000 hashes, you can run the following command:

```bash
cargo run --release -- --null 3 --find 1000
```

## Examples

Here are some examples of using the utility:

Generate hashes for numbers at least 3 trailing zeros and find the first 6 hashes:
```bash
$ cargo run --release -- -n 3 -f 6
4163, "95d4362bd3cd4315d0bbe38dfa5d7fb8f0aed5f1a31d98d510907279194e3000"
11848, "cb58074fd7620cd0ff471922fd9df8812f29f302904b15e389fc14570a66f000"
12843, "bb90ff93a3ee9e93c123ebfcd2ca1894e8994fef147ad81f7989eccf83f64000"
13467, "42254207576dd1cfb7d0e4ceb1afded40b5a46c501e738159d8ac10b36039000"
20215, "1f463eb31d6fa7f3a7b37a80f9808814fc05bf10f01a3f653bf369d7603c8000"
28892, "dab12874ecae90c0f05d7d87ed09921b051a586c7321850f6bb5e110bc6e2000"
```
Generate hashes for numbers at least 3 trailing zeros and find the first 6 hashes:
```bash
$ cargo run --release -- -n 5 -f 3
828028, "d95f19b5269418c0d4479fa61b8e7696aa8df197082b431a65ff37595c100000"
2513638, "862d4525b0b60779d257be2b3920b90e3dbcd60825b86cfc6cffa49a63c00000"
3063274, "277430daee71c67b356dbb81bb0a39b6d53efd19d14177a173f2e05358a00000"
```

## GIF

Check out this GIF to see the utility in action:

GIF



## License

This project is licensed under the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html#license-text)