use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Expr, Lit};

#[proc_macro_derive(DocConsts)]
pub fn doc_consts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let field_docs = match input.data {
        syn::Data::Struct(val) => parse_struct_docs(val),
        syn::Data::Enum(val) => todo!(),
        syn::Data::Union(val) => todo!(),
    };

    let fields = field_docs
        .iter()
        .map(|(field, _)| format!("{field}: &'static str"))
        .collect::<Vec<_>>()
        .join(",\n");

    let values = field_docs
        .iter()
        .map(|(field, comment)| format!("{field}: \"{comment}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    let map_items = field_docs
        .iter()
        .map(|(field, comment)| format!("(\"{field}\", \"{comment}\")"))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        "
        struct {ident}Docs {{
            {fields}
        }}

        use std::collections::HashMap;
        impl doc_consts::DocConsts for {ident}{{
            fn get_docs_map() -> HashMap<&'static str,&'static str> {{
                HashMap::from([
                    {map_items}
                ])                
            }}
        }}

        #[automatically_derived]
        impl {ident} {{
            pub const fn get_docs() -> {ident}Docs {{
                {ident}Docs{{
                    {values}
                }}
            }}
        }}
    "
    )
    .parse()
    .unwrap()
}

fn parse_struct_docs(val: syn::DataStruct) -> Vec<(String, String)> {
    let mut field_docs = vec![];
    let fields = val
        .fields
        .iter()
        .filter_map(|f| if f.ident.is_some() { Some(f) } else { None });
    for f in fields {
        if let Some(ident) = &f.ident {
            let comment = get_comment(&f.attrs);
            if comment.len() > 0 {
                field_docs.push((ident.to_string(), comment))
            }
        }
    }
    field_docs
}

fn get_comment(attrs: &Vec<syn::Attribute>) -> String {
    let mut comment = vec![];
    for attr in attrs {
        if attr.path().is_ident("doc") {
            match &attr.meta {
                syn::Meta::NameValue(val) => match &val.value {
                    Expr::Lit(lit) => match &lit.lit {
                        Lit::Str(c) => {
                            let c = c.value();
                            comment.push(c.strip_prefix(" ").unwrap_or(c.as_str()).to_string())
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
    return comment.join("\n");
}
