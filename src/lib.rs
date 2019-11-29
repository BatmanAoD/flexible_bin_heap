extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Data, Fields, parse_macro_input};

#[proc_macro_attribute]
pub fn with_comparator(cmp: TokenStream, newtype: TokenStream) -> TokenStream {
    let decl = parse_macro_input!(newtype as DeriveInput);
    let comparator = parse_macro_input!(cmp as Ident);
    let name = &decl.ident;
    let wrapped = match &decl.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(f) => f.unnamed.first().unwrap(),
            // TODO nicer error handling
            _ => panic!("not a tuple struct")
        }
        _ => panic!("Not a struct")
    };
    TokenStream::from(quote!{
        #[derive(Debug)]    // TODO only derive traits that exist for wrapped struct
        #decl

        impl From<#wrapped> for #name {
            fn from(w: #wrapped) -> #name {
                #name ( w )
            }
        }

        impl PartialEq for #name {
            fn eq(&self, other: & #name) -> bool {
                (self.0).#comparator .eq(&(other.0).#comparator)
            }
        }

        impl Eq for #name { }

        // XXX why can't `PartialOrd` be derived based on `Ord`?
        impl PartialOrd for #name {
            fn partial_cmp(&self, other: & #name) -> Option<Ordering> {
                (self.0).#comparator .partial_cmp(&(other.0).#comparator)
            }
        }

        impl Ord for #name {
            fn cmp(&self, other: & #name) -> Ordering {
                // To allow floats, just panic  if values are not comparable
                (self.0).#comparator .partial_cmp(&(other.0).#comparator).unwrap()
            }
        }
    })
}
