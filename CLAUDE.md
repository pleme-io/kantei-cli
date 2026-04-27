# kantei-cli

> **★★★ CSE / Knowable Construction.** This repo operates under **Constructive Substrate Engineering** — canonical specification at [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md). The Compounding Directive (operational rules: solve once, load-bearing fixes only, idiom-first, models stay current, direction beats velocity) is in the org-level pleme-io/CLAUDE.md ★★★ section. Read both before non-trivial changes.


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
