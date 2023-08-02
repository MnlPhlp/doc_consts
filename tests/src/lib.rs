#[cfg(test)]
mod tests {
    use doc_consts::DocConsts;

    #[derive(DocConsts)]
    struct Test {
        /// doc comment
        ///     with indentation
        field: (),
    }

    #[test]
    fn it_works() {
        assert_eq!("doc comment\n    with indentation", Test::get_docs().field);
    }
}
