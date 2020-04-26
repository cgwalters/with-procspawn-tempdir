Macro that uses https://docs.rs/procspawn/ to run
functions in a new process in a temporary directory.

Development note
---

`procspawn` requires a global initializer.  This makes it hard
to use Rust's builtin `#[test]` framework.  So our tests instead
use:

```
$ cargo run --example tests
```
