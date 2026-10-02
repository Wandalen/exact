# 005: Display::fmt for Report

## Representation

Renders a `Report` as a one-line balanced/unbalanced summary.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:177-187`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  if self.is_balanced()
  {
    write!( f, "balanced: entries {}, net 0", self.entries )
  }
  else
  {
    write!( f, "UNBALANCED: entries {}, net {} minor units", self.entries, self.net_minor )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 177-187 | Declaration |
| `tests/conservation_test.rs:76,79-80` | — | Asserts both branches' exact rendered text |
| `exchange_core/tests/submission_test.rs:258` | — | `assert!( report.is_balanced(), "{report}" )` — rendered only on assertion failure |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | **Rendered** — both branches exercised by exact string match |
| `exchange_core` | `tests/submission_test.rs` | Rendered only if the assertion fails (a diagnostic path, not a guaranteed render) |

## Caller Tree

No caller anywhere outside the standard `Display`/`ToString` dispatch
mechanism itself — `exact_conserve`'s own test and `exchange_core`'s
assertion-failure message both invoke it through `{}`/`{report}` formatting,
not a direct `.fmt()` call.

## Callee Tree

- [is_balanced](003_is_balanced.md) (`src/lib.rs:179`)
