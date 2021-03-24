# chameleon

## Health warning: this is a WIP prototype to prove the concept.

SCALE compatible type generation for substrate runtimes. No dependencies on substrate crates required!

Accepts a SCALE encoded metadata file via a CLI or a macro, and generates a Rust module with all types required for interacting with pallets for the given substrate runtime.

## Current status

- Generated code not compiling yet, need to handle generics
- Only System and Balances module converted to the new metadata in https://github.com/paritytech/substrate/compare/gui-macro-attribute...aj-metadata-vnext

Using the command: `cargo run -p chameleon-cli | rustfmt --edition=2018 --emit=stdout`, generates the [following](.
/default_node_runtime_types.rs).
