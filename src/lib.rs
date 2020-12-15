#[cfg(test)]
mod tests {
    use std::{env, fs, io::Read, path};
    use scale_info::form::CompactForm;
    use codec::Decode;

    #[test]
    fn decode_metadata() {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".into());
        let root_path = path::Path::new(&root);
        let path = root_path.join("node-runtime.scale");

        let mut file = fs::File::open(&path).expect("Error opening file");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        if &bytes[..1] == b"Ox" {
            let bytes = hex::decode(&bytes[2..]).unwrap();
        }

        let metadata =
            frame_metadata::RuntimeMetadataPrefixed::<CompactForm<String>>::decode(&mut &bytes[..]).unwrap();
    }
}
