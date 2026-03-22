# kantei-cli

CLI compliance runner. Run GrapheneOS/CIS/STIG profiles against Android devices.

## Commands

```
kantei check <serial>                    # run GrapheneOS hardened profile
kantei check <serial> --profile <path>   # run custom YAML profile
kantei list-profiles                     # list built-in profiles
kantei report <serial> --format json     # JSON compliance report
kantei report <serial> --format text     # human-readable report
```

## Build & Test

```bash
cargo build          # build binary
cargo check          # compile check
cargo run -- --help  # CLI usage
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT license
- clippy pedantic, release profile (codegen-units=1, lto=true)
- Binary name: `kantei`
