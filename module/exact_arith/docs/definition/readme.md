# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition reachable through this crate in one place, so a reader can find where something is actually declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item named in `src/lib.rs`'s `pub use` lists.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning leaf crate's own doc definition instead of restating it here; this crate has no `impl` blocks of its own, so there are no associated items to enumerate beyond the 93 top-level names below.

### Module Index

Every row's "Declared" column points to the leaf crate that actually
declares the item — this facade declares none of them itself (→
[Facade Re-Exports Only](../invariant/001_facade_re_exports_only.md)). "Documented
in" is `—` throughout: an individual item's own rationale lives in its
declaring crate's own docs, out of this crate's scope per the Scope block
above; this crate's own `feature/`/`invariant/` instances document the
facade's aggregate commitments, never one re-exported name's specific
behaviour.

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Backing` | type alias | `../../../exact_minor/src/lib.rs:20` | — |
| `MinorError` | enum | `../../../exact_minor/src/lib.rs:30` | — |
| `minor_checked_add` | fn | `../../../exact_minor/src/lib.rs:72` | — |
| `minor_checked_neg` | fn | `../../../exact_minor/src/lib.rs:101` | — |
| `minor_checked_sub` | fn | `../../../exact_minor/src/lib.rs:86` | — |
| `minor_is_zero` | fn | `../../../exact_minor/src/lib.rs:62` | — |
| `minor_saturating_add` | fn | `../../../exact_minor/src/lib.rs:117` | — |
| `minor_saturating_sub` | fn | `../../../exact_minor/src/lib.rs:125` | — |
| `minor_zero` | fn | `../../../exact_minor/src/lib.rs:55` | — |
| `CEILING_MINOR_UNITS` | const | `../../../exact_scale/src/lib.rs:45` | — |
| `CEILING_WHOLE_UNITS` | const | `../../../exact_scale/src/lib.rs:29` | — |
| `HEADROOM_FACTOR` | const | `../../../exact_scale/src/lib.rs:22` | — |
| `MONEY_SCALE` | const | `../../../exact_scale/src/lib.rs:37` | — |
| `pow10` | fn | `../../../exact_scale/src/lib.rs:64` | — |
| `Rounding` | enum | `../../../exact_round/src/lib.rs:27` | — |
| `RoundError` | enum | `../../../exact_round/src/lib.rs:75` | — |
| `round_div` | fn | `../../../exact_round/src/lib.rs:109` | — |
| `round_div_wide` | fn | `../../../exact_round/src/lib.rs:209` | — |
| `rounding_default` | fn | `../../../exact_round/src/lib.rs:53` | — |
| `rounding_name` | fn | `../../../exact_round/src/lib.rs:63` | — |
| `Sign` | enum | `../../../exact_sign/src/lib.rs:16` | — |
| `is_negative` | fn | `../../../exact_sign/src/lib.rs:48` | — |
| `is_zero` | fn | `../../../exact_sign/src/lib.rs:55` | — |
| `sign_neg_allowed` | fn | `../../../exact_sign/src/lib.rs:69` | — |
| `sign_of` | fn | `../../../exact_sign/src/lib.rs:30` | — |
| `Decimal` | struct | `../../../exact_kind/src/lib.rs:153` | — |
| `KindError` | enum | `../../../exact_kind/src/lib.rs:73` | — |
| `Money` | type alias | `../../../exact_kind/src/lib.rs:56` | — |
| `Price` | type alias | `../../../exact_kind/src/lib.rs:62` | — |
| `Qty` | struct | `../../../exact_kind/src/lib.rs:397` | — |
| `Quantity` | type alias | `../../../exact_kind/src/lib.rs:65` | — |
| `money_add` | fn | `../../../exact_add/src/lib.rs:35` | — |
| `money_checked_neg` | fn | `../../../exact_add/src/lib.rs:95` | — |
| `money_saturating_add` | fn | `../../../exact_add/src/lib.rs:110` | — |
| `money_sub` | fn | `../../../exact_add/src/lib.rs:45` | — |
| `price_add` | fn | `../../../exact_add/src/lib.rs:75` | — |
| `price_sub` | fn | `../../../exact_add/src/lib.rs:85` | — |
| `qty_add` | fn | `../../../exact_add/src/lib.rs:55` | — |
| `qty_saturating_add` | fn | `../../../exact_add/src/lib.rs:124` | — |
| `qty_sub` | fn | `../../../exact_add/src/lib.rs:65` | — |
| `Ratio` | struct | `../../../exact_ratio/src/lib.rs:78` | — |
| `RatioError` | enum | `../../../exact_ratio/src/lib.rs:38` | — |
| `money_div_round` | fn | `../../../exact_ratio/src/lib.rs:186` | — |
| `money_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:140` | — |
| `price_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:165` | — |
| `qty_div_round` | fn | `../../../exact_ratio/src/lib.rs:199` | — |
| `qty_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:153` | — |
| `ratio_new` | fn | `../../../exact_ratio/src/lib.rs:112` | — |
| `money_from_str` | fn | `../../../exact_parse/src/lib.rs:43` | — |
| `price_from_str` | fn | `../../../exact_parse/src/lib.rs:64` | — |
| `qty_from_str` | fn | `../../../exact_parse/src/lib.rs:53` | — |
| `FmtError` | enum | `../../../exact_fmt/src/lib.rs:31` | — |
| `fmt_into` | fn | `../../../exact_fmt/src/lib.rs:81` | — |
| `money_fmt` | fn | `../../../exact_fmt/src/lib.rs:91` | — |
| `price_fmt` | fn | `../../../exact_fmt/src/lib.rs:105` | — |
| `qty_fmt` | fn | `../../../exact_fmt/src/lib.rs:98` | — |
| `KIND_MONEY` | const | `../../../exact_bytes/src/lib.rs:29` | — |
| `KIND_PRICE` | const | `../../../exact_bytes/src/lib.rs:33` | — |
| `KIND_QTY` | const | `../../../exact_bytes/src/lib.rs:31` | — |
| `Wire` | struct | `../../../exact_bytes/src/lib.rs:87` | — |
| `WireError` | enum | `../../../exact_bytes/src/lib.rs:40` | — |
| `money_from_wire` | fn | `../../../exact_bytes/src/lib.rs:175` | — |
| `money_to_wire` | fn | `../../../exact_bytes/src/lib.rs:163` | — |
| `price_from_wire` | fn | `../../../exact_bytes/src/lib.rs:228` | — |
| `price_to_wire` | fn | `../../../exact_bytes/src/lib.rs:218` | — |
| `qty_from_wire` | fn | `../../../exact_bytes/src/lib.rs:203` | — |
| `qty_to_wire` | fn | `../../../exact_bytes/src/lib.rs:190` | — |
| `Lot` | struct | `../../../exact_snap/src/lib.rs:86` | — |
| `SnapError` | enum | `../../../exact_snap/src/lib.rs:17` | — |
| `Tick` | struct | `../../../exact_snap/src/lib.rs:58` | — |
| `price_snap_tick` | fn | `../../../exact_snap/src/lib.rs:118` | — |
| `qty_snap_lot` | fn | `../../../exact_snap/src/lib.rs:132` | — |
| `money_cmp` | fn | `../../../exact_cmp/src/lib.rs:24` | — |
| `money_eq` | fn | `../../../exact_cmp/src/lib.rs:45` | — |
| `price_cmp` | fn | `../../../exact_cmp/src/lib.rs:38` | — |
| `price_max` | fn | `../../../exact_cmp/src/lib.rs:59` | — |
| `price_min` | fn | `../../../exact_cmp/src/lib.rs:52` | — |
| `qty_cmp` | fn | `../../../exact_cmp/src/lib.rs:31` | — |
| `DustError` | enum | `../../../exact_dust/src/lib.rs:65` | — |
| `DustTo` | enum | `../../../exact_dust/src/lib.rs:52` | — |
| `money_dust_remainder` | fn | `../../../exact_dust/src/lib.rs:184` | — |
| `money_dust_split` | fn | `../../../exact_dust/src/lib.rs:151` | — |
| `money_dust_split_into` | fn | `../../../exact_dust/src/lib.rs:166` | — |
| `qty_dust_remainder` | fn | `../../../exact_dust/src/lib.rs:226` | — |
| `qty_dust_split` | fn | `../../../exact_dust/src/lib.rs:196` | — |
| `qty_dust_split_into` | fn | `../../../exact_dust/src/lib.rs:210` | — |
| `ConservationError` | enum | `../../../exact_conserve/src/lib.rs:108` | — |
| `Entry` | struct | `../../../exact_conserve/src/lib.rs:87` | — |
| `Report` | struct | `../../../exact_conserve/src/lib.rs:141` | — |
| `money_conserve_into` | fn | `../../../exact_conserve/src/lib.rs:223` | — |
| `money_sum_assert_zero` | fn | `../../../exact_conserve/src/lib.rs:245` | — |
| `qty_conserve_into` | fn | `../../../exact_conserve/src/lib.rs:234` | — |
| `qty_sum_assert_zero` | fn | `../../../exact_conserve/src/lib.rs:273` | — |
| `verify` | fn | `../../../exact_conserve/src/lib.rs:205` | — |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_arith
printf 'pub use statements in src/lib.rs: '; grep -c '^pub use' src/lib.rs
printf 'rows in Module Index:             '; grep -cE '^\| `' docs/definition/readme.md
# pub use statements in src/lib.rs: 14
# rows in Module Index:             93
```
