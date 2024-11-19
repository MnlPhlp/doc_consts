#[cfg(test)]
mod tests {
    use doc_consts::DocConsts;

    #[allow(unused)]
    #[derive(DocConsts)]
    struct Test {
        /// doc comment
        ///     with indentation
        field: (),
        /// another doc comment
        field2: (),
    }

    #[allow(unused)]
    #[derive(DocConsts)]
    struct Test2 {
        /// a third doc comment
        field: (),
    }

    #[test]
    fn it_works() {
        assert_eq!("doc comment\n    with indentation", Test::get_docs().field);
        assert_eq!("another doc comment", Test::get_docs().field2);
        assert_eq!("a third doc comment", Test2::get_docs().field);
    }

    fn trait_test(_: impl DocConsts) {}

    #[test]
    fn trait_works() {
        trait_test(Test {
            field: (),
            field2: (),
        });
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

    #[derive(DocConsts)]
    pub struct GenericStruct<T>
    where
        T: Clone,
    {
        /// Hello, world!
        pub non_generic_field: u32,

        /// Hello, T!
        pub generic_field: T,
    }

    #[test]
    fn derive_generic_field_works() {
        let doc_struct = GenericStruct::<u8>::get_docs();
        assert_eq!(doc_struct.non_generic_field, "Hello, world!");
        assert_eq!(doc_struct.generic_field, "Hello, T!");
    }

    #[test]
    fn derive_generic_instance_works() {
        let instance = GenericStruct {
            non_generic_field: 1,
            generic_field: 2u16,
        };
        assert_doc_for_instance(&instance);
    }

    fn assert_doc_for_instance<T: Clone>(instance: &GenericStruct<T>) {
        assert_eq!(GenericStruct::<T>::get_docs().generic_field, "Hello, T!");
        drop(instance.generic_field.clone());
        let _ = instance.non_generic_field;
    }
}
