use std::collections::HashMap;

pub trait DocConsts {
    fn get_docs_map() -> HashMap<&'static str, &'static str>;
}
