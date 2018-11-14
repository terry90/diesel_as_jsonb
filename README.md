# Diesel As JSONB (for PG)

## Usage

```rust
#[derive(AsJsonb)]]
struct Something {
    thing: String,
}

struct Wrapper {
    things: Vec<Something> // For JSONB[]
    thing: Something // For JSONB
}
```