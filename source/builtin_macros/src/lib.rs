#![cfg_attr(
    verus_keep_ghost,
    feature(proc_macro_span),
    feature(proc_macro_tracked_env),
    feature(proc_macro_quote),
    feature(proc_macro_expand),
    feature(proc_macro_diagnostic)
)]

use proc_macro::{Ident, TokenTree};
#[cfg(verus_keep_ghost)]
use std::sync::OnceLock;
use synstructure::{decl_attribute, decl_derive};

#[macro_use]
mod syntax;
mod atomic_ghost;
mod calc_macro;
mod enum_synthesize;
mod fndecl;
mod is_variant;
mod rustdoc;
mod struct_decl_inv;
mod structural;
mod topological_sort;
use std::env;
use syn_verus::visit_mut::VisitMut;

decl_derive!([Structural] => structural::derive_structural);

decl_attribute!([is_variant] => is_variant::attribute_is_variant);
decl_attribute!([is_variant_no_deprecation_warning] => is_variant::attribute_is_variant_no_deprecation_warning);

#[proc_macro_attribute]
pub fn verus_enum_synthesize(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enum_synthesize::attribute_verus_enum_synthesize(&cfg_erase(), attr, input)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum EraseGhost {
    /// keep all ghost code
    Keep,
    /// erase ghost code, but leave ghost stubs
    Erase,
    /// erase all ghost code
    EraseAll,
}

impl EraseGhost {
    fn keep(&self) -> bool {
        match self {
            EraseGhost::Keep => true,
            EraseGhost::Erase | EraseGhost::EraseAll => false,
        }
    }

    fn erase(&self) -> bool {
        match self {
            EraseGhost::Keep => false,
            EraseGhost::Erase | EraseGhost::EraseAll => true,
        }
    }

    fn erase_all(&self) -> bool {
        match self {
            EraseGhost::Keep | EraseGhost::Erase => false,
            EraseGhost::EraseAll => true,
        }
    }
}

// Proc macros must reside at the root of the crate
#[proc_macro]
pub fn fndecl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(fndecl::fndecl(proc_macro2::TokenStream::from(input)))
}

#[proc_macro]
pub fn verus_keep_ghost(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_items(input, EraseGhost::Keep, true)
}

#[proc_macro]
pub fn verus_erase_ghost(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_items(input, EraseGhost::Erase, true)
}

fn is_user_function() -> bool {
    let args: Vec<String> = env::args().collect();

    let verus_keywords =
        vec!["libbuiltin", "libbuiltin_macros", "vstd.rs", "--extern", "--is-vstd"];

    let is_verus_build =
        args.iter().any(|arg| verus_keywords.iter().any(|keyword| arg.contains(keyword)));

    let is_user_code = args.iter().any(|arg| arg.ends_with(".rs"));

    !is_verus_build && is_user_code
}

struct ReplaceSpecNatural;

impl VisitMut for ReplaceSpecNatural {
    fn visit_ident_mut(&mut self, ident: &mut syn::Ident) {
        println!("{:?}", ident);
    }
}

use prettyplease_verus::unparse_expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn_verus::visit::visit_assert_forall;
use syn_verus::AssertForall;
use syn_verus::Expr;
use syn_verus::File;

struct TokenVisitor;

impl VisitMut for TokenVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            // Match function calls
            Expr::Call(call_expr) => {
                // Check if the function being called is `builtin::forall`
                if let Expr::Path(path) = call_expr.func.as_mut() {
                    let segments = &mut path.path.segments;
                    if segments.len() >= 2
                        && segments[0].ident == "builtin"
                        && segments[1].ident == "forall"
                    {
                        // Replace `forall` with `exists`
                        segments[1].ident = syn::Ident::new("exists", segments[1].ident.span());

                        // Print for debugging purposes
                        let formatted = unparse_expr(expr);
                        println!("Replaced forall with exists: {}", formatted);
                    }
                }
                // Continue visiting subnodes
                syn_verus::visit_mut::visit_expr_mut(self, expr);
            }
            _ => {
                // For other expressions, continue visiting subnodes
                syn_verus::visit_mut::visit_expr_mut(self, expr);
            }
        }
    }
}


fn visit_rewritten_stream(
    stream: proc_macro2::TokenStream,
) -> Result<syn_verus::File, syn_verus::Error> {
    let mut visitor = TokenVisitor;
    // Parse the token stream into a `syn_verus::File`
    if let Ok(mut ast) = syn_verus::parse2::<syn_verus::File>(stream) {
        // Mutably visit the AST nodes, including `AssertForall`
        visitor.visit_file_mut(&mut ast);
        Ok(ast) // Return the modified AST
    } else {
        Err(syn_verus::Error::new(
            proc_macro2::Span::call_site(),
            "Failed to parse stream into AST",
        ))
    }
}

#[proc_macro]
pub fn verus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let old_stream = syntax::rewrite_items(input, cfg_erase(), true);

    if !is_user_function() {
        old_stream
    } else {
        println!("Before: {}", old_stream);
        let modified = visit_rewritten_stream(old_stream.clone().into());

        match modified {
            Ok(ast) => {
                let mut token_stream = TokenStream::new();
                for item in &ast.items {
                    item.to_tokens(&mut token_stream);
                }

                println!("Modified: {}", token_stream);

                token_stream.into()
            }
            Err(e) => {
                eprintln!("Error during AST modification: {}", e);
                old_stream
            }
        }
    }
}

#[proc_macro]
pub fn verus_proof_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_expr(EraseGhost::Keep, true, input)
}

#[proc_macro]
pub fn verus_exec_expr_keep_ghost(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_expr(EraseGhost::Keep, false, input)
}

#[proc_macro]
pub fn verus_exec_expr_erase_ghost(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_expr(EraseGhost::Keep, false, input)
}

#[proc_macro]
pub fn verus_exec_expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::rewrite_expr(cfg_erase(), false, input)
}

#[cfg(verus_keep_ghost)]
pub(crate) fn cfg_erase() -> EraseGhost {
    let ts: proc_macro::TokenStream = quote::quote! { ::core::cfg!(verus_keep_ghost_body) }.into();
    let ts_stubs: proc_macro::TokenStream = quote::quote! { ::core::cfg!(verus_keep_ghost) }.into();
    let (bool_ts, bool_ts_stubs) = match (ts.expand_expr(), ts_stubs.expand_expr()) {
        (Ok(name), Ok(name_stubs)) => (name.to_string(), name_stubs.to_string()),
        _ => {
            panic!("cfg_erase call failed")
        }
    };
    match (bool_ts.as_str(), bool_ts_stubs.as_str()) {
        ("true", "true" | "false") => EraseGhost::Keep,
        ("false", "true") => EraseGhost::Erase,
        ("false", "false") => EraseGhost::EraseAll,
        _ => {
            panic!("cfg_erase call failed")
        }
    }
}

#[cfg(not(verus_keep_ghost))]
pub(crate) fn cfg_erase() -> EraseGhost {
    EraseGhost::EraseAll
}

#[cfg(verus_keep_ghost)]
pub(crate) fn cfg_verify_core() -> bool {
    static CFG_VERIFY_CORE: OnceLock<bool> = OnceLock::new();
    *CFG_VERIFY_CORE.get_or_init(|| {
        let ts: proc_macro::TokenStream = quote::quote! { ::core::cfg!(verus_verify_core) }.into();
        let bool_ts = match ts.expand_expr() {
            Ok(name) => name.to_string(),
            _ => {
                panic!("cfg_verify_core call failed")
            }
        };
        match bool_ts.as_str() {
            "true" => true,
            "false" => false,
            _ => {
                panic!("cfg_verify_core call failed")
            }
        }
    })
}

// Because 'expand_expr' is unstable, we need a different impl when `not(verus_keep_ghost)`.
#[cfg(not(verus_keep_ghost))]
pub(crate) fn cfg_verify_core() -> bool {
    false
}

/// verus_proof_macro_exprs!(f!(exprs)) applies verus syntax to transform exprs into exprs',
/// then returns f!(exprs'),
/// where exprs is a sequence of expressions separated by ",", ";", and/or "=>".
#[proc_macro]
pub fn verus_proof_macro_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::proof_macro_exprs(EraseGhost::Keep, true, input)
}

#[proc_macro]
pub fn verus_exec_macro_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::proof_macro_exprs(cfg_erase(), false, input)
}

// This is for expanding the body of an open_*_invariant in exec mode
#[proc_macro]
pub fn verus_exec_inv_macro_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // We pass `treat_elements_as_ghost: false` to treat all elements besides
    // the third ($eexpr) as ghost.
    syntax::inv_macro_exprs(cfg_erase(), false, input)
}

// This is for expanding the body of an open_*_invariant in `proof` mode
#[proc_macro]
pub fn verus_ghost_inv_macro_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // We pass `treat_elements_as_ghost: true` to treat all elements as ghost.
    syntax::inv_macro_exprs(cfg_erase(), true, input)
}

/// `verus_proof_macro_explicit_exprs!(f!(tts))` applies verus syntax to transform `tts` into
/// `tts'`, then returns `f!(tts')`, only applying the transform to any of the exprs within it that
/// are explicitly prefixed with `@@`, leaving the rest as-is. Contrast this to
/// [`verus_proof_macro_exprs`] which is likely what you want to try first to see if it satisfies
/// your needs.
#[proc_macro]
pub fn verus_proof_macro_explicit_exprs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax::proof_macro_explicit_exprs(EraseGhost::Keep, true, input)
}

#[proc_macro]
pub fn struct_with_invariants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct_decl_inv::struct_decl_inv(input)
}

#[proc_macro]
pub fn atomic_with_ghost_helper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    atomic_ghost::atomic_ghost(input)
}

#[proc_macro]
pub fn calc_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    calc_macro::calc_macro(input)
}
