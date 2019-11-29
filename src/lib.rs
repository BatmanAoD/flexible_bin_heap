// FIRST IMPL: no macro
/* 

use std::collections::BinaryHeap;

use std::cmp::Ordering;
use std::iter::FromIterator;
use std::fmt::Debug;


// XXX how to get `Debug` here? Need to mark it for `T` everywhere?
// XXX similar: Clone
trait ContainerLike<T: PartialOrd>: /* Clone + Debug + */ Default + FromIterator<T> + IntoIterator { }
trait CmpFn<T: PartialOrd> {
    fn compare(l: &T, r: &T) -> Option(Ordering);
}

// #[derive(Debug)]
struct ItemWrapper<T: PartialOrd, F: CmpFn<T>> (T);

impl<T: PartialOrd, F: CmpFn<T>> PartialOrd for ItemWrapper<T, F> {
    fn partial_cmp(&self, other: &T) -> Option(Ordering) {
        F::compare(self, other)
    }
}

trait WithCmp {
    type NewContainer;
    // XXX how does `F` get used here?
    fn with_cmp<F: CmpFn<T>>() -> ???
}

impl<T: PartialOrd> WithCmp for BinaryHeap<T> {
    fn with_cmp<F: CmpFn<T>>() -> BinaryHeap<ItemWrapper<T, F>> {
        BinaryHeap::new()
    }
}
*/

// SECOND IMPL: Macro

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_attribute]
pub fn with_comparator(cmp: TokenStream, newtype: TokenStream) -> TokenStream {
    let decl = parse_macro_input!(newtype as DeriveInput);
    let comparator = parse_macro_input!(cmp as Ident);
    let name = &decl.ident;
    TokenStream::from(quote!{
        #decl   // XXX why is this necessary? I thought proc_macros only _appended_ code?

        impl PartialOrd for #name {
            fn partial_cmp(&self, other: & #name) -> Option(Ordering) {
                self.#comparator .partial_cmp(other.#comparator)
            }
        }
    })
}
