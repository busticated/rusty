# AGENTS.md

Instructions for AI coding agents working on this repository. **Read [README.md](README.md) first** — it covers setup, conventions, and the full command list, and takes precedence if this document conflicts with it.

## Hard rules — require explicit approval

- **Never** run `cargo add`. Ask; the user must say yes explicitly. Do not infer approval from a request to implement a feature that needs a dependency.
- **Never** `git commit`. Make changes locally, then prompt the user to review and commit.
- **Never** `git push`. Prompt the user, and explain why the push is needed.
- **Never** publish a crate. See "[How to publish crates](README.md#develop-publish-crate)".

## Commands

All local dev commands are `xtask` scripts — run `cargo xtask help` for the full list.

- `cargo xtask test` — fast inner loop while iterating.
- `cargo xtask ci` — run before prompting the user to review. Chains `format --check`, `spellcheck`, `lint` (clippy), and `coverage` (which runs the tests). Coverage is known to be flaky, so don't reach for it mid-iteration. Report results honestly.
- `cargo xtask crate:add` — the only supported way to add a crate. Never hand-create one under `crates/`.

## Conventions

- **Toolchain is pinned** to Rust 1.86 (`rust-toolchain.toml`, `Cargo.toml`). CI verifies this MSRV — don't use newer language or std features.
- **Commit messages** are `[<crate name>] <message>` (e.g. `[node-js-release-info] update docs`) and crate changes are staged separately from workspace changes. This drives automated changelogs. Workspace-level changes (including `xtask`) get no prefix.
- **Don't hand-edit** the crate list in `README.md` — it's generated between the `<!-- crate-list-start -->` / `<!-- crate-list-end -->` markers.
- **Document public interfaces** with inline rustdoc annotations.
- **TODO comments** are formatted `// TODO (<name>): <message>`.
