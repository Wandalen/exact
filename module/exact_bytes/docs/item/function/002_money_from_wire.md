# 002: money_from_wire

## Representation

Decode a money value from its wire form.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_bytes/src/lib.rs:211-215`

```rust
pub fn money_from_wire< const S : u32 >( w : Wire ) -> Result< Decimal< S >, WireError >
{
  check_header( w, KIND_MONEY, scale_byte::< S >() )?;
  Decimal::< S >::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 211-215 | Declaration |
| `tests/wire_roundtrip_test.rs` | 17,48,59,79,89,100,111,119,138 | Decoding across 9 of the crate's 14 tests, covering every failure mode and the success path |
| `exact_arith/src/lib.rs:127` | — | Facade re-export |
| `exact_arith/tests/any_scale_test.rs:71,72` | — | Decodes money at scale 2, and refuses it at scale 6 with `BadScale` |
| `exact_arith/tests/facade_test.rs:42` | — | Decoding in the facade's own end-to-end settlement test |
| `smoke_exact_market_split/src/lib.rs:215` | — | Scene 009 decodes `10.00` back at scale 2 |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | The crate's most-tested function — exercises every `WireError` variant except `Truncated` |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs`, `tests/any_scale_test.rs` | Re-exported, and actually called in the facade's own tests — at the standard scale, and at scale 2 including the `BadScale` refusal |
| `smoke_exact_market_split` | `src/lib.rs` | Demo lane: scene 009 |

## Caller Tree

- **External:** `smoke_exact_market_split::golden` (`smoke_exact_market_split/src/lib.rs:215`) — demo-lane code, not production

No caller in **production** code. `exact_arith/tests/facade_test.rs:42` calls it, but that
is a test file in a different crate, out of scope for this tree.

## Callee Tree

- `check_header` (`src/lib.rs:107`, private — no Item Instance of its own): the kind check (`BadKind`), then the scale check against the expected scale (`BadScale`)
- `scale_byte::< S >()` (`src/lib.rs:50`, private — no Item Instance of its own): the expected scale byte, `S` itself; its inline `const` assertion refuses at compile time a scale the `u8` cast would cut
- `kind_error_to_wire_error` (`src/lib.rs:94`, private — no Item Instance of its own)
- **External:** `exact_kind::Decimal::from_minor` (`src/lib.rs:214`)
