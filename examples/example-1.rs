// Create a new buffer with a capacity of 4
let mut buffer = CircularBuffer::new(4);

// Push some elements onto the buffer
buffer.push(1);
buffer.push(2);
buffer.push(3);

// The buffer now contains [1, 2, 3]
assert_eq!(&buffer[..], [1, 2, 3]);

// Pop an element off the buffer
let x = buffer.pop().unwrap();
assert_eq!(x, 1);

// The buffer now contains [2, 3]
assert_eq!(&buffer[..], [2, 3]);

// Push an element onto the buffer
buffer.push(4);

// The buffer now contains [2, 3, 4]
assert_eq!(&buffer[..], [2, 3, 4]);

// Push another element onto the buffer, causing the storage to grow
buffer.push(5);

// The buffer now contains [2, 3, 4, 5]
assert_eq!(&buffer[..], [2, 3, 4, 5]);
