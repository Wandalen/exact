# Research: `exact` vs. Open-Source Alternatives

### Scope

- **Purpose**: Answer, with evidence, whether an existing open-source crate could have replaced this family — or a part of it — instead of building from scratch.
- **Responsibility**: A dated, sourced comparison of `exact`'s 16 crates against the closest Rust-ecosystem alternatives, checked against the requirements this family's own docs actually state.
- **In Scope**: Decimal/fixed-point crates, currency-safe money crates, in-process double-entry/conservation crates, and order-book/exchange crates on crates.io, as surveyed 2026-10-01 to 2026-10-02.
- **Out of Scope**: Non-Rust ecosystems; this family's own implementation detail (→ each crate's own `docs/`); the original 15-crate proposal's own rationale (→ [`../crate/`](../crate/readme.md), [`../hard_problem/`](../hard_problem/readme.md)).

**Design status**: first research pass, dated 2026-10-01; extended 2026-10-02 to cover cross-scale compile-time safety ([HP 013](../hard_problem/013_scale_mismatch.md)) and hot-path performance ([HP 012](../hard_problem/012_hot_path_performance.md)), the two remaining hard problems the first pass didn't compare against. Findings are sourced to crates.io/docs.rs/GitHub pages fetched on each date noted — re-verify before relying on a specific version number or feature-flag claim, since these crates evolve.

## Question

Workstream 006 built 16 small, zero-external-dependency crates for exact money/quantity arithmetic rather than depending on an existing crate. Was there an existing crate (or combination) that already solved this, such that building from scratch duplicated it?

## Candidates Surveyed

| Crate | Category | Link |
|---|---|---|
| `rust_decimal` | Fixed-size (128-bit) decimal, `Copy` | [GitHub](https://github.com/paupino/rust-decimal) · [docs.rs](https://docs.rs/rust_decimal/latest/rust_decimal/) |
| `bigdecimal` (bigdecimal-rs) | Arbitrary-precision decimal, heap-backed | [GitHub](https://github.com/akubera/bigdecimal-rs) · [docs.rs](https://docs.rs/bigdecimal) |
| `fastnum` | Fixed-size decimal, pure Rust, `Copy`, widths beyond 128-bit | [GitHub](https://github.com/neogenie/fastnum) · [crates.io](https://crates.io/crates/fastnum) |
| `primitive_fixed_point_decimal` | Const-generic-scale fixed-point decimal, `Copy`, no_std+no_alloc | [docs.rs](https://docs.rs/primitive_fixed_point_decimal) |
| `fixed` | Binary (not decimal) fixed-point, `Copy`, no_std | [docs.rs](https://docs.rs/fixed/latest/fixed/) |
| `typed-money` / `moneta` / `use-money` | Currency-safe `Money` newtypes | [typed-money](https://docs.rs/typed-money/latest/typed_money/) · [moneta](https://docs.rs/moneta) · [use-money](https://docs.rs/use-money/latest/use_money/) |
| `doubleentry` | In-process double-entry bookkeeping engine | [docs.rs](https://docs.rs/doubleentry/latest/doubleentry/) |
| `orderbook-rs` | Order-book engine with tick/lot-size order validation | [docs.rs](https://docs.rs/orderbook-rs) |
| `num-rational` (`Ratio<T>`) | Exact rational arithmetic | [docs.rs](https://docs.rs/num-rational/latest/num_rational/) |

`orderbook-rs` and `num-rational` are not scored in the requirements table below — neither is a drop-in alternative for the whole family the way the decimal/fixed-point crates are. Each is addressed in its own note instead: [orderbook-rs within Tick/Lot Grid-Snapping Policy](#requirement-ticklot-grid-snapping-policy), [num-rational in its own note](#note-num-rational-and-exact_ratio).

## Comparison Against This Family's Actual Requirements

Requirements this family's own docs state — seven as numbered hard problems ([`../hard_problem/`](../hard_problem/readme.md)), two from the Workstream Charter and this family's own observed convention ([`../readme.md`](../readme.md), [`../readme.md`](../readme.md) § Workstream Charter):

| Requirement | Source | rust_decimal | bigdecimal | fastnum | primitive_fixed_point_decimal | fixed | money newtypes | doubleentry |
|---|---|---|---|---|---|---|---|---|
| No binary floats, exact decimal fractions | [HP 001](../hard_problem/001_float_money_is_wrong.md) | ✅ | ✅ | ✅ | ✅ | ❌ binary base — own docs: "0.001 cannot be represented exactly" | depends on backing type | ✅ |
| Money/Qty/Price as distinct, non-interchangeable types | [HP 003](../hard_problem/003_distinct_kinds.md) | ❌ one generic `Decimal` | ❌ one generic `BigDecimal` | ❌ one generic type | ❌ one generic `ConstScaleFpdec<Repr,N>` | ❌ one generic type | distinguishes **currency**, not Money-vs-Qty-vs-Price | ❌ one `Amount<P>` |
| Cross-scale operations fail to compile, not resolve silently at runtime | [HP 013](../hard_problem/013_scale_mismatch.md) | ❌ scale is a runtime field; auto-rescales | ❌ scale is a runtime field; auto-rescales | ❌ runtime 16-bit exponent; auto-aligns | ✅ `+`/`-` across different `N` is a compile error | ✅ distinct bit-width types (e.g. `I16F16` vs `I8F24`) | ❌ typed by currency, not by scale | ✅ `Amount<P>` — compile-time precision |
| Plain `Copy` bits, no heap, VM-snapshot-safe | [HP 010](../hard_problem/010_closed_vm_types.md) | ✅ | ❌ `Vec`-backed, not `Copy` | ✅ | ✅ | ✅ | varies by crate | ✅ `Amount<P>` is an exact int |
| Built-in zero-sum conservation audit | Workstream Charter + features, not a hard problem | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | 🟡 closest analog — see below |
| Explicit dust-destination policy | [HP 006](../hard_problem/006_dust_destination.md) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Tick/lot grid-snapping policy | [HP 011](../hard_problem/011_tick_and_lot_snap.md) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| Hot match-loop ops (add/compare/ratio) cheap, no heap alloc | [HP 012](../hard_problem/012_hot_path_performance.md) | 🟡 mid-pack, 3rd of 4 benchmarked | ❌ slowest of 4 benchmarked | 🟡 2nd-slowest — pays a scale-alignment check per op | ✅ fastest of 4 benchmarked — no scale-alignment needed; measured directly against `exact` 2026-10-02 — comparable on add/compare, `exact` 2x faster on ratio-multiply | ⚠️ not benchmarked against the others | ⚠️ not benchmarked | ⚠️ not benchmarked |
| Zero external (crates.io) dependencies | Observed convention — not in hard_problem or feature | n/a (would be the dependency) | n/a | n/a | n/a | n/a | n/a | n/a |

`⚠️` = no data found for this candidate on this requirement in the sources checked — see that requirement's own section for what was and wasn't found.

## Requirement: No Binary Floats, Exact Decimal Fractions

**Represented in**:
- Hard problem — `../hard_problem/001_float_money_is_wrong.md`
- No single dedicated feature — the closest is `../feature/020_reject_non_finite_extra_digits.md` (parse-boundary reinforcement: rejects malformed/over-precise text so float-style parsing never re-enters downstream, but not the core guarantee itself)
- Actual enforcement mechanism (most precise citation) — `../../module/exact_minor/docs/invariant/001_no_float_in_representation.md` and `../../module/exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md`

**Explanation**: solved territory. `rust_decimal`, `bigdecimal`, `fastnum`, and `primitive_fixed_point_decimal` all avoid floats and represent decimal fractions exactly; this is not a problem unique to `exact`. The `fixed` crate is the one disqualified outright — it is *binary* fixed-point, and its own docs state plainly it cannot represent `0.001` exactly, the same class of error as `f64` for this family's purposes.

## Requirement: Money/Qty/Price as Distinct, Non-Interchangeable Types

**Represented in**:
- Hard problem — `../hard_problem/003_distinct_kinds.md`
- Feature — `../feature/003_newtypes_money_qty_price.md`

**Explanation**: unsolved by every decimal crate surveyed, because it isn't a decimal-representation problem; it's a newtype-wrapping problem layered on top of one. Every decimal crate ships exactly one generic numeric type and leaves domain-specific wrapping to the caller. The money-specific crates (`typed-money`, `moneta`, `use-money`) do wrap a decimal in a distinct type, but the axis they distinguish is **currency** (USD vs. EUR), not **domain role** (Money vs. Quantity vs. Price) — a currency-safe `Money` type still lets a quantity and a price of the same currency collide, which is exactly what the hard problem above rules out.

## Requirement: Cross-Scale Operations Fail to Compile

**Represented in**:
- Hard problem — `../hard_problem/013_scale_mismatch.md`
- No feature — this hard problem's own Design status line states it is "avoided by construction rather than solved by a checked operation," so the proposed `scale_convert`/`ScaleError` surface (feature 019) was never built

**Explanation**: the one requirement where candidates split sharply by representation strategy, checked 2026-10-02. `rust_decimal` stores scale as a runtime field inside its 128-bit struct and silently rescales mismatched operands during arithmetic (confirmed via its own docs.rs page). `fastnum` does the same with a 16-bit exponent carried in each value's own control block — its own addition docs describe operands being aligned by comparing exponents, never rejected. `primitive_fixed_point_decimal`'s `ConstScaleFpdec<Repr, N>` is the opposite: its own docs state `+`/`-` between different `N` values "only perform between same types in same scale... there is no implicitly type or scale conversion," explicitly "for we do not want to add `Balance` type by `Price` type" — independently landing on the same justification the Distinct Kinds requirement above gives for this family's own kind-distinctness. `fixed`'s binary types carry the same property structurally (`I16F16` and `I8F24` are distinct Rust types; mixing needs an explicit conversion). `doubleentry`'s `Amount<P>`, already noted above as compile-time-precision, satisfies this too. **This family's own resolution uses the identical mechanism**: `exact_kind` makes `SCALE` a `const` generic on `Decimal<SCALE>`/`Qty<SCALE>`, so mismatched scales are incompatible Rust types rather than a runtime condition to check — the same const-generic-scale technique `primitive_fixed_point_decimal` already ships.

## Requirement: Plain `Copy` Bits, No Heap, VM-Snapshot-Safe

**Represented in**:
- Hard problem — `../hard_problem/010_closed_vm_types.md`
- No feature — this hard problem's own Design status line states it is "satisfied by construction... no dedicated crate or decision was needed," so no feature instance exists for it; confirmed independently by a broad `grep` for `copy`/`heap`/`snapshot`/`relocat` across all 22 `feature/*.md` files, which returns no match for this topic

**Explanation**: solved territory, for three of the five fixed-size candidates. `rust_decimal` is a 128-bit `Copy` struct (96-bit mantissa plus sign/scale); `fastnum` and `primitive_fixed_point_decimal` are explicitly stack-only, no-heap, no_std-capable. Only `bigdecimal` fails this, by design — its `Vec<u32>`/`Vec<u64>`-backed mantissa is the price of true arbitrary precision. **This means `exact_minor` + `exact_kind`'s own hand-rolled `Decimal<SCALE>` is not solving a problem existing crates couldn't — `primitive_fixed_point_decimal`'s `ConstScaleFpdec<Repr, SCALE>` is structurally the closest published analog to it.**

## Requirement: Built-In Zero-Sum Conservation Audit

**Represented in**:
- No hard problem — this is not one of the 14 numbered hard problems; it comes from the Workstream Charter, which "predates, and sits above" the hard_problem/feature breakdowns (`../readme.md` § Workstream Charter)
- Feature — `../feature/013_sum_assert_zero.md` and `../feature/014_conservation_error.md`

**Explanation**: `doubleentry` is the one real analog found, and it is a strong one: in-process, no I/O by design ("a calculation library, not a platform"), exact-integer `Amount<P>` with compile-time precision, and type-state validation that an entry cannot reach storage unbalanced. But it brings a materially heavier abstraction than `exact_conserve` needs — hierarchical accounts, a `Journal`, Merkle-log proofs, period seals — built for persistent bookkeeping across time, not for verifying that one in-memory slice of match-loop legs sums to zero before the function returns. Adopting it would mean either using a small fraction of a much larger library, or shaping this family's matching engine around `doubleentry`'s account/journal model instead of the other way around.

## Requirement: Dust-Destination Policy

**Represented in**:
- Hard problem — `../hard_problem/006_dust_destination.md`
- Feature — `../feature/010_remainder_assign_dust_destination.md`

**Explanation**: not found anywhere in the survey, and for a structural reason rather than an oversight. Every decimal/fixed-point candidate here — `rust_decimal`, `bigdecimal`, `fastnum`, `primitive_fixed_point_decimal` — provides checked and rounding-mode-aware division; that part of the problem (compute a quotient under a named rounding rule) is already solved ground, same as plain decimal representation. What none of them provide is a *destination* for what the rounding mode doesn't consume: splitting 10 units three ways under any rounding mode still leaves one subunit unaccounted for, and a generic numeric type has no way to know whether that subunit belongs to the first leg, the last leg, or the house — that is a policy choice about *this exchange's* matching rules, not a property of division. `orderbook-rs` (surveyed for the next requirement) is the closest adjacent domain and still doesn't surface this: its validation model rejects malformed quantities rather than splitting them. The gap is real, not a search miss — this is the one requirement a correctly-chosen decimal crate would have left exactly as unsolved as it was before adopting it.

## Requirement: Tick/Lot Grid-Snapping Policy

**Represented in**:
- Hard problem — `../hard_problem/011_tick_and_lot_snap.md`
- Feature — `../feature/017_snap_tick_snap_lot.md`

**Explanation**: absent from every *decimal-type* crate surveyed — tick/lot grids are exchange-specific market-structure policy, not something a general-purpose numeric type has any reason to know about. But a closer relative exists outside that category: `orderbook-rs`, an order-book *engine*, does know about tick and lot size — its own docs describe `UpdateQuantity` as "validate-first (projected tick / lot / min-max / representability / risk before touching the level)" and orders that violate the configured tick/lot size receive a typed `InvalidTickSize`/`InvalidLotSize` error. That is a materially different policy from `exact_snap`'s: `orderbook-rs` **rejects** an order that doesn't already land on the grid, where `exact_snap`'s `Tick`/`Lot` **round** a price or quantity onto the nearest valid grid point. Reject-vs-snap is a real design fork, not a wording difference — a matching engine built on reject-only validation pushes the rounding decision back onto the order's own sender, while one built on snap absorbs it internally. `orderbook-rs` also represents price/quantity as raw `u128`/`u64` integers rather than a reusable decimal type, so even where it matches this family's *policy awareness*, it isn't a library this family's own types could depend on — adopting it would mean adopting its whole order-book engine, the same bundling problem found under [Built-In Zero-Sum Conservation Audit](#requirement-built-in-zero-sum-conservation-audit).

## Requirement: Hot-Path Performance

**Represented in**:
- Hard problem — `../hard_problem/012_hot_path_performance.md`
- Feature — `../feature/022_bench_note_vs_f64.md`, built 2026-10-02 at `../../module/exact_arith/tests/bench_vs_f64.rs`

**Explanation**: this family's own half of the comparison is no longer missing. The hard problem's own Design status records that `exact_add`/`exact_ratio`/`exact_cmp` are thin integer functions with no heap allocation by construction; that claim is now measured, not merely argued. `exact_arith/tests/bench_vs_f64.rs` times add/compare/ratio over 10,000,000 iterations each, directly against both `f64` and `primitive_fixed_point_decimal` (the Tier-0/1 analog identified above), re-measured on an Apple M5, 2026-10-06 (three runs), after `money_mul_ratio` moved from a truncating `/` to `exact_round::round_div_wide` — an `i128` division that rounds per the caller's mode:

| Operation | `exact` | `f64` | vs `f64` | `primitive_fixed_point_decimal` | vs pfpd |
|---|---|---|---|---|---|
| add | 10.3-10.7ns | 6.1-6.2ns | 1.7x slower | 7.0ns | 1.5x slower |
| compare | 6.4-6.6ns | 6.9-7.0ns | 0.9x (faster) | 6.6ns | 1.0x (even) |
| ratio-multiply | 27.2-27.6ns | 6.0ns | 4.6x slower | 32.0-32.2ns | 0.9x (slightly faster) |

The comparison against `f64` lands where expected: a small, bounded constant-factor overhead (1.7-4.6x) for checked, kind-safe, conservation-auditable arithmetic — tens of nanoseconds or less, tens of millions of ops/sec even in the slowest case, comfortably inside "cheap enough for matching." Against `primitive_fixed_point_decimal`, add and compare are close (both pay a similar checked-overflow cost), and ratio-multiply is now slightly faster rather than the 2x lead the first measurement (2026-10-02, a different machine, while the multiply still truncated) showed: rounding per the caller's mode in `i128` costs most of that lead. The closest published analog is still not a performance upgrade over this family's own specialized arithmetic, but the margin on ratio-multiply is now small.

What *also* exists is third-party relative benchmarking between the other candidates, for the operations this family's own bench doesn't reach. [A published comparison](https://wubingzheng.github.io/en/Decimal-Crates-Comparison.html) ranks `bigdecimal` as slowest, `fastnum` next, then `rust_decimal`, with `primitive_fixed_point_decimal` fastest of the four. [A separate write-up](https://crustyengineer.com/blog/semantic-types-for-money-in-rust-with-fastnum/) explains *why*: `fastnum` stores its scale as a runtime 16-bit exponent, so "it must first check whether the scales are equal before addition. This check itself is relatively expensive," while fixed-point types like `primitive_fixed_point_decimal` "don't require this scale alignment step and therefore perform addition operations faster." That is the exact same split already found under [Cross-Scale Operations Fail to Compile](#requirement-cross-scale-operations-fail-to-compile) — runtime-scale types pay at every operation for a check that a const-generic-scale type resolves once, at compile time, for free. `fixed`, the money newtypes, and `doubleentry` have no published numbers against this same baseline, and this family's own bench doesn't reach them either, so they stay genuinely unscored rather than guessed at.

Re-run via `cargo test -p exact_arith --test bench_vs_f64 -- --nocapture`; treat the ratios between operations as the stable finding and the absolute nanosecond figures as this machine's own snapshot, not a portable constant.

## Requirement: Zero External (crates.io) Dependencies

**Represented in**:
- No hard problem, no feature — neither collection mentions dependency policy at all, confirmed by a broad `grep` for `depend` across every `hard_problem/*.md` and `feature/*.md` file (the only hits were unrelated: a determinism cross-reference and a platform-dependent-JSON note)
- Verified instead directly against the 16 crates' own manifests: `grep` across every `Cargo.toml` in `../../module/` turns up zero `[dependencies]` entries outside this family's own `exact_*`/`smoke_*` crates; the only external `[dev-dependencies]` are two test-only crates in `exact_arith` — `primitive_fixed_point_decimal` for the bench's comparison and `assert_no_alloc` for the allocation test's counting allocator — neither of which reaches a consumer

**Explanation**: this is a convention this family follows in practice, not a requirement sourced from the original 15-crate proposal's own hard_problem/feature breakdown, and not a quoted line from the charter either — the charter's "Depends on: nothing in the catalog" is about not depending on another *workstream*, not a crates.io dependency ban. It is nonetheless a real, observed constraint this comparison has to account for, since every candidate surveyed above is an external dependency by definition.

## Note: `num-rational` and `exact_ratio`

Not a row in the table above — `num-rational`'s `Ratio<T>` doesn't compete for the family's storage type (`Money`/`Qty`/`Price`), it competes for one specific crate's own approach: `exact_ratio`'s rational multiplier.

`Ratio<T>` represents a value as an exact `numerator/denominator` pair over any integer `T`, reduced to lowest terms — genuinely exact, with no decimal scale at all, which sidesteps the Cross-Scale requirement entirely rather than satisfying it (two ratios combine by cross-multiplication; there is no "scale" to mismatch). It is `Copy` when `T` is `Copy`, so it clears the VM-snapshot requirement the same way the fixed-size decimal candidates do. What it doesn't give this family is the specific operation `exact_ratio` actually needs: a rational multiplier applied *to* a `Decimal<SCALE>`/`Qty<SCALE>` and rounded back down into that same fixed scale under an explicit mode (`exact_round`'s `Down`/`Up`/`HalfEven`). `Ratio<T>` has no opinion about decimal scale because it was never designed to produce one — reducing a `Ratio` back to a fixed-scale decimal is exactly the rounding-mode-aware division `exact_ratio` and `exact_dust` implement, and `num-rational` stops one step before that, at the exact fraction itself. Adopting it would have replaced `exact_ratio`'s *multiplier representation* only, not its *division-with-rounding* responsibility — a smaller, more surgical substitution than any candidate discussed above, but still a partial one.

## Conclusion

No single existing crate — and no realistic combination of them — covers this family's actual requirement set. But the honest reading is narrower than "nothing out there helps":

- The **boring majority** of the problem (exact decimal storage, no floats, `Copy`, no-heap) is genuinely solved ground. `primitive_fixed_point_decimal`'s `ConstScaleFpdec<Repr, SCALE>` is close enough to `exact_kind`'s own `Decimal<SCALE>` that adopting it as the Tier-0/1 foundation — then layering this family's own `Money`/`Qty`/`Price` newtypes, conservation audit, and dust/snap policy on top of it — was a real, viable alternative path, not a strawman. The resemblance goes beyond storage shape: both use the identical const-generic-scale mechanism to make cross-scale mixing a compile error rather than a runtime check (see Cross-Scale Operations Fail to Compile above).
- **Hot-path performance now has a first-party answer.** `exact_arith/tests/bench_vs_f64.rs`, built 2026-10-02, measures add/compare/ratio directly: 1.7-4.6x `f64`'s cost (re-measured 2026-10-06; tens of nanoseconds or less — comfortably cheap enough for matching), and against `primitive_fixed_point_decimal` specifically, comparable on add/compare and slightly faster on ratio-multiply, where this family's hand-specialized `exact_ratio` still edges out the generic cross-scale `Mul` the closest published analog ships (see Hot-Path Performance above). Third-party relative benchmarks among the other candidates still favor the same const-generic-scale shape this family converged on independently, for the operations this family's own bench doesn't reach.
- The **genuinely novel part** — kind-distinction as a first-class constraint, a conservation audit scoped to a single match-loop slice rather than a persistent ledger, and an explicit dust-remainder destination — has no existing crate to adopt, on or off this list. Tick/lot grid-snapping comes closest to an exception: `orderbook-rs` already encodes the concept, but as reject-on-violation rather than snap-to-grid — a different policy wearing a similar name (see Tick/Lot Grid-Snapping Policy above).
- `exact_ratio` specifically has a narrower near-miss of its own: `num-rational`'s `Ratio<T>` gives an exact rational representation but stops one step short of rounding one back into this family's fixed scale (see the `num-rational` note above).
- The zero-external-dependency convention this family actually follows was a choice, not a forced hand — nothing in the charter prohibits a crates.io dependency for the Tier-0/1 layer specifically.

Building all 16 crates from scratch therefore duplicated some already-solved ground (the Tier-0/1 decimal representation) in exchange for a uniform, dependency-free, single-author surface across both the solved and unsolved parts. That is a defensible trade, not a free one — it is not the case that no existing crate could help.

## Sources

- [rust_decimal — GitHub](https://github.com/paupino/rust-decimal)
- [rust_decimal — docs.rs](https://docs.rs/rust_decimal/latest/rust_decimal/)
- [bigdecimal-rs — GitHub](https://github.com/akubera/bigdecimal-rs)
- [bigdecimal — docs.rs](https://docs.rs/bigdecimal)
- [fastnum — GitHub](https://github.com/neogenie/fastnum)
- [fastnum — crates.io](https://crates.io/crates/fastnum)
- [primitive_fixed_point_decimal — docs.rs](https://docs.rs/primitive_fixed_point_decimal)
- [fixed — docs.rs](https://docs.rs/fixed/latest/fixed/)
- [typed-money — docs.rs](https://docs.rs/typed-money/latest/typed_money/)
- [moneta — docs.rs](https://docs.rs/moneta)
- [use-money — docs.rs](https://docs.rs/use-money/latest/use_money/)
- [doubleentry — docs.rs](https://docs.rs/doubleentry/latest/doubleentry/)
- [orderbook-rs — docs.rs](https://docs.rs/orderbook-rs)
- [num-rational — docs.rs](https://docs.rs/num-rational/latest/num_rational/)
- [Comparison and Benchmarking of Rust Decimal Crates](https://wubingzheng.github.io/en/Decimal-Crates-Comparison.html)
- [Semantic Types for Money in Rust, with Better Precision and Fixed-point Decimal Arithmetic](https://crustyengineer.com/blog/semantic-types-for-money-in-rust-with-fastnum/)
- [fastnum — docs.rs](https://docs.rs/fastnum/latest/fastnum/)
