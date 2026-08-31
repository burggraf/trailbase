# Refreshed Admin UI Extension Design

## Goal

Package the UI from `origin/feat/admin-ui-refresh` as an installable WASM component for TrailBase v0.33.5 without changing the TrailBase backend or replacing the built-in `/_/admin/` UI.

## Architecture

The existing admin frontend remains the single source of truth. Its base path becomes build-configurable while retaining `/_/admin/` as the default, so normal TrailBase builds are unchanged. The extension build sets the base to `/_/admin-refresh/` and emits static assets into a standalone component under `examples/admin-ui-refresh/`.

The component follows the existing Tetris component pattern: two GET routes serve the SPA root and wildcard assets from `rust-embed`, and metadata advertises `/_/admin-refresh/`. Extensionless paths fall back to `index.html`, allowing Solid Router deep links without backend SPA support. The browser-side TrailBase client continues calling same-origin `/api/auth/v1/*` and `/api/_admin/*`; existing backend middleware remains the only authorization boundary.

The extension is deliberately outside the root Cargo workspace so missing generated assets do not affect normal TrailBase builds. A Makefile performs the ordered frontend and WASM builds and produces one distributable `.wasm` file. No registry, alternate-UI selector, backend configuration, redirect, or dedicated `--admin-address` support is added.

## Compatibility and trust

Compatibility is targeted at TrailBase v0.33.5. The relevant admin endpoints, WASM guest interfaces, and server middleware are unchanged between v0.33.5 and this branch. Acceptance testing will run the component with the installed v0.33.5 `trail` binary.

The UI shell is public like the built-in admin shell. All privileged calls remain protected by bearer/cookie authentication, the live database admin check, and CSRF validation. Installing an alternate admin UI grants it administrator-level capability, so users must treat the component as trusted code.
