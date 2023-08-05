# Textship #

Build a spaceship out of ASCII Symbols and fight in a 2 Player deathmatch.

## Setup Bevy to run in the browser ##

- On Linux:

    ```
    rustup target install wasm32-unknown-unknown

    cargo install wasm-pack

    sudo apt-get install npm wasm-bindgen lld

        or sudo pacman -S lld

    cargo install wasm-server-runner

    cargo install cargo-watch

    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sudo sh

    # (Optional) Install the signalling server
    cargo install matchbox_server
    ```

- On Windows (WSL):

    ```
    rustup target install wasm32-unknown-unknown

    cargo install wasm-pack

    sudo apt-get install npm wasm-bindgen

    cargo install wasm-server-runner

    cargo install cargo-watch

    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

    cargo install matchbox_server

    cargo install -f cargo-binutils 
    
    rustup component add llvm-tools-preview
    ```

## Client: Run a development build ##

- Start the wasm-server-runner:
    ```
    cargo run --target wasm32-unknown-unknown
    ```

- Open a browser tab at 127.0.0.1:1334 (or whatever the port is).

- Run the signalling server locally:
    ```
    matchbox_server
    ```

Multiple tabs can connect to the wasm-server-runner, which means you can run multiple clients on the same machine.

### Client: Run tests ###

```
wasm-pack test --headless --firefox
```

## Client: Build a release ##

```
wasm-pack build --release --target bundler --out-dir target/bundle
```

## Libraries ##

- rust syntax refresher: https://learnxinyminutes.com/docs/rust/

- bevy api: https://docs.rs/bevy/latest/bevy/

- bevy unofficial book: https://bevy-cheatbook.github.io/

- wasm with rust book: https://rustwasm.github.io/docs/book/

- wasm bindgen book: https://rustwasm.github.io/docs/wasm-bindgen/

- wasm pack book: https://rustwasm.github.io/docs/wasm-pack/

- wasm_bindgen api: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/

- web_sys api: https://rustwasm.github.io/wasm-bindgen/api/web_sys/

- wasm-bindgen-futures api: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/

- rocket api: https://api.rocket.rs/v0.5-rc/rocket/

- matchbox: https://github.com/johanhelsing/matchbox/

- wasm server runner: https://github.com/jakobhellermann/wasm-server-runner
