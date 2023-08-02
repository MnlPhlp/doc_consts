#[cfg(test)]
mod tests {
    use doc_consts::DocConsts;

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

    fn trait_test(_: impl DocConsts) {}

    #[test]
    fn trait_works() {
        trait_test(Test {
            field: (),
            field2: (),
        })
    }

    #[test]
    fn get_map_works() {
        let doc_map = Test::get_docs_map();
        if let Some(&doc) = doc_map.get("field2") {
            assert_eq!("another doc comment", doc);
        } else {
            panic!("doc comment not found");
        }
    }
}
