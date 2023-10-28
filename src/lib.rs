#![recursion_limit = "256"]
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(AsJsonb)]
pub fn asjsonb_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let scope = Ident::new(
        &format!("{}_as_jsonb", name).to_snake_case(),
        Span::call_site(),
    );
    let proxy = Ident::new(&format!("{}ValueProxy", name), Span::call_site());

    let gen = quote! {
        mod #scope {
            use std::io::Write;
            use ::diesel::{AsExpression, FromSqlRow};
            use ::diesel::sql_types::Jsonb;
            use ::diesel::pg::{Pg, PgValue};
            use ::diesel::serialize::{self, IsNull, Output, ToSql};
            use ::diesel::deserialize::{self, FromSql};

            #[derive(FromSqlRow, AsExpression)]
            #[diesel(foreign_derive)]
            #[diesel(sql_type = Jsonb)]
            struct #proxy(#name);

            impl FromSql<Jsonb, Pg> for #name {
                fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
                    let bytes = bytes.as_bytes();

                    if bytes[0] != 1 {
                        return Err("Unsupported JSONB encoding version".into());
                    }
                    serde_json::from_slice(&bytes[1..]).map_err(Into::into)
                }
            }

            impl ToSql<Jsonb, Pg> for #name {
                fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
                    out.write_all(&[1])?;
                    serde_json::to_writer(out, self)
                        .map(|_| IsNull::No)
                        .map_err(Into::into)
                }
            }
        }
    };
    gen.into()
}
