# gifme

`gifme` is a CLI tool that pulls a random GIF from GIPHY and displays it directly in your terminal... so long as you're using iTerm2. It's a fun way to bring some pizzaz to your command line scripts.

![Running `gifme`](https://github.com/rgbkrk/gifme/assets/836375/97122037-f5f9-44ed-a1fe-df8ceae318d4)

## Features

* Fetch random GIFs from GIPHY
* Filter GIFs by tags
* Specify a content rating for the GIFs
* Display inline in iTerm2

## Installation

### Prerequisites

* [Rust!](https://www.rust-lang.org/tools/install)

### Building from Source

1. Clone the repository

```
git clone https://github.com/rgbkrk/gifme.git
cd gifme
```

2. Build the project

```
cargo build --release
```

3. Use the executable from `target/release/gifme`

## Usage

Run `gifme` to fetch a random GIF.

* Fetch a random GIF: `gifme`
* Fetch a GIF with a specific tag: `gifme lambda`
* Fetch a GIF, limited by content rating: `gifme --rating=pg`
* Pipe tags to `gifme` (`command | gifme`)

### Options

* `--rating=[g|pg|pg-13|r]`: Specify the content rating for the GIF (e.g., g, pg, pg-13, r).
* `[tags]`: Enter one or more tags to filter the GIFs.

## Roadmap

* [x] CLI that can display a random GIF
* [x] Able to pipe tags to `gifme`. E.g. `echo "hey" | gifme`
* [ ] Expose more GIPHY endpoints for funsies

## Contributing

Send pull requests. I'll review them.
