# Preventing Data Races in Rust

Rust provides inherent protection against data races and thread safety issues at compile time.

Mutation of a shared vraible using 1) 'Atomically Reference Counted' [thread-safe reference-counting pointer](https://doc.rust-lang.org/std/sync/struct.Arc.html)

2) [Asynchronous channels](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html)



## Challenges

1. Experiment by adding different thread safety bugs like concurrent writes. See errors.

2. Learn about Rust concurrency patterns like Mutex, Arc, Channels. Implement solutions.

3. Use Rust analyzers in IDE to detect issues around threads and shared state.

4. Set up Clippy linter and deny warnings for detectable concurrency errors.

5. Refactor code to minimize shared mutable state across threads.