# chameleon

SCALE compatible type generation for substrate runtimes. No dependencies on substrate crates required!

Accepts a SCALE encoded metadata file via CLI or a macro, and generates a Rust module with all types required for
interacting with pallets for the given substrate runtime.

## Health warning :warning:

This is a prototype, will likely be changed significantly or even destroyed completely. So do not depend
on it for any production code (yet).

## Current status

- Can generate all the runtime modules and types for `node-runtime` with metadata from https://github.
  com/paritytech/substrate/compare/aj-metadata-vnext.

Using the command: `cargo run -p chameleon-cli | rustfmt --edition=2018 --emit=stdout`, generates the [following](./default_node_runtime_types.rs).
