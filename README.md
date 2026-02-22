# Minecraft CLI Launcher

A lightweight command-line launcher for Minecraft written in Rust.

## Features

- Automatic version manifest fetching
- Library downloading and management
- Microsoft OAuth 2.0 authentication
- Configurable game settings via TOML
- Support for multiple Minecraft versions

## Configuration

Create a `config.toml` file in the project root:

```toml
[user]
username = "your_username"
version = "1.21.11"
uuid = "your-uuid-here"

[directories]
java = "path/to/javaw.exe"
instance = "./instance"

[launch]
features = { is_demo_user = false, has_custom_resolution = false }
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## License

MIT
