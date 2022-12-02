## Metacritic CLI

A very simple HTML scraper for [https://www.metacritic.com/](metacritic) made in Rust. Unfortunately metacritic has no API, so this is the only way to make a CLI app to search scores of games, movies etc.

## Prerequisities

- rustc 1.65.0

- cargo 1.65.0

## Dependencies

- clap = { version = "4.0.25", features = ["derive"] }

- colored = "2.0.0"

- reqwest = {version = "0.11", features = ["blocking"]}

- scraper = "0.12.0"

- urlencoding = "2.1.2"    

# Quick start

Just run this commands in your terminal:

```bash
git clone https://github.com/Nithe14/metacritic-cli.git
cd metacritic-cli
cargo install --path .
```

Wait till isntallation complete. Now you can use my program `metacritic-cli`. Run this command to learn how to use it:

```bash
metacritic-cli -h 
```
