# Project Description
Small project to test Rust multi-crate setup and CLI + WASM functionality alongside each other.
Strongly inspired by the toolbox-project of Tibor Hercz (in Go) found on Github [here](https://github.com/tiborhercz/toolbox).

## Project Layout
```
+-------+    +-------+
|shared | -> |cli    | - cli server --
+-------+    +-------+               |
             +-------+    +--------------+
          -> |wasm   | -> |toolboxrs-ui  |
             +-------+    +--------------+
```
- Shared crate contains the core functionality
- Cli contains the cli-applications that uses the core functionality
- Cli also contains a warp-server that gets active using the `server` command. This serves the toolboxrs-ui files.
- Wasm contains the functions that get compiled into wasm32-unknown-unknown using `wasm-pack`. This is used as a wasm-binary in the toolboxrs-ui files.


### Requirements
- make sure you have a recent version of [rustup](https://www.rust-lang.org/tools/install) installed 
- make sure you have a recent version of [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) installed.
- make sure you have Make installed using `brew install make`


#### Rust Version
Currently this project requires rust nightly (which you can fetch using `rustup toolchain install nightly`).
You can set the nightly as standard for this project by running `rustup override set nightly` in the project root.
The reason for this, is that we require the `per-package-target` feature to compile wasm to wasm32-unknown-unknown, which is different from the other 2.
You also need to redownload the target wasm32 again when switching: `rustup target add wasm32-unknown-unknown`.

### Building the project
1. To build to entire rust-project use the Makefile in the root by running `make build-all` from the project root.
This will build the project as a whole, both all three crates with their own targets (mainly for wasm getting stuck during compilation). After this the `wasm-pack build` command is executed to build a local package that can be imported into the toolboxrs-ui front-end project. This uses vite and Svelte for the front-end code, and uses the commands from the `shared` crate via the `wasm` binary.
For this Vite requires the wasm and the top-level-await plugins in order to run with webassembly.
2. To build the front-end project into a /dist-folder, run `npm install` and `npm run build` in the `wasm/toolboxrs-ui/`-folder. 
This will build the front-end components and use the latest version of your wasm-files that are outputted in the `wasm/pkg`-folder.

### Running the project locally
- Make sure you at least ran the build commands once if you want to test the `server` command of the CLI. Otherwise the `dist` folder for the UI doesn't exist yet, showing you a 404.
- You can also develop locally on the CLI by running `cargo run` followed by the command you wish to test.
- You can also develop locally on the UI by running `npm run dev` from within the npm-project (`wasm/toolboxrs-ui`). 
- There is no auto-reload set up for the Rust-part, so after changes, rerun your commands to see the latest output.

### Current list of todo's still:
- use functions from shared in wasm-crate
- built UI that uses the functions from the wasm-crate
- check release/deploy options.
