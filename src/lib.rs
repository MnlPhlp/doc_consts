use std::collections::HashMap;

pub use doc_consts_derive::DocConsts;

pub trait DocConsts {
    /// Returns for example `{ "field": "Documentation" }`
    fn get_docs_map() -> HashMap<&'static str, &'static str>;
}
