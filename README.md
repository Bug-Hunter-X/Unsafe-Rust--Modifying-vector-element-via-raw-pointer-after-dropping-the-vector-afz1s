# Unsafe Rust Bug: Modifying Vector After Dropping

This repository demonstrates a common error in unsafe Rust code. The bug involves modifying a vector's element through a raw pointer after the vector has been dropped. This leads to undefined behavior, causing unpredictable results or crashes.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file demonstrates a correct approach.

## How to Reproduce

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug`. Observe the erratic output or potential crash.
4. Compile and run `bugSolution.rs` using `rustc bugSolution.rs && ./bugSolution` to see the corrected behavior.

## Learning Points

This example highlights the importance of careful memory management when using raw pointers in Rust.  Always ensure the memory you are accessing remains valid throughout the lifetime of the pointer. Using safe abstractions wherever possible is strongly recommended. 