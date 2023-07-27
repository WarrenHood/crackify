# crackify
A simple PDF password cracker written in Rust.

It uses multithreading. I haven't done this in the most efficient way though...
It supports using a word list or it can generate numeric passwords given a min and max password length.

Usage
```bash
Usage: crackify [OPTIONS] --pdf <PDF>

Options:
  -p, --pdf <PDF>
          Path to a password protected PDF file
  -w, --wordlist <WORDLIST>
          Path to a wordlist. Each word should be on a new line
  -i, --is-numeric
          Whether or not the password is entirely numeric
  -s, --smallest-numeric-length <SMALLEST_NUMERIC_LENGTH>
          Minimum length of numeric password [default: 1]
  -l, --largest-numeric-length <LARGEST_NUMERIC_LENGTH>
          Maximum length of numeric password [default: 8]
  -t, --threads <THREADS>
          Number of threads to use [default: 16]
  -h, --help
          Print help
  -V, --version
          Print version
```
