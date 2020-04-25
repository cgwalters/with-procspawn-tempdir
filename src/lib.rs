
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
    // Remove our attribute
    func.attrs = Vec::new();
    let output = quote! {
        fn #fident() -> std::io::Result<()> {
            let tmp_dir = tempfile::Builder::new().prefix("procspawn-tmpdir").tempdir()?;
            let h = procspawn::spawn(tmp_dir.path().to_path_buf(), |path| -> std::result::Result<(), String> {
                #func
                std::env::set_current_dir(&path).expect("changing to tempdir");
                std::fs::write(path.join(".procspawn-tmpdir"), "").expect("writing tmpfile stamp");
                #fident().map_err(|e| e.to_string())?;
                Ok(())
            });
            h.join().unwrap().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
    };
    output.into()
}
