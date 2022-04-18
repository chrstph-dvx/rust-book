# Shadowing vs mutations

```rust
let mut x = 5
x = 6
```

- type need to be the same
- 1 variable name, 1 storage location

vs

```rust
let x = 5
let x = 6
```

- 1 variable name, 2 storage locations

https://stackoverflow.com/a/53235438/2551816

# Trivia

- `_` for thousands separator: `let x = 1_000;`
- `Array` and `tuple` can't be used in println without implementing display trait
- Accessing `tuple` with `.`: `tup.0`
- Accessing `array` with `[]`: `arr[0]`
