# Decoding
This is a Rust implementation of a lab for D0013E Microcomputer engineering. A coded string is decoded and then printed. Data is heap allocated using `static` declarations. This requires some `unsafe` code. If the coded data is not null terminated, the implementation tries to index outside the array and the program panics.

# Running
The program isn't interactive, so all you can do is
```$ cargo run``` 
and read the decoded string.
