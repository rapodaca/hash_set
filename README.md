# HashSet: A Python Wrapper for Rust's HashSet Type

This project demonstrates one way to build Rust wrappers in Python from scratch. For details, see [Wrapping Rust Types as Python Classes](https://depth-first.com/articles/2020/08/03/wrapping-rust-types-as-python-classes/).

To build:

```bash
cargo build
```

To run on macOS:

```bash
LD_LIBRARY_PATH=target/debug python3

>>> import hash_set
>>> from hash_set import HashSet
>>> s = HashSet()
>>> s.contains(0)
False
>>> s.insert(0)
True
>>> s.contains(0)
True
>>> s.insert(0)
False
>>> s.insert(1)
True
>>> s.len()
2
>>> s.collect()
[1, 0]
```

Your operating system may require a different prefix to detect the native library.