# WASM Life

The goal of this repository is to showcase how a Rust library can be used within a NextJS frontend, thanks to the power of WebAssembly.

It can be accessed live on GitHub Pages:

- [Text version](https://niss36.github.io/game-of-life/)
- [Canvas version (slow)](https://niss36.github.io/game-of-life/canvas)
- [WebGL version (fast)](https://niss36.github.io/game-of-life/webgl)

## Conway's Game of Life

Conway's Game of Life is a 0-player "game" in which a 2D universe evolves over time based on simple rules. See [Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) for more details.

It was chosen because it is simple enough to implement, and can be computationally heavy depending on the size of the universe.

## Why Rust / WASM?

- Better performance: 10x faster than TypeScript for large enough universes (500x500)
- Better portability: the same code can be compiled to run on many platforms (macOS, Windows, Linux, mobile, web, etc.)
- Better safety / stability: strict typing is mandatory, and the borrow checker prevents many bugs related to mutations

## Build instructions

1. Install prerequisites:
   - [Rust](https://www.rust-lang.org/tools/install)
   - [wasm-pack](https://github.com/rustwasm/wasm-pack): `cargo install wasm-pack`
   - [Node 18](https://nodejs.org/en/download/current)
   - [pnpm](https://pnpm.io/installation)
2. Install JS dependencies: `pnpm i`
3. Build the packages
   - Go to `wasm-game-of-life` and run `wasm-pack build`
   - Go to `ts-game-of-life` and run `pnpm package`
4. Start the dev server: go to `frontend` and run `pnpm dev`
5. [Optional] build the production-ready frontend: go to `frontend` and run `pnpm build`
