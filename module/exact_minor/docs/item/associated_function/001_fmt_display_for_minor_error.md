# 001: Display::fmt for MinorError

## Representation

Renders the one `MinorError` variant as a sentence. See
[impl Display for MinorError](../implementation/001_display_for_minor_error.md)
for the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_minor/src/lib.rs:42`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  match self
  {
    Self::Overflow { operation } => write!( f, "{operation} left the representable range" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 42-48 | Declaration |

No file anywhere in the workspace calls this method explicitly or via
`.to_string()`/format interpolation on a `MinorError`-typed value — confirmed
by grep across every `.rs` file in `module/`. `exact_minor`'s own
test suite only matches `MinorError` by equality, never renders it.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Declared here; not exercised by this crate's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `fmt::Formatter::write_fmt` — via the `write!` macro
