# Shivarthu client based on leptos

<!-- Change the auth.rs.bak file to auth.rs and give the authentication token -->

```
cargo install --locked trunk
cd shivarthu-client
trunk serve
```

## Tailwindcss

```bash

npx tailwindcss -i ./src/input.css -o ./css/output.css --watch

```

## daisyUI

```bash
npm i -D daisyui@latest
npm i -D flowbite
```

## Leptos format

### Examples

**Single file**

Format a specific file by name

`leptosfmt ./examples/counter/src/lib.rs`

**Current directory**

Format all .rs files within the current directory

`leptosfmt .`

**Directory**

Format all .rs files within the examples directory

`leptosfmt ./examples`

**Glob**

Format all .rs files ending with `_test.rs` within the examples directory

`leptosfmt ./examples/**/*_test.rs`

## Rust format check

`cargo fmt --all -- --check`

## Rust format

`cargo fmt --all`

<!-- ## rpc call to substrate
## transaction
## storage call
## ipfs file upload -->
