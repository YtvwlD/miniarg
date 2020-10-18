# miniarg

A minimal argument parser, with support for no-std and no-alloc

Only cmdlines in the form of `program -foo value -bar value` are supported.
(That means: values are strings, keys start with a single dash, keys can occur multiple times.)

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
miniarg = "0.1"
```
The feature `std` is enabled by default and `alloc` and `derive` are optional.

## Examples

A minimal example looks like this:
```rust
let cmdline = "executable -key value";
let mut args = miniarg::parse(&cmdline, &["key"]);
assert_eq!(args.next(), Some(Ok((&"key", "value"))));
assert_eq!(args.next(), None);
```

You can use `collect::<Result<Vec<_>, _>>()` to get a `Vec`:
```rust
let cmdline = "executable -key value";
let args = miniarg::parse(&cmdline, &["key"]).collect::<Result<Vec<_>, _>>()?;
assert_eq!(args, vec![(&"key", "value")]);
```

If you compile with `std` or `alloc`, it also supports passing [`ToString`] instead of strings,
for example your own enum:
```rust
#[derive(Debug,PartialEq)]
enum MyKeys {
    Foo,
    Bar,
}
impl std::fmt::Display for MyKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
let cmdline = "executable -foo value -bar value";
let args = miniarg::parse(&cmdline, &[MyKeys::Foo, MyKeys::Bar])
.collect::<Result<Vec<_>, _>>()?;
assert_eq!(args, vec![(&MyKeys::Foo, "value"), (&MyKeys::Bar, "value")]);
```
As you can see, the first character of the enum kinds is converted to lowercase.

If you compile with `derive`, you can use a custom derive instead:
```rust
#[derive(Debug,PartialEq,Key)]
enum MyKeys {
    Foo,
    Bar,
}
let cmdline = "executable -foo value -bar value";
let args = MyKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>()?;
assert_eq!(args, vec![(&MyKeys::Foo, "value"), (&MyKeys::Bar, "value")]);
```

The code never panics, but the returned iterator will contain [`ParseError`]s
if anything goes wrong.

[`ToString`]: https://doc.rust-lang.org/nightly/alloc/string/trait.ToString.html
[`ParseError`]: enum.ParseError.html

License: MPL-2.0
