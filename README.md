# circular_buffer

A growable circular buffer for working with bytes in Rust.

## Example

```rust
use circular_buffer::CircularBuffer;

let mut buffer = CircularBuffer::new(4);
buffer.push(1);
buffer.push(2);
buffer.push(3);
assert_eq!(&buffer[..], [1, 2, 3]);
```


# API

```rust
struct CircularBuffer {
    storage: Vec<u8>,
    start: usize,
    end: usize,
}

impl CircularBuffer {
    fn new(capacity: usize) -> Self;
    fn push(&mut self, element: u8);
    fn pop(&mut self) -> Option<u8>;
}

impl Deref for CircularBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target;
}

impl DerefMut for CircularBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

# LICENSE

This library is licensed under the MIT license. See LICENSE for details.

