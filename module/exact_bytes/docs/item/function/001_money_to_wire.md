# 001: money_to_wire

## Representation

Encode a money value to its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:176-179`

```rust
pub fn money_to_wire( v : Money ) -> Wire
{
  Wire { minor : v.minor(), scale : exact_scale::MONEY_SCALE as u8, kind : KIND_MONEY }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 176-179 | Declaration |
| `tests/wire_roundtrip_test.rs` | 14,43,54,65,76,109,128,146 | Encoding across 8 of the crate's 14 tests |
| `exact_arith/src/lib.rs:128` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:41` | — | Constructed in the facade's own end-to-end settlement test |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Exercised by the majority of its own test suite |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-exported, and actually called in the facade's own test — unlike `qty_to_wire`/`price_to_wire`, which the facade only re-exports without exercising |

## Caller Tree

No caller anywhere, intra-crate or external, in **production** code — an
honest empty tree. `exact_arith/tests/facade_test.rs:41` calls it, but that
is a test file in a different crate, out of scope for this tree per
§ Instance Documentation : Completeness Verification (test-only call sites
are excluded from Caller/Callee Trees even when they are the only call site
found).

## Callee Tree

- **External:** `exact_kind::Money::minor` (`v.minor()`, `src/lib.rs:178`)
