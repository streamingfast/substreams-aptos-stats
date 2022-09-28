# StreamingFast Substreams Aptos Statistics
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Quick Start (Locally)

Use this quickstart guide to set up your environment to use Substreams locally.

First, [copy this repository](https://github.com/streamingfast/substreams-aptos-stats/generate) and clone it.

## Install Dependencies

### Install Rust

We're going to be using the [Rust programming language](https://www.rust-lang.org/), to develop some custom logic.

There are [several ways to install Rust](https://www.rust-lang.org/tools/install), but for the sake of brevity:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env # to configure your current shell
```

## Obtain the `substreams` CLI tool

### From `brew` (for Mac OS)

```
brew install streamingfast/tap/substreams
```

### From pre-compiled binary

- Download the binary

```bash
# Use correct binary for your platform
wget https://github.com/streamingfast/substreams/releases/download/v0.0.20/substreams_0.0.20_linux_x86_64.tar.gz
tar -xzvf substreams_0.0.20_linux_x86_64.tar.gz
export PATH="`pwd`:$PATH"
```

> Check https://github.com/streamingfast/substreams/releases and use the latest release available

### Validation

Ensure that `substreams` CLI works as expected:

```
substreams -v
version (...)
```

## Compile

At this point, we're ready to build our WASM binary and Protobuf definitions.

```bash
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM artifact will be found at `./target/wasm32-unknown-unknown/release/substreams_aptos_stats.wasm`

## Run your Substreams

We're now ready to run our example Substreams!

> Don't forget to be at the root of the project to run the following commands

```bash
substreams run -e testnet.aptos.streamingfast.io:443 substreams.yaml store_count --stop-block +100
```

## Generating Protobuf

Generating protobuf is required only when changes to protobuf definitions has been made and updated corresponding Rust code must be generated. To update the corresponding Rust code, follow this instructions.

### Install `buf`

[https://buf.build](https://buf.build) is a tool used to simplify the generation of typed structures in any language. It invokes `protoc` and simplifies a good number of things. Substreams packages are compatible with [buf Images](https://docs.buf.build/reference/images).

See the [installation instructions here](https://docs.buf.build/installation).

### Generate

```bash
substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

## Next Steps

Congratulations! You've successfully run a Substreams.

- Read the documentation at https://github.com/streamingfast/substreams under _Documentation_.
- Look at [Playground](https://github.com/streamingfast/substreams-playground) for more learning examples.
