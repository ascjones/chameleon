use color_eyre::eyre::Error;

fn main() -> Result<(), Error> {
    // todo: accept these as arguments from command line and/or stdin
    let mod_name = "node_runtime";
    let path = {
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let root_path = std::path::Path::new(&root);
        root_path.join("../core/node-runtime.scale")
    };

    let code = core::generate_runtime_types(mod_name, path)?;
    println!("{}", code.to_string());
    Ok(())
}
