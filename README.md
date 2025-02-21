# Off-by-One Error in Rust
This repository demonstrates a common off-by-one error in Rust and its solution.
The error occurs when accessing array or vector elements with an index that is out of the valid range, commonly caused by incorrect loop conditions or indexing calculations.  This can lead to unexpected behavior, crashes, or security vulnerabilities.
The `bug.rs` file contains the erroneous code. The `bugSolution.rs` demonstrates a corrected version.