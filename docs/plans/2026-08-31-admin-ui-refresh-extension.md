# Refreshed Admin UI Extension Implementation Plan

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** Build the refreshed admin frontend as a standalone WASM component served at `/_/admin-refresh/` and compatible with TrailBase v0.33.5.

**Architecture:** Keep the existing admin source and make only its build/router base configurable. A standalone Rust WASM component embeds the alternate build output and serves the SPA using the existing guest HTTP routing API. No TrailBase backend routes or authorization code change.

**Tech Stack:** SolidJS, Vite, Vitest, Rust, `trailbase-wasm`, `rust-embed`, Make.

---

### Task 1: Configurable admin base path

**Files:**
- Create: `crates/assets/js/admin/src/lib/admin-base.ts`
- Create: `crates/assets/js/admin/tests/admin-base.test.ts`
- Modify: `crates/assets/js/admin/src/App.tsx`
- Modify: `crates/assets/js/admin/src/components/auth/LoginPage.tsx`
- Modify: `crates/assets/js/admin/vite.config.mts`

**Step 1: Write the failing test**

Add focused assertions that `adminBasePath()` removes Vite's trailing slash, preserves `/_/admin-refresh`, and maps `/` to an empty Solid Router base.

**Step 2: Verify RED**

Run:

```bash
pnpm --dir crates/assets/js/admin exec vitest run tests/admin-base.test.ts
```

Expected: FAIL because `admin-base.ts` does not exist.

**Step 3: Implement the minimum configuration**

Add one helper:

```ts
export function adminBasePath(base = import.meta.env.BASE_URL): string {
  return base === "/" ? "" : base.replace(/\/$/, "");
}
```

Use it for Solid Router's `base` and the OTP redirect. Set Vite's base to `process.env.TRAILBASE_ADMIN_BASE ?? "/_/admin"`; keep every default build unchanged.

**Step 4: Verify GREEN**

Run the focused test and `pnpm --dir crates/assets/js/admin exec tsc --noEmit --skipLibCheck`.

### Task 2: Standalone WASM component

**Files:**
- Create: `examples/admin-ui-refresh/Cargo.toml`
- Create: `examples/admin-ui-refresh/src/lib.rs`
- Create: `examples/admin-ui-refresh/assets/.gitignore`

**Step 1: Write the failing unit tests**

In `src/lib.rs`, add tests for the wished-for route-to-asset behavior: root maps to `index.html`, known asset paths remain unchanged, and extensionless SPA paths may fall back to `index.html`.

**Step 2: Verify RED**

Run:

```bash
cargo test --manifest-path examples/admin-ui-refresh/Cargo.toml
```

Expected: FAIL because the routing helpers are absent.

**Step 3: Implement the component**

Use the existing Tetris component pattern:

- `GET /_/admin-refresh/`
- `GET /_/admin-refresh/{*wildcard}`
- `rust-embed` assets
- safe 404 responses
- correct MIME types
- immutable caching only for generated assets
- metadata name `TrailBase Admin UI Refresh`
- `admin_ui_path: "/_/admin-refresh/"`

Do not add component-side authorization; the shell must remain reachable for login and `/api/_admin/*` remains protected by TrailBase.

**Step 4: Verify GREEN**

Run the focused Rust tests.

### Task 3: Reproducible build and installation

**Files:**
- Create: `examples/admin-ui-refresh/Makefile`
- Create: `examples/admin-ui-refresh/README.md`

**Step 1: Add a build-output check**

The Makefile's `check-assets` target must fail unless generated `assets/index.html` references `/_/admin-refresh/` and does not reference built-in `/_/admin/assets` URLs.

**Step 2: Build the frontend**

Run Vite with:

```bash
TRAILBASE_ADMIN_BASE=/_/admin-refresh/ pnpm --dir crates/assets/js/admin build -- --outDir ../../../../examples/admin-ui-refresh/assets
```

**Step 3: Build the WASM artifact**

Run:

```bash
cargo build --manifest-path examples/admin-ui-refresh/Cargo.toml --target wasm32-wasip2 --release
```

Document copying the resulting component into `<traildepot>/wasm/` and restarting TrailBase.

### Task 4: Compatibility acceptance

**Files:**
- Modify only if acceptance reveals a defect; first add a failing regression test.

**Step 1: Run frontend validation**

```bash
pnpm --dir crates/assets/js/admin exec vitest run tests/admin-base.test.ts
pnpm --dir crates/assets/js/admin exec tsc --noEmit --skipLibCheck
pnpm --dir crates/assets/js/admin exec eslint src/lib/admin-base.ts src/App.tsx src/components/auth/LoginPage.tsx tests/admin-base.test.ts
```

**Step 2: Run component validation**

```bash
cargo test --manifest-path examples/admin-ui-refresh/Cargo.toml
make -C examples/admin-ui-refresh build
```

**Step 3: Test against the released binary**

Run the generated component with installed `trail v0.33.5`, then verify:

- `/_/admin-refresh/` returns the alternate index.
- A generated JS asset returns successfully.
- An extensionless SPA route falls back to the index.
- `/api/_admin/tables` remains unauthorized without credentials.
- No backend source files changed.
