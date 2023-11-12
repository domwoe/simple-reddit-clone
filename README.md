# Simple Reddit Clone

This project is a simple example of a Reddit-like social application running entirely on the Internet Computer.
It is based on the [Vite + React + Rust](https://github.com/rvanasa/vite-react-ic-rust) template.

Check out [Vite + React + Motoko](https://github.com/rvanasa/vite-react-motoko) for a beginner-friendly starter project with a [Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko) backend.

## What's included

- How to use the Rust Canister Development Kit (CDK)
- How to use stable memory on the Internet Computer using the [stable structures library](https://docs.rs/ic-stable-structures/latest/ic_stable_structures/)
- How to use [`@dfinity/agent`](https://www.npmjs.com/package/@dfinity/agent) to interact with the backend canister from a React frontend
- How to use Internet Identity for authentication and guard canister methods

## Super Quick Start

Run and develop in your browser:

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/domwoe/simple-reddit-clone)

When using Gitpod, please set `ALLOW_ANONYMOUS` to `true` in `backend/src/lib.rs` to skip logging in via Internet Identity.

For an even simpler version of the dapp without (Internet Identity) authentication, see [this branch](https://github.com/domwoe/simple-reddit-clone/tree/simple-without-auth)

## Prerequisites

Make sure that [Node.js](https://nodejs.org/en/) `>= 16.x`, [`dfx`](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) `>= 0.12.x`, and [Rust](https://www.rust-lang.org/tools/install) are installed on your system.

After installing Rust, run these commands to configure your system for IC canister development:

```sh
rustup target add wasm32-unknown-unknown # Required for building IC canisters
cargo install cargo-watch # Optional; used for live reloading in `npm start`
```

## Quick Start

Next, run the following commands in a new, empty project directory:

```sh
git clone degit rvanasa/vite-react-ic-rust # Clone this repo
dfx start --clean --background # Run local IC replica in the background
npm run setup # Install packages, deploy canisters, and generate type bindings

npm start # Start the development server
```

When ready, run `dfx deploy` to build and deploy your application locally.

### Deploying to the Internet Computer (mainnet)

#### Playground (Temporary)

You can deploy your dapp to the playground, where your dapp will be garbage collected after ~ 20m.

```sh
dfx deploy --playground
```

#### Permanent

You'll need to have cycles to deploy to the Internet Computer. You can get cycles from the [faucet](https://faucet.dfinity.org/).

```sh
dfx deploy --network ic
```

You


## Technology Stack


- [IC Rust CDK](https://docs.rs/ic-cdk/latest/ic_cdk/): Canister Development Kit for Rust
- [Stable Structures](https://docs.rs/ic-stable-structures/latest/ic_stable_structures/): a collection of scalable data structures for the Internet Computer that persist across upgrades.
- [Internet Identity](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/overview): authentication service for the Internet Computer
- [Vite](https://vitejs.dev/): high-performance tooling for front-end web development
- [React](https://reactjs.org/): a component-based UI library
- [TypeScript](https://www.typescriptlang.org/): JavaScript extended with syntax for types
- [Sass](https://sass-lang.com/): an extended syntax for CSS stylesheets
- [Prettier](https://prettier.io/): code formatting for a wide range of supported languages
- [Rust](https://www.rust-lang.org/): a fast, safe programming language for writing [Internet Computer](https://internetcomputer.org/) canisters


## Documentation

- [Vite developer docs](https://vitejs.dev/guide/)
- [React quick start guide](https://beta.reactjs.org/learn)
- [Internet Computer docs](https://internetcomputer.org/docs/current/developer-docs/ic-overview)
- [Rust developer docs](https://rustc-dev-guide.rust-lang.org/)
- [`dfx.json` reference schema](https://internetcomputer.org/docs/current/references/dfx-json-reference/)

## Tips and Tricks

- Customize your project's code style by editing the `.prettierrc` file and then running `npm run format`.
- Reduce the latency of update calls by passing the `--emulator` flag to `dfx start`.
- Split your frontend and backend console output by running `npm run frontend` and `npm run backend` in separate terminals.


