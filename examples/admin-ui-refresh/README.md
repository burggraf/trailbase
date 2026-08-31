# Admin UI refresh

This example packages the admin frontend as a WASM component served at
`/_/admin-refresh/`. It targets TrailBase `v0.33.5`.

## Prerequisites

- `pnpm`
- Rust and Cargo with the `wasm32-wasip2` target installed
- A TrailBase checkout with this repository available at its expected relative paths

Build the frontend and WASM component from the repository root:

```sh
make -C examples/admin-ui-refresh build
```

The resulting component is:

```text
examples/admin-ui-refresh/target/wasm32-wasip2/release/trailbase_admin_ui_refresh_component.wasm
```

Copy it into `<traildepot>/wasm/`, restart TrailBase, and open
`/_/admin-refresh/`.

This example has the normal single-address limitation: it is intended to be
served by one configured TrailBase address. Only use WASM components you trust;
a component can handle requests and access the capabilities granted by the
server.

The built-in admin UI at `/_/admin/` is unchanged.
