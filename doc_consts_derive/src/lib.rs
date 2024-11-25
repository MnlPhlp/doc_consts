use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Expr};

#[proc_macro_derive(DocConsts)]
pub fn doc_consts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_docs = match input.data {
        syn::Data::Struct(val) => parse_struct_docs(&val),
        syn::Data::Enum(_val) => todo!(),
        syn::Data::Union(_val) => todo!(),
    };

    let fields = field_docs
        .iter()
        .map(|(field, _)| {
            quote! {
                pub #field: &'static str,
            }
        })
        .collect::<proc_macro2::TokenStream>();

    let values = field_docs
        .iter()
        .map(|(field, comment)| {
            quote! {
                #field: #comment,
            }
        })
        .collect::<proc_macro2::TokenStream>();

    #[cfg(not(feature = "no_std"))]
    let map_items = field_docs
        .iter()
        .map(|(field, comment)| {
            let field = field.to_string();
            quote! {
                (#field, #comment),
            }
        })
        .collect::<proc_macro2::TokenStream>();

    let ident_docs = format_ident!("{}Docs", ident);

    let impl_generics = impl_generics.to_token_stream();
    let ty_generics = ty_generics.to_token_stream();
    let where_clause = where_clause.to_token_stream();

    let stream = quote! {
        pub struct #ident_docs {
            #fields
        }

        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const fn get_docs() -> #ident_docs {
                #ident_docs{
                    #values
                }
            }
        }
    };

    #[cfg(not(feature = "no_std"))]
    let mut stream = stream;
    #[cfg(not(feature = "no_std"))]
    stream.extend(quote! {
        #[automatically_derived]
        impl #impl_generics doc_consts::DocConsts for #ident #ty_generics #where_clause {
            fn get_docs_map() -> std::collections::HashMap<&'static str, &'static str> {
                std::collections::HashMap::from([
                    #map_items
                ])
            }
        }
    });

    stream.into()
}

fn parse_struct_docs(val: &syn::DataStruct) -> Vec<(proc_macro2::Ident, String)> {
    let mut field_docs = Vec::new();
    let fields = val.fields.iter().filter(|f| f.ident.is_some());
    for f in fields {
        if let Some(ident) = &f.ident {
            let comment = get_comment(&f.attrs);
            if !comment.is_empty() {
                field_docs.push((ident.clone(), comment));
            }
        }
    }
    field_docs
}

fn get_comment(attrs: &Vec<syn::Attribute>) -> String {
    let mut comment = String::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(syn::MetaNameValue {
                value:
                    Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(c),
                        ..
                    }),
                ..
            }) = &attr.meta
            {
                let c = c.value();
                let stripped = c.strip_prefix(" ").unwrap_or(c.as_str());
                comment.push_str(stripped);
                comment.push('\n');
            }
        }
    }
    if !comment.is_empty() {
        // pop last '\n'
        comment.pop();
    }
    comment
}
