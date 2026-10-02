# 002: Price

## Representation

A price at the standard money scale — identical to [`Money`](001_money.md)
today. A disclosed deviation from the preferred design, which lists `Price`
as an independent struct: it earns no independence yet because it has no
behaviour distinguishing it from `Money` and no real consumer whose
requirement would give that distinction content (module doc comment,
`src/lib.rs:14-25`). Kept as a plain alias rather than a hand-duplicated
wrapper, matching the Approach Gate's YAGNI check.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:62`

```rust
pub type Price = Decimal< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 62 | Declaration only |
| `exact_parse/src/lib.rs:64,66` | — | `price_from_str`'s signature and body |
| `exact_bytes/src/lib.rs:218,220,228,238` | — | `price_to_wire`/`price_from_wire` |
| `exact_add/src/lib.rs:75,85` | — | `price_add`/`price_sub` |
| `exact_fmt/src/lib.rs`, `exact_cmp/src/lib.rs` | — | Imported alongside `Money`/`Quantity` |
| `exact_snap/src/lib.rs:67,78,118` | — | `Tick::new`, `Tick::price`, `price_snap_tick` |
| `exact_ratio/src/lib.rs:165,167` | — | `price_mul_ratio` |
| `exact_arith/src/lib.rs:81` | — | Facade re-export |

No production file constructs a `Price` whose value differs in any way from
how a `Money` would be constructed — every call site above is parallel
structure to the corresponding `money_*`/`Money` path, which is the concrete
evidence behind the "identical today" claim in Representation.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; not exercised by this crate's own tests directly (its own test files use `Money`/`Quantity`, not `Price` — an honest gap, not an omission: nothing here behaves differently for `Price` than for `Money`) |
| `exact_parse`, `exact_bytes`, `exact_add`, `exact_fmt`, `exact_cmp`, `exact_snap`, `exact_ratio` | `src/lib.rs` | Third leg of the `money_*`/`qty_*`/`price_*` triple each of these crates exposes |
| `exact_arith` | `src/lib.rs` | Re-export only |
