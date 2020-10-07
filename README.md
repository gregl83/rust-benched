# rust-benched

Rust Benchmarks repository.

The intent of this repository is to provide proofs, using benchmarks, that solutions to common logical operations are optimal.

Rust has emerged as a low-level language with intent of solving problems traditionally encountered using C/C++ without sacrificing performance. Notably, memory safety and ease-of-use.

In many ways, Rust is C++ re-imagined. 

Data based development decisions.

## Usage

```bash
$ cargo +nightly bench
```

## Considerations

These benchmarks are designed for experimentation and the pursuit of knowledge but it should be noted that the Rust compiler does a great deal to push towards the limits of computation.

Premature optimization can hurt an engineer's performance but having intuition as to how a language performs improves decision making downstream outweighing any upfront cost.

### Memory Safety

Memory-safety is a first-class citizen in Rust. Rather than rewrite "the book", please see official Rust references for memory-safety details.

References:

- [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)

### Zero-Cost Abstractions

The Rust compiler, ideally, assembles operations identically regardless of high-level language implementations (Rust source code).

For example, writing a custom "sum" function compared with a standard lib or similar function emitted as assembly would yield the same results given the same arguments and return value type. The source code or abstraction should compile to the same optimized assembly regardless of trivial variance in implementations.

## License

MIT
