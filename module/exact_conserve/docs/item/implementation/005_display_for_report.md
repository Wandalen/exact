# 005: Display for Report

## Representation

Renders a `Report` as either `"balanced: entries N, net 0"` or
`"UNBALANCED: entries N, net M minor units"`, branching on
`is_balanced()`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:175-188`

```rust
impl core::fmt::Display for Report
{
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
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 175-188 | Declaration |
| `tests/conservation_test.rs:73-83` | — | `the_report_renders_both_outcomes_in_words` — asserts both branches' exact text |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | **Rendered** — its own test exercises both the balanced and unbalanced branches by exact string match |
