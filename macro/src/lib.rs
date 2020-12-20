use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_types(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_matches('"');

    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
    let root_path = std::path::Path::new(&root);
    let path = root_path.join(input);

    core::generate_runtime_types("runtime", path)
        .expect("Runtime generation failed")
        .into()
}
