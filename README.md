# Game of Life Trick
A [Matricks](https://github.com/wymcg/matricks) plugin that runs [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).

![lifetrick](https://github.com/wymcg/life_trick/assets/3410869/d44bea1d-480a-4046-a6f7-03810725f436)

## Build
- Install the `wasm32-wasi` toolchain by running `rustup target add wasm32-wasi`
- Run `cargo build --release --target wasm32-wasi`
- Run the plugin with [Matricks](https://github.com/wymcg/matricks) (on a Raspberry Pi) or with [Simtricks](https://github.com/wymcg/simtricks) (on other devices).
