# xorio

[![Crates.io](https://img.shields.io/crates/v/xorio.svg)](https://crates.io/crates/xorio)
[![Docs.rs](https://docs.rs/xorio/badge.svg)](https://docs.rs/xorio)
[![Katharos License](https://img.shields.io/badge/License-Katharos-blue)](https://github.com/katharostech/katharos-license)

[`Read`]/[`Write`] implementation that Xor's the bytes that come through it and wraps around
another [`Read`] or [`Write`].

## Examples

### Writing

```rust
let mut file = File::create("my_xored_file.bin").unwrap();
let mut writer = Xor::new(file);
writer.write_all("Hello World".as_bytes());
```

### Reading

```rust
let mut file = File::open("my_xored_file.bin").unwrap();
let mut reader = Xor::new(file);
let mut content = String::new();
reader.read_to_string(&mut content);
```

### Custom Xor Bytes

You can also customize the bytes that it will XOR the stream with. By default it uses a single
byte `0b01010101` to calculate the XOR.

```rust
let mut file = File::create("my_xored_file.bin").unwrap();
let mut writer = Xor::new_with_xor_bytes(file, vec![1, 2, 3]);
writer.write_all("Hello World".as_bytes());
```

## License

Bevy LDtk is licensed under the [Katharos License][k_license] which places certain restrictions
on what you are allowed to use it for. Please read and understand the terms before using Bevy
LDtk for your project.

[k_license]: https://github.com/katharostech/katharos-license

[`Read`]: https://doc.rust-lang.org/stable/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
