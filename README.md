# comma-cache

A tool to manipulate [comma](https://github.com/nix-community/comma) choice cache.

## Disclaimer

This tool heavily relies on how and where [comma](https://github.com/nix-community/comma) serializes its cache.


## Usage

```bash
comma-cache              # List cache entries
comma-cache list         # List cache entries
comma-cache remove KEY   # Remove a cache entry
```


## Installation

```bash
cargo install comma-cache --release
```

