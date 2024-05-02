
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, ItemFn, Block, Stmt, Item };


/// Add `#[anal_eyes]` attribute above a function to autogenerate println! calls
/// for each statement in the function, including any inner functions it may contain.
#[proc_macro_attribute]
pub fn anal_eyes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_type = &input.sig.output;
    let fn_block = &input.block;

    let debug_block = insert_print_statements(&fn_block);
    let output = quote! {
        fn #fn_name(#fn_args) -> #fn_type {
            println!("Executing function '{}'", #fn_name);
            #debug_block
        }
    };

    output.into()
}

fn insert_print_statements(block: &Block) -> proc_macro2::TokenStream {
    let mut statements = block.stmts.iter().peekable();
    let mut debug_statements = quote! {};

    let mut locals = 0u32;
    let mut exprs = 0u32;
    let mut items = 0u32;

    // Does not need to peek because Expr branch only matches if there's a semicolon,
    // meaning it should not place a println! after returns statements.
    while let Some(stmt) = statements.next() {
        let debug = match stmt {
            Stmt::Local(l) => { debug_local(&l, &mut locals) },
            Stmt::Expr(e, Some(_)) => { debug_expr(&e, &mut exprs) },
            Stmt::Item(i) => match i {
                // TODO: How to distinguish subitems in recursive calls?
                Item::Fn(item_fn) => { insert_print_statements(&item_fn.block) },
                _ => { debug_item(&i, &mut items) }
            }
            _ => { proc_macro2::TokenStream::new() }
        };

        debug_statements.extend( quote! {
            #stmt
            #debug
        });
    }

    debug_statements
}

// TODO Get local info and print
fn debug_local(_local: &syn::Local, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!("Declaration {}", #count) }
}


// TODO Get item info and print
fn debug_item(_item: &syn::Item, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!("Subitem {}", #count) }
}

// TODO Get expr info and print
fn debug_expr(_expr: &syn::Expr, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!("Espression {}", #count) }
}

