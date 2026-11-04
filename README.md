A small utility for tracking your activity in programs using niri.

## Installation

### Manual installation

You need a Rust toolchain with cargo.

```sh
git clone https://github.com/InvalidSyntax443/niri_time_tracker.git
cd niri_time_tracker
cargo build --release
```

The compiled file is located in target/release/

### NixOS

Add to flake.nix

```nix
niri_tracker.url = "github:InvalidSyntax443/niri_time_tracker";
```

Then into their own packages

```nix
environment.systemPackages = [ inputs.niri_tracker.packages.x86_64-linux.default ];
```

```nix
home.packages = [ inputs.niri_tracker.packages.x86_64-linux.default ];
```
