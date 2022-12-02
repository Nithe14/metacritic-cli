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

Wait till isntallation complete. Now you can use my program `metacritic-cli`. 

How to use:

```bash
metacritic-cli -h 

Usage: metacritic-cli [OPTIONS] <NAME>

Arguments:
  <NAME>
          Word to search.

          Example: metacritic-cli witcher\ 3

Options:
  -s, --single
          Print only the first object from the result page.
          Works as -n 1.

  -n, --number-of-results <NUMBER_OF_RESULTS>
          Print only n firsts objects from the result page (n = 1-10)

          [default: 3]

  -t, --type <ITYPE>
          Specify object type.
          Aviable types:
          movie, game, album, tv, person, video, company, story, all

          [default: all]

  -p, --platform <PLATFORM>
          Specify platform (only for game type for now).
          Aviable options:
          ps, ps2, ps3, ps4, xbox, xbox-360, xbox-one, switch, pc, ds, 3ds, ps-vita, psp, wii, wii-u, gameboy-advance, iphone, all

          [default: all]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
