# Bitwise

A small Rust utility for safe, index-based bit manipulation on a `u64`.
Treats a 64-bit integer as 64 individually addressable bits (indices `0..=63`),
with bounds-checked get/set/clear/flip operations and an 8x8 board visualizer.

## Features
- Get, set, clear, and flip individual bits or multiple bits at once
- Automatic range validation (0–63) with descriptive errors
- ASCII 8x8 board rendering for visualizing bitboards (e.g. chess-style boards)

## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
bitwise = { path = "." } # or version once published
```

## Usage
```rust
use bitwise::Bitwise;

let mut bw = Bitwise::new(0);
bw.set_bit(3)?;
bw.flip_bit(5)?;
assert_eq!(bw.scan_bit(3)?, 1);
bw.clear_bit(3)?;

bw.set_multiple_bits(&[1, 2, 4])?;
println!("{}", bw.to_string(Some(4)));
```

## Documentation
Run the following command in the repo directory and open generated HTML file (by clicking the path generated in the terminal) for documentation:
```bash
cargo doc
```

## Testing
Run the following command in the repo directory for testing:
```bash
cargo test
```


## API
| Method | Description |
|---|---|
| `new(data: u64)` | Create a new instance |
| `scan_bit(index)` | Read a bit's value |
| `set_bit` / `set_multiple_bits` | Set bit(s) to 1 |
| `clear_bit` / `clear_multiple_bits` | Set bit(s) to 0 |
| `flip_bit` / `flip_multiple_bits` | Toggle bit(s) |
| `to_string(mark)` | Render as 8x8 grid, optionally highlighting one index |

## License
MIT