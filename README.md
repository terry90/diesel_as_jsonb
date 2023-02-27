# Diesel As JSONB (for PG)

## Usage

```rust
#[derive(AsJsonb)]]
struct Something {
    thing: String,
}

struct Wrapper {
    things: Vec<Something> // For field type Array<Jsonb>
    thing: Something // For field type Jsonb
}
```
