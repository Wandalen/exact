# 005: Wire::kind

## Representation

The kind discriminator — one of [`KIND_MONEY`](../constant/001_kind_money.md),
[`KIND_QTY`](../constant/002_kind_qty.md),
[`KIND_PRICE`](../constant/003_kind_price.md).

Unlike its sibling accessors [`minor`](003_wire_minor.md) and
[`scale`](004_wire_scale.md) — both never called anywhere — this one IS
exercised, by this crate's own round-trip tests, asserting the discriminator
survives the encode step before the decode step is exercised separately.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:128-131`

```rust
pub const fn kind( self ) -> u8
{
  self.kind
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 128-131 | Declaration |
| `tests/wire_roundtrip_test.rs` | 15,25,35 | Asserting each kind's own round-trip preserves its discriminator |

No production function in `exact_bytes` calls `Wire::kind` — like `minor`/
`scale`, the `*_from_wire` functions read `w.kind` as a direct private-field
access (`src/lib.rs:177,205,230`) instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `tests/wire_roundtrip_test.rs` | Exercised by all 3 per-kind round-trip tests |

## Caller Tree

No caller anywhere, intra-crate or external, in production code — an honest
empty production tree. Exercised only by this crate's own tests (out of
scope for this tree per § Instance Documentation : Completeness
Verification).

## Callee Tree

No callees — returns the private `kind` field directly.
