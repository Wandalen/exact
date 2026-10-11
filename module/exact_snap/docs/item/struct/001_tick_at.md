# 001: Tick

## Representation

A price grid's spacing — the smallest meaningful price increment a snapped
value may land on. A single-field tuple struct wrapping `Price`, kept
distinct from a bare `Price` so a caller cannot pass an arbitrary price where
a validated, non-zero grid spacing is required.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_snap/src/lib.rs:72-74`

```rust
/// A price grid's spacing — the smallest meaningful price increment.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Tick( Price );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 74 | Declaration |
| `src/lib.rs` | 76-104 | `impl Tick` (constructor and accessor) |
| `src/lib.rs` | 141,152 | `tick` parameter type and field access in `price_snap_tick` |
| `tests/snap_test.rs` | 7,13,22,34,43,52 | Constructed via `Tick::new` and passed into `price_snap_tick` |
| `exact_arith/src/lib.rs:135` | — | Facade re-export |

The derived traits (`Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
Hash`) are not hand-written `impl` blocks and are not cataloged as separate
Implementation instances, per established precedent (`exact_kind`/`exact_ratio`
only catalog their types' hand-written impls).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `src/lib.rs` | The grid-spacing type `price_snap_tick` requires; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not constructed by the facade's own test suite |
