extern crate proc_macro;

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
// use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Fields, FieldsNamed, ItemStruct};

macro_rules! ensure {
    ($input:expr => $enum:ident::$field:pat => $retval:tt) => {{
        ensure!($input => $enum::$field => $retval, "invalid syntax")
    }};
    ($input:expr => $enum:ident::$field:pat => $retval:tt, $msg:expr) => {{
        use $enum::*;
        match $input {
            $field => $retval,
            v => abort!(v.span(), $msg)
        }
    }}
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(table))]
struct TableOption {
    #[darling(default)]
    name: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(get))]
struct GetOption {
    #[darling(default)]
    pk: bool,
}

#[proc_macro_derive(Get, attributes(table, get))]
#[proc_macro_error]
pub fn get(ast: TokenStream) -> TokenStream {
    let ast2 = ast.clone();
    let st = parse_macro_input!(ast2 as ItemStruct);
    let struct_ident = st.ident;
    let fields: FieldsNamed = ensure!(st.fields => Fields::Named(v) => v);

    let di = parse_macro_input!(ast as DeriveInput);
    let opt = TableOption::from_derive_input(&di).unwrap();
    let table_name = opt
        .name
        .unwrap_or(struct_ident.to_string().to_ascii_lowercase());

    let field = fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = GetOption::from_field(f).unwrap();
            if attr.pk {
                Some(f)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let pk = match field.len() {
        0 => abort!(fields.span(), "`#[get(pk)]` must be specified for the primary key"),
        1 => field[0].clone(),
        _ => abort!(fields.span(), "`#[get(pk)]` can be used only once")
    };
    let (pk_ident, pk_ty) = (pk.ident.clone().unwrap(), pk.ty.clone());
    let query = format!(
        "SELECT * FROM {} WHERE {} = $1",
        table_name,
        pk_ident,
    );
    let gen = quote! {
        impl #struct_ident {
            pub async fn get(pool: & ::sqlx::PgPool, #pk_ident: #pk_ty) -> Result<(), ::sqlx::Error> {
                ::sqlx::query_as!(
                    Self,
                    #query,
                    #pk_ident,
                )
                .fetch_one(pool)
                .await?;
                Ok(())
            }
        }
    };
    eprintln!("{:?}", query);
    gen.into()
}

#[derive(Debug, FromField)]
#[darling(attributes(create))]
struct CreateOption {
    #[darling(default)]
    ignore: bool,
}

#[proc_macro_derive(Create, attributes(table, create))]
#[proc_macro_error]
pub fn create(ast: TokenStream) -> TokenStream {
    let ast2 = ast.clone();

    let st = parse_macro_input!(ast2 as ItemStruct);
    let struct_ident = st.ident;
    let fields: FieldsNamed = ensure!(st.fields => Fields::Named(v) => v);

    let di = parse_macro_input!(ast as DeriveInput);
    let opt = TableOption::from_derive_input(&di).unwrap();
    let table_name = opt
        .name
        .unwrap_or(struct_ident.to_string().to_ascii_lowercase());

    let field_idents = fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = CreateOption::from_field(f).unwrap();
            if !attr.ignore {
                Some(f.ident.clone().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let field_names = field_idents
        .iter()
        .map(|f| f.clone().to_string())
        .collect::<Vec<_>>();
    let fn_args = fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = CreateOption::from_field(f).unwrap();
            if !attr.ignore {
                let ident = f.ident.clone();
                let ty = f.ty.clone();
                Some(quote!(#ident: #ty))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let incremental = (1..=field_names.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>();
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        field_names.join(", "),
        incremental.join(", ")
    );

    let gen = quote! {
        impl #struct_ident {
            pub async fn create(pool: & ::sqlx::PgPool, #(#fn_args),*) -> Result<(), ::sqlx::Error> {
                ::sqlx::query!(
                    #query,
                    #(#field_idents),*
                )
                .execute(pool)
                .await?;
                Ok(())
            }
        }
    };
    eprintln!("{:?}", query);
    gen.into()
}

#[derive(Debug, FromField)]
#[darling(attributes(update))]
struct UpdateOption {
    #[darling(default)]
    ignore: bool,
}

#[proc_macro_derive(Update, attributes(table, update, get))]
pub fn update(ast: TokenStream) -> TokenStream {
    let ast2 = ast.clone();
    let st = parse_macro_input!(ast2 as ItemStruct);
    let struct_ident = st.ident;
    let fields: FieldsNamed = ensure!(st.fields => Fields::Named(v) => v);

    let di = parse_macro_input!(ast as DeriveInput);
    let opt = TableOption::from_derive_input(&di).unwrap();
    let table_name = opt
        .name
        .unwrap_or(struct_ident.to_string().to_ascii_lowercase());

    let field = fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = GetOption::from_field(f).unwrap();
            if attr.pk {
                Some(f)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let pk = match field.len() {
        0 => abort!(fields.span(), "`#[delete(pk)]` must be specified for the primary key"),
        1 => field[0].clone(),
        _ => abort!(fields.span(), "`#[delete(pk)]` can be used only once")
    };
    let (pk_ident, pk_ty) = (pk.ident.clone().unwrap(), pk.ty.clone());

    let field_idents = fields
        .named
        .iter()
        .filter_map(|f| {
            let update_attr = UpdateOption::from_field(f).unwrap();
            let get_attr = GetOption::from_field(f).unwrap();
            if !update_attr.ignore && !get_attr.pk {
                Some(f.ident.clone().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let field_names = field_idents
        .iter()
        .map(|f| f.clone().to_string())
        .collect::<Vec<_>>();
    let fn_args = fields
        .named
        .iter()
        .filter_map(|f| {
            let update_attr = UpdateOption::from_field(f).unwrap();
            let get_attr = GetOption::from_field(f).unwrap();
            if !update_attr.ignore && !get_attr.pk {
                let ident = f.ident.clone();
                let ty = f.ty.clone();
                Some(quote!(#ident: #ty))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let col_setter = fields
        .named
        .iter()
        .filter_map(|f| {
            let update_attr = UpdateOption::from_field(f).unwrap();
            let get_attr = GetOption::from_field(f).unwrap();
            if !update_attr.ignore && !get_attr.pk {
                let ident = f.ident.clone().unwrap();
                let set_ident = syn::Ident::new(&format!("set_{}", &ident), ident.span());
                let ty = f.ty.clone();
                let query = format!("UPDATE {} SET {} = $1 WHERE {} = $2", &table_name, &pk_ident, &ident);
                Some(quote!(
                    pub async fn #set_ident(pool: & ::sqlx::PgPool, #pk_ident: #pk_ty, #ident: #ty) -> Result<(), ::sqlx::Error> {
                        ::sqlx::query!(#query, #pk_ident, #ident)
                            .execute(pool)
                            .await?;
                        Ok(())
                    }
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let incremental = (2..=field_names.len()+1)
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>();
    let query = format!(
        "UPDATE {} SET {} WHERE {} = $1",
        table_name,
        field_names.iter().zip(incremental.iter()).map(|(f, i)| format!("{} = {}", f, i)).collect::<Vec<_>>().join(","),
        &pk_ident
    );
    
    let gen = quote! {
        impl #struct_ident {
            pub async fn update(pool: & ::sqlx::PgPool, #pk_ident: #pk_ty, #(#fn_args),*) -> Result<(), ::sqlx::Error> {
                ::sqlx::query!(
                    #query,
                    #pk_ident,
                    #(#field_idents),*
                )
                .execute(pool)
                .await?;
                Ok(())
            }

            #(#col_setter)*
        }
    };
    eprintln!("{:?}", query);
        
    gen.into()
}


#[proc_macro_derive(Delete, attributes(table, get))]
#[proc_macro_error]
pub fn delete(ast: TokenStream) -> TokenStream {
    let ast2 = ast.clone();
    let st = parse_macro_input!(ast2 as ItemStruct);
    let struct_ident = st.ident;
    let fields: FieldsNamed = ensure!(st.fields => Fields::Named(v) => v);

    let di = parse_macro_input!(ast as DeriveInput);
    let opt = TableOption::from_derive_input(&di).unwrap();
    let table_name = opt
        .name
        .unwrap_or(struct_ident.to_string().to_ascii_lowercase());

    let field = fields
        .named
        .iter()
        .filter_map(|f| {
            let attr = GetOption::from_field(f).unwrap();
            if attr.pk {
                Some(f)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let pk = match field.len() {
        0 => abort!(fields.span(), "`#[delete(pk)]` must be specified for the primary key"),
        1 => field[0].clone(),
        _ => abort!(fields.span(), "`#[delete(pk)]` can be used only once")
    };
    let (pk_ident, pk_ty) = (pk.ident.clone().unwrap(), pk.ty.clone());
    let query = format!(
        "DELETE FROM {} WHERE {} = $1",
        table_name,
        pk_ident,
    );
    let gen = quote! {
        impl #struct_ident {
            pub async fn delete(pool: & ::sqlx::PgPool, #pk_ident: #pk_ty) -> Result<(), ::sqlx::Error> {
                ::sqlx::query!(
                    #query,
                    #pk_ident,
                )
                .execute(pool)
                .await?;
                Ok(())
            }
        }
    };
    eprintln!("{:?}", query);
    gen.into()
}
