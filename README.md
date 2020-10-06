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

These benchmarks are designed for experimentation and the pursuit of knowledge but it should be noted that the Rust compiler does a great deal to strive for system performance limits.

Premature optimization can hurt an engineer's performance but having intuition as to how a language performs does not.

### Memory Safety

Memory-safety is a first-class citizen in Rust.

References:

- [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

### Zero-Cost Abstractions

The Rust compiler, most times, assembles operations in the same manor regardless of the high-level language implementations.

// fixme

## License

MIT
