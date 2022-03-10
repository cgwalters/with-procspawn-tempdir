
//! This crate provides the [`with_procspawn_tempdir`](attr.with_procspawn_tempdir.html) macro.
//! 
//!
//! ```
//! use with_procspawn_tempdir::with_procspawn_tempdir;
//!
//! #[with_procspawn_tempdir]
//! #[test]
//! fn my_test() -> Result<(), Box<dyn std::error::Error>> {
//!   assert!(std::path::Path::new(".procspawn-tmpdir").exists());
//!   Ok(())
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Wraps function using `procspawn` to allocate a new temporary directory,
/// make it the process' working directory, and run the function.
#[proc_macro_attribute]
pub fn with_procspawn_tempdir(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = syn::parse_macro_input!(input as syn::ItemFn);
    let fident = func.sig.ident.clone();
    let rval = func.sig.output.clone();
    let innercall = match rval {
        syn::ReturnType::Default => quote! { #fident(); },
        syn::ReturnType::Type(_, _) => quote! { #fident().map_err(|e| format!("{:#}", e))?; }
    };
    let outerrval = match rval {
        syn::ReturnType::Default => quote! { h.join().unwrap().expect("procspawn result"); },
        syn::ReturnType::Type(_, _) => quote! { h.join().unwrap().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:#}", e)))?; Ok(()) }
    };
    let fvis = func.vis.clone();
    // Remove our attribute
    func.attrs = Vec::new();
    let output = quote! {
        #fvis fn #fident() #rval {
            let h = procspawn::spawn((), |_| -> std::result::Result<(), String> {
                let tmpdir = tempfile::Builder::new().prefix("procspawn-tmpdir").tempdir().expect("procspawn tempdir");
                let path = tmpdir.path();
                std::env::set_current_dir(&path).expect("changing to tempdir");
                std::fs::write(path.join(".procspawn-tmpdir"), "").expect("writing tmpfile stamp");
                #func
                #innercall;
                Ok(())
            });
            #outerrval
        }
    };
    output.into()
}
