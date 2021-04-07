use proc_macro2::TokenStream as TokenStream2;
use scale::Decode;
use std::{
    fs,
    io::{self, Read},
    path,
};

mod generate_runtime;
mod generate_types;

pub use generate_types::TypeGenerator;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error decoding runtime metadata")]
    Codec(#[from] scale::Error),
    #[error("Io error")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn generate_runtime_types<P>(mod_name: &str, path: P) -> Result<TokenStream2>
where
    P: AsRef<path::Path>,
{
    let mut file = fs::File::open(path).expect("Error opening file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let metadata = frame_metadata::RuntimeMetadataPrefixed::decode(&mut &bytes[..])?;

    let generator = generate_runtime::RuntimeGenerator::new(metadata);
    Ok(generator.generate_runtime(mod_name))
}

#[cfg(test)]
mod tests {
    use std::{env, path};

    #[test]
    fn generate_runtime_types() {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let root_path = path::Path::new(&root);

        // generate with:
        // curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
        // | jq .result \
        // | cut -d '"' -f 2 \
        // | xxd -r -p > ./core/node-runtime.scale
        //
        //
        let path = root_path.join("node-runtime.scale");

        super::generate_runtime_types("test_runtime", path).unwrap();
    }
}
