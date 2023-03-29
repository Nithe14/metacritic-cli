## Metacritic CLI

A very simple HTML scraper for [https://www.metacritic.com/](metacritic) made in Rust. Unfortunately metacritic has no API, so this is the only way to make a CLI app to search scores of games, movies etc.

You can provide title as an argument and get score and basic info about platform and release date or you can provide "coming-soon" to get upcoming game releases. The options should work the same way in both use cases.

## Prerequisities

- rustc 1.68.0

- cargo 1.68.0

## Dependencies

- clap = { version = "4.0.25", features = ["derive"] }

- colored = "2.0.0"

- reqwest = {version = "0.11", features = ["blocking"]}

- scraper = "0.12.0"
- serde = "1.0.158"
- serde_derive = "1.0.158"

- serde_json = "1.0.89"

- urlencoding = "2.1.2"    

# Quick start

For testing purposes:
```bash
git clone https://github.com/Nithe14/metacritic-cli.git
cd metacritic-cli
cargo run -- -h
```
Install it as a binary:

```bash
git clone https://github.com/Nithe14/metacritic-cli.git
cd metacritic-cli
cargo install --path .
# Add cargo path to your path (in .bashrc etc)
export PATH=~/.cargo/bin:$PATH
metacritic-cli -h
```
How to use:

```bash
$ metacritic-cli -h 

Usage: metacritic-cli [OPTIONS] <NAME>

Arguments:
  <NAME>
          Word to search.

          Example: metacritic-cli witcher\ 3

Options:
  -s, --single
          Print only the first object from the result page.
          Works as -n 1.
 -j, --json
          Print output as json

  -n, --number-of-results <NUMBER_OF_RESULTS>
          Print only n first objects from the result page (n = 1-10)

          [default: 3]

  -t, --type <ITYPE>
          Specify object type.
          Available types:
          movie, game, album, tv, person, video, company, story, all

          [default: all]

  -p, --platform <PLATFORM>
          Specify platform (only for game type for now).
          Available options:
          ps, ps2, ps3, ps4, xbox, xbox-360, xbox-one, switch, pc, ds, 3ds, ps-vita, psp, wii, wii-u, gameboy-advance, iphone, all

          [default: all]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
