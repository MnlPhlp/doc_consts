# doc_consts
get doc comments on fields as runtime constants

```rust
#[derive(DocConsts)]
struct Test {
    /// doc comment
    ///     with indentation
    field: (),
    /// another doc comment
    field2: (),
}

#[test]
fn it_works() {
    assert_eq!("doc comment\n    with indentation", Test::get_docs().field);
    assert_eq!("another doc comment", Test::get_docs().field2);
}
```

## no_std support
Set the `no_std` feature to remove map access and make the crate `no_std` compatible
