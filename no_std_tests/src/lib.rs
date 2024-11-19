#![no_std]

use doc_consts::DocConsts;

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
