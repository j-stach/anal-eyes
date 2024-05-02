
extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, ItemFn, Block, Stmt, Item };


/// Add `#[anal_eyes]` attribute above a function to autogenerate println! calls
/// for each statement in the function, including any inner functions it may contain.
#[proc_macro_attribute]
pub fn anal_eyes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let name_as_string = fn_name.to_string();
    let fn_args = &input.sig.inputs;
    let fn_type = &input.sig.output;
    let fn_block = &input.block;

    let debug_block = insert_print_statements(&name_as_string, &fn_block);
    let output = quote! {
        fn #fn_name(#fn_args) #fn_type {
            #debug_block
        }
    };

    output.into()
}

fn insert_print_statements(fn_name: &str, block: &Block) -> proc_macro2::TokenStream {
    let mut statements = block.stmts.iter();
    let mut debug_statements = quote! {
        println!("Executing '{}'", #fn_name);
    };

    let mut locals = 0u32;
    let mut exprs = 0u32;
    let mut items = 0u32;

    // Does not need to peek because Expr branch only matches if there's a semicolon,
    // meaning it should not place a println! after return statements.
    while let Some(stmt) = statements.next() {
        let debug = match stmt {
            Stmt::Local(l) => { debug_local(&l, fn_name, &mut locals) },
            Stmt::Expr(e, Some(_)) => { debug_expr(&e, fn_name, &mut exprs) },
            Stmt::Item(i) => match i {
                // TODO: How to distinguish subitems in recursive calls?
                Item::Fn(item_fn) => {
                    insert_print_statements(&item_fn.sig.ident.to_string(), &item_fn.block)
                },
                _ => { debug_item(&i, fn_name, &mut items) }
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

// TODO Get local info to print
fn debug_local(_local: &syn::Local, fn_name: &str, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!(" {}, declaration {}", #fn_name, #count); }
}


// TODO Get item info to print
fn debug_item(_item: &syn::Item, fn_name: &str, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!(" {}, sub-item {}", #fn_name, #count); }
}

// TODO Get expr info to print
fn debug_expr(_expr: &syn::Expr, fn_name: &str, count: &mut u32) -> proc_macro2::TokenStream {
    *count += 1;
    quote! { println!(" {}, expression {}", #fn_name, #count); }
}

