#[cfg(test)]
mod tests {
    use std::{env, fs, io::Read, path};
    use scale_info::form::CompactForm;
    use codec::Decode;

    #[test]
    fn decode_metadata() {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let root_path = path::Path::new(&root);

        /* generate with
            curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
            | jq .result \
            | cut -d '"' -f 2 \
            | xxd -r -p > ./node-runtime.scale
        */
        let path = root_path.join("node-runtime.scale");

        let mut file = fs::File::open(&path).expect("Error opening file");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        let metadata =
            frame_metadata::RuntimeMetadataPrefixed::<String>::decode(&mut &bytes[..]).unwrap();
    }
}
