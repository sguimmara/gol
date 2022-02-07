# Game Of Life

A very simple implementation of the famous [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust !

## Getting started

> Note: this application uses the [ncurses](https://crates.io/crates/ncurses) crate that depends on the [ncurses](https://linux.die.net/man/3/ncurses) library.

Install `ncurses`. For example with apt-get package manager:

```shell
sudo apt-get update 
sudo apt-get install libncurses5-dev libncursesw5-dev
```

Clone this repository:

```shell
git clone git@github.com:sguimmara/gol.git
```

Build with Cargo:

```
cargo build --release
```

Run the game:

```
cargo run
```