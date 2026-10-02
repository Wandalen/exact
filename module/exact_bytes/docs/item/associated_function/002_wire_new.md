# 002: Wire::new

## Representation

Build a wire record directly from its three fields. Infallible: `Wire`
carries no invariant of its own to check — a mismatched kind or scale, or a
`minor` outside a kind's range, is detected by the `*_from_wire` functions
that interpret the record, not by this constructor.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:107-110`

```rust
pub const fn new( minor : i64, scale : u8, kind : u8 ) -> Self
{
  Self { minor, scale, kind }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 107-110 | Declaration |
| `tests/wire_roundtrip_test.rs` | 87,95,98 | Constructing a `Wire` directly to exercise overflow/negative decode failure modes |

No production function in `exact_bytes` calls `Wire::new` — `money_to_wire`/
`qty_to_wire`/`price_to_wire` all construct `Wire` via struct-literal syntax
(`Wire { minor: ..., scale: ..., kind: ... }`) instead, since they already
have every field in scope.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised only by its own tests, to hand-construct an out-of-range or tampered `Wire` that no real encoder would ever produce |

## Caller Tree

No caller anywhere, intra-crate or external, in production code — an honest
empty tree. Exercised only by this crate's own tests (out of scope for this
tree per § Instance Documentation : Completeness Verification).

## Callee Tree

No callees — constructs `Self` directly from its three parameters.
