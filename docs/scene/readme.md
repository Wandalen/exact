# Scene Doc Definition

### Scope

- **Purpose**: Document the proposed `smoke_exact_market_split` golden-path demo step by step, so the target smoke lane's coverage is traceable to its source conversation.
- **Responsibility**: The 10 scene steps of the proposed smoke lane and their golden-print pass criteria.
- **In Scope**: Each step's operation, expected result, and which hard problem/feature it exercises.
- **Out of Scope**: The old, differently-scoped `smoke_exact_arithmetic` lane this directory originally sat beside — deleted at the migration's cutover and no longer on disk, retrievable only via `git show HEAD:smoke_exact_arithmetic/<path>`.

**Design status**: implemented — [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md) is the real demo lane for this family. Its step 6 runs all 10 steps below in their golden-print shape (`golden`, `checksum`), after five steps of its own (a parse/render round trip, an exact-vs-`f64` summation control arm, a `Quantity` below-zero refusal, a ledger conservation audit, and a 3-way market split with dust folded to the first share). Two steps print a different number than proposed, because `exact_dust` and `exact_bytes` take `Money`, fixed at scale 6, rather than a scale-2 value: 003 prints `parts=3.333333,3.333333,3.333333 dust=0.000001` and 009 prints `wire=10000000`. Step 007 has no runtime line, by its own design. See each instance below for the per-step account.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Parse Money At Scale Two](001_parse_money_at_scale_two.md) | Parse "10.00" and "3.33" as `Money` at scale 2 | 🔄 |
| 002 | [Add Subtract Assert Sum Zero](002_add_subtract_assert_sum_zero.md) | Add the two, subtract "13.33", assert the sum is zero | 🔄 |
| 003 | [Split With Dust To Sink](003_split_with_dust_to_sink.md) | Split 10.00 into 3 parts rounding down, dust to sink | 🔄 |
| 004 | [Price Snap Tick](004_price_snap_tick.md) | `price_snap_tick` of "1.26" to tick "0.05" → "1.25" | 🔄 |
| 005 | [Qty Snap Lot](005_qty_snap_lot.md) | `qty_snap_lot` of 10 to lot 3 → 9 | 🔄 |
| 006 | [Reject Extra Digits Parse](006_reject_extra_digits_parse.md) | Reject "1.234" at scale 2 (`ExtraDigits`) | 🔄 |
| 007 | [Reject Money Plus Qty](007_reject_money_plus_qty.md) | Kind mismatch has no function to call — only money+money compiles | 🔄 |
| 008 | [Checked Add Overflow At Max](008_checked_add_overflow_at_max.md) | Checked add at `i64::MAX` → `Overflow`, not wrap | 🔄 |
| 009 | [Wire Round Trip](009_wire_round_trip.md) | Wire round-trip of "10.00" equals the original minor | 🔄 |
| 010 | [Checksum Equality Across Calls](010_checksum_equality_across_calls.md) | Two calls produce the same checksum of minors | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/scene
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              10
# rows in Overview Table: 10
```
