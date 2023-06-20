# bevy-project-1 #
A small game based on the bevy engine

## Setup Bevy to run in the browser ##

- Run the following commands:

    rustup target install wasm32-unknown-unknown

    rustup default wasm32-unknown-unknown

    cargo install wasm-pack

    cargo install cargo-generate

    sudo apt-get install npm

    cargo install wasm-server-runner

    sudo apt-get install wasm-bindgen

- Add this to your .cargo/config.toml file

    [target.wasm32-unknown-unknown]
    runner = "wasm-server-runner"

- run the game in a browser

    cargo run --target wasm32-unknown-unknown

## Build a release ##

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/

## Libraries ##

- rust syntax refresher: https://learnxinyminutes.com/docs/rust/

- bevy api: https://docs.rs/bevy/latest/bevy/

- bevy unofficial book: https://bevy-cheatbook.github.io/

- wasm with rust book: https://rustwasm.github.io/docs/book/
