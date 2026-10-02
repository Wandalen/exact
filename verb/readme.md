# verb

do-protocol verb scripts for the `exact` repository — one Cargo workspace, 16 members
(15 `exact_*` crates plus `smoke_exact_market_split`). Every verb here reaches the
whole family directly — no fan-out step exists or is needed.

This is the repo root's `verb/`. Every one of the 16 crates also has its own `verb/`
(`test`, `test_only`, `lint`, `build` — see e.g. [`../module/exact_kind/verb/readme.md`](../module/exact_kind/verb/readme.md)),
for iterating on one crate without touching the other 15. Those are thin wrappers —
every crate's `verb/test` is a 4-line file that `exec`s this directory's own
`_crate_dispatch` with its own crate name baked in, so the actual logic (argument
parsing, the cargo invocation shape) lives in exactly one place regardless of how
many crates call into it. `_crate_dispatch` is not itself a verb — it takes a verb
name and a crate name as its first two positional args and has no meaning invoked
on its own, which is why `verbs` (below) skips anything starting with `_`.

**Parameter convention:** every parameter is `key::val` (`level::3`, `crate::exact_kind`,
`dry::1`) — never `--flag val`. Every verb rejects an unrecognized parameter loudly
(exit 2) rather than silently ignoring or mis-forwarding it. `dry::1` (where supported)
prints the command(s) the verb would run without running them.

This set mirrors `../../ring/verb/` (a sibling crate family under the same governance),
with four of its verbs deliberately not carried over — see the table's own notes below
for why each is inapplicable to this family rather than merely unported.

No `verb.rulebook.md` exists in this repo to govern these, same as `ring`.

| File | Responsibility |
|------|-----------------|
| `test` | Leveled full-suite verification — `level::1` (nextest) through `level::5` (+doctests, clippy, udeps, audit, a full clean doc rebuild). Default `level::3`. Final verification only |
| `test_only` | Filtered nextest run — `filter::<substring>`, `crate::<name>`. Ordinary verification during development |
| `lint` | Clippy, warnings as errors. `crate::<name>` narrows to one package |
| `build` | Compile the workspace. `crate::<name>` narrows to one package |
| `doc` | Rebuild rustdoc from a clean slate (`rm -rf target/doc` first — incremental `cargo doc` hides errors in unchanged crates) |
| `clean` | Remove `target/` and this family's own `-NNNN_*` scratch logs |
| `verify` | Full pre-push gate — alias for `test level::5` |
| `verbs` | List all verbs with their purpose line |
| `package_info` | Family manifest info as flat JSON — no single crate here is "the" package (16 peer members), so this describes the family the workspace root declares |
| `_crate_dispatch` | Shared implementation behind every per-crate `verb/{test,test_only,lint,build}` wrapper. Not a verb — takes a verb name and crate name as its first two args, never invoked directly |

### Four verbs ring has that this family doesn't

- **`fmt`** — ring's applies `cargo +nightly fmt --all` against a committed `rustfmt.toml`.
  `CLAUDE.md` forbids `cargo fmt` for this project: its own custom codestyle uses own-line
  braces and spaced delimiters (`Vec< T >`, `fn foo( x : i32 )`), neither representable by
  any rustfmt option, stable or nightly — ring's own `rustfmt.toml` discloses this exact
  limitation in its header comment. Running `cargo fmt` here would silently rewrite both
  away from the house style rather than enforce it, so neither a `verb/fmt` nor a CI fmt-check
  job exists — style is hand-applied and reviewed, same as everywhere else in this family's
  code (see `CLAUDE.md`'s "Active Rejection" rule).
- **`gate`** — ring's dispatches to `bench_harness/gate/run_all.sh`, a G1-G21 corpus/quality
  suite. This family has no such harness; nothing in `docs/` is an executable gate corpus,
  only design documentation. Adding one is out of scope for this verb/CI parity work.
- **`bench`** — ring's runs `ring_bench`'s comparison example. This family has no benchmark
  crate — `docs/feature/022_bench_note_vs_f64.md` proposes one, but its own Design status
  (and [`docs/research/001_exact_vs_open_source_alternatives.md`](../docs/research/001_exact_vs_open_source_alternatives.md)'s
  Hot-Path Performance section) record that it was never built.
- **`publish_check`** — ring's dry-runs `cargo publish` for every publishable crate. Every one
  of this family's 16 manifests sets `publish = false` (confirmed via
  `grep -l "publish = false" module/*/Cargo.toml`), so there is nothing publishable to check.

### Why `test` is leveled

Mirrors the separate `will .test level::N` ladder from `CLAUDE.md` (levels 1-5, escalating
from nextest alone up to nextest+doctests+clippy+udeps+audit). Level 5 in that canonical
ladder inserts `will .test dry:0` as its own self-check — that exact invocation does not run
against this workspace (`will`'s CLI uses `::` parameter separators, not `:`, so `dry:0` is
a syntax error on this machine's `will`), and this family has no `gate`-suite substitute the
way `ring` does. `test`'s own level 5 instead runs a full clean rustdoc rebuild (`./verb/doc`)
as its maximal check — real, workspace-wide, and not already covered by levels 1-4.
