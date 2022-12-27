use std::str;

// Convert the buffer to a slice and pass it to a function that expects a slice of bytes
let slice = &buffer;
let s = str::from_utf8(slice).unwrap();
assert_eq!(s, "2345");

// Modify the buffer by dereferencing it as a mutable slice
let mut slice = &mut buffer;
slice[1] = b'X';

// The buffer now contains [2, 88, 4, 5]
assert_eq!(&buffer[..], [2, 88, 4, 5]);
