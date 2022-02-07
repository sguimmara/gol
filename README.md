# Game Of Life

A very simple implementation of the famous [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust !

## Getting started

### Preparation

Clone this repository:

```
git clone git@github.com:sguimmara/gol.git
```

### Using Cargo

> Note: this application uses the [ncurses](https://crates.io/crates/ncurses) crate that depends on the [ncurses](https://linux.die.net/man/3/ncurses) library.

Install `ncurses`. For example with apt-get package manager:

```
sudo apt-get update 
sudo apt-get install libncurses5-dev libncursesw5-dev
```

Build with Cargo:

```
cargo build --release
```

Run the game:

```
cargo run
```

### Using Docker

Build the image

```
docker build -t GameOfLife .
```

Run the game:

```
docker run -it GameOfLife
```