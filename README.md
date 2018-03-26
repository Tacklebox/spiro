# spiro
This is a small example of a web assembly application written in rust. Uses [stdweb](https://github.com/koute/stdweb)
## Installation
you need cargo-web installed.
````
cargo install -f cargo-web
````
You currently need rust nightly
````
rustup install nightly && rustup default nightly
````
or if you don't care about wasm and just want to play with it you can delete Web.toml and cargo-web will use enscripten instead of wasm32

## Running
after installing cargo-web, you should be able to run the application with `cargo web start` and navigating a web browser to http://[::1]:8000/
