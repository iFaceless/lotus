lotus
===================

Convert html table to markdown table. 

# Usage

```shell
cargo run -- --help

lotus 0.1.0
A handful cli to convert html table to markdown table.

USAGE:
    main [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <OUTPUT>    output file to save markdown table (default to stdout)

ARGS:
    <INPUT>    input html file with tables
```

# Example Output

```shell
$ cargo run -- example/html/test.html

| | get | insert | remove | predecessor | append|
|--- | --- | --- | --- | --- | ---|
|HashMap | O(1)~ | O(1)~* | O(1)~ | N/A | N/A|
|BTreeMap | O(log n) | O(log n) | O(log n) | O(log n) | O(n+m)|
```
