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

## Post-Build Installation

After building the project, you may want to create a symbolic link to the `gifme` executable in your `~/bin/` directory for easier access. You can do this with the following command:

```bash
ln -s $PWD/target/release/gifme ~/bin/
```

This command creates a symbolic link to the `gifme` executable in the `~/bin/` directory. The `$PWD` variable represents the current directory, which should be the root directory of the `gifme` project if you've followed the build instructions.

Please ensure that `~/bin/` is included in your PATH. If it's not, you can add it with the following command:


```
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
```

This command appends the line `export PATH="$HOME/bin:$PATH"` to your `~/.zshrc` file, which makes your shell able to see `gifme` from anywhere.


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
