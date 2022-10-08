# transpose-csv

Simple rust utility to transpose csv-files (columns separated by `,`, rows separated by `\n`, should work for unicode files). Reads from stdin and writes to stdout.

## Install

```
cargo build -r transpose-csv
```

## Usage

```
cat input.csv | ./transpose-csv > output.csv
```
