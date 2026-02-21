# pleme-providers

Multi-provider integration library for Pleme platform - generic trait abstraction and registry

## Installation

```toml
[dependencies]
pleme-providers = "0.1"
```

## Usage

```rust
use pleme_providers::{Provider, ProviderRegistry};

#[async_trait]
impl Provider for MyProvider {
    async fn execute(&self, input: &Input) -> Result<Output> {
        // Provider implementation
    }
}

let registry = ProviderRegistry::new();
registry.register("my-provider", MyProvider::new());
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `reqwest` | HTTP client error conversion (default) |

Enable features in your `Cargo.toml`:

```toml
pleme-providers = { version = "0.1", features = ["full"] }
```

## Development

This project uses [Nix](https://nixos.org/) for reproducible builds:

```bash
nix develop            # Dev shell with Rust toolchain
nix run .#check-all    # cargo fmt + clippy + test
nix run .#publish      # Publish to crates.io (--dry-run supported)
nix run .#regenerate   # Regenerate Cargo.nix
```

## License

MIT - see [LICENSE](LICENSE) for details.
