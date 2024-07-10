# cli-rust

Template for Command Line Interface (CLI) tool in Rust

## Development

### Setup for macOS

Install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Test:

```bash
source $HOME/.cargo/env
rustc --help
```

### Work on macOS

Configure project:

```bash
source configure.sh
```

Open the project in Visual Studio Code:

```bash
code .
```

### Build

```bash
cargo build
```

### Run

#### Run the compiled binary

```bash
pushd ./target/debug

./cli-rust

echo "John" > name.txt

./cli-rust greet name.txt
./cli-rust greet --language es name.txt
./cli-rust greet -l bg name.txt

popd
```

Output:

```bash
Hello, John!
Hola, John!
Здравей, John!
```

#### Run through `cargo`

```bash
cargo run greet name.txt
cargo run greet --language es name.txt
cargo run greet -l bg name.txt
```

### Test

```bash
cargo test
```

### Generate Docs

```bash
cargo doc
```

Browse docs:

```bash
cargo doc --open
```

### How to create a new project

```bash
# create new project
cargo init

# add packages
# see: https://crates.io/crates/clap
cargo add clap --features derive
cargo add tempfile --dev
```
