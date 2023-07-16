# Filter Unique Items

- Implment `unique()`

It accepts a `Vec<i32>` as an argument, returning a `Vec<i32>` with no duplicates.

## Examples

```rust
let list = vec![1, 4, 5];
assert_eq!(unique(list), list);
```

```rust
let list = vec![1, 1, 3];
assert_eq!(unique(list), vec![1, 3]);
```

## Extra Credit

1. Use generics - Change `unique()` to accept a `Vec<T>` where `T` is a type that implements `Ord`.
2. Retain order - Return elements in their original order.
