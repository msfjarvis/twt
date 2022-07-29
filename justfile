alias b := build
alias c := check
alias cl := clippy
alias f := fmt
alias i := install
alias r := run

set positional-arguments := true
set dotenv-load := true

build type="":
    cargo build {{ type }}

check type="":
    cargo check {{ type }}

clippy flags="":
    cargo clippy -- {{ flags }}

fmt:
    cargo fmt

install:
    cargo build --release
    install -m 0755 ./target/release/twt ~/bin/

run type="":
    cargo run {{ type }}
