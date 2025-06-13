# `gettid`

A crate to help with fetching thread IDs across multiple platforms.

```rust
use gettid::gettid;

let main_tid = gettid();
let pid = std::process::id();
let thread_tid = std::thread::spawn(gettid).join().unwrap();
assert_ne!(main_tid, thread_tid);
```
