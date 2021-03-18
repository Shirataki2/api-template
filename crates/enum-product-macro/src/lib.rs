use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenTree};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use regex::Regex;
use syn::spanned::Spanned;

extern crate proc_macro;

#[proc_macro]
#[proc_macro_error]
pub fn enum_product(ast: TokenStream) -> TokenStream {
    let ast2 = TokenStream2::from(ast.clone());
    let mut ast_iter = TokenStream2::from(ast.clone()).into_iter();
    let first = pick(&ast2, &mut ast_iter);
    let ident = to_ident(first);
    let visibility = match ident.to_string().as_str() {
        "pub" => true,
        "enum" => false,
        _ => abort!(ident.span(), "Expect `pub` or `enum`"),
    };
    if visibility {
        let next = to_ident(pick(&ast2, &mut ast_iter));
        match next.to_string().as_str() {
            "enum" => {}
            _ => abort!(ident.span(), "Expect `enum`"),
        };
    }
    let enum_ident = to_ident(pick(&ast2, &mut ast_iter));
    let next = extract_stream(pick(&ast2, &mut ast_iter));
    let idents = extract_groups(next);
    let (idents, strings) = listup(&idents);
    let matches = idents
        .clone()
        .into_iter()
        .zip(strings.into_iter())
        .map(|(ident, string)| quote! { &#ident => #string.to_string() })
        .collect::<Vec<_>>();
    let gen = if visibility {
        quote! {
            #[derive(Clone, Debug)]
            pub enum #enum_ident {
                #(#idents),*
            }

            impl std::string::ToString for #enum_ident {
                fn to_string(&self) -> String {
                    use #enum_ident::*;
                    match self {
                        #(#matches),*
                    }
                }
            }
        }
    } else {
        quote! {
            #[derive(Clone, Debug)]
            enum #enum_ident {
                #(#idents),*
            }

            impl std::string::ToString for #enum_ident {
                fn to_string(&self) -> String {
                    use #enum_ident::*;
                    match self {
                        #(#matches),*
                    }
                }
            }
        }
    };
    gen.into()
}

fn pick<I: Iterator<Item = TokenTree> + IntoIterator>(
    ast: &TokenStream2,
    iter: &mut I,
) -> TokenTree {
    let item = iter.next();
    match item {
        Some(item) => item,
        None => abort!(ast.span(), "Unexpected end of input"),
    }
}

fn to_ident(tt: TokenTree) -> syn::Ident {
    match tt.clone() {
        TokenTree::Ident(id) => id,
        other => abort!(tt.span(), "Expect ident but got {:?}", other),
    }
}

fn extract_stream(tt: TokenTree) -> TokenStream2 {
    match tt.clone() {
        TokenTree::Group(gr) => gr.stream(),
        other => abort!(tt.span(), "Expect block but got {:?}", other),
    }
}

fn extract_groups(ts: TokenStream2) -> Vec<Vec<String>> {
    let mut res = vec![];
    for (i, tt) in ts.clone().into_iter().enumerate() {
        if i % 2 == 0 {
            let mut res_inner = vec![];
            match tt.clone() {
                TokenTree::Group(_gr) => {
                    for (j, st) in extract_stream(tt.clone()).into_iter().enumerate() {
                        if j % 2 == 0 {
                            match st.clone() {
                                TokenTree::Literal(lit) => {
                                    res_inner.push(lit.to_string().trim_matches('"').to_string())
                                }
                                e => abort!(e.span(), "Expect literal but got {:?}", e),
                            }
                        } else {
                            match st {
                                TokenTree::Punct(_) => {}
                                _other => abort!(ts.span(), "Expect punctuator"),
                            }
                        }
                    }
                }
                _other => abort!(ts.span(), "Expect list"),
            };
            res.push(res_inner);
        } else {
            match tt {
                TokenTree::Punct(_) => {}
                _other => abort!(ts.span(), "Expect punctuator"),
            };
        }
    }
    res
}

fn listup(names: &Vec<Vec<String>>) -> (Vec<Ident>, Vec<String>) {
    let n = names.iter().fold(1, |acc, v| acc * v.len());
    let mut idents = vec![];
    let mut strings = vec![];
    for i in 0..n {
        let mut indices = vec![0; names.len()];
        let mut k = 1;
        let mut l = 1;
        for j in 0..names.len() {
            k *= names[j].len();
            indices[j] = (i % k) / l;
            l *= names[j].len();
        }
        let mut s = String::new();
        for (i, &j) in indices.iter().enumerate() {
            s = format!("{}{}", s, names[i][j]);
        }
        strings.push(s.clone());
        let mut cs = s.chars();
        let s = match cs.next() {
            None => String::new(),
            Some(f) => format!("{}{}", f.to_ascii_uppercase(), cs.as_str()),
        };
        let re = Regex::new(r#"[^A-Za-z0-9]"#).unwrap();
        let s = format!("{}", re.replace_all(&s, ""));
        idents.push(Ident::new(&s, proc_macro2::Span::call_site()));
    }
    (idents, strings)
}
