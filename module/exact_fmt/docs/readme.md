# docs

Design documentation for `exact_fmt`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `algorithm/` | Per-kind rendering dispatch, the allocation-free buffer-writing primitive, and the round-trip's exit half |
| `invariant/` | Bounds-safety of the allocation-free buffer-writing primitive |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

Tier 2 of the family: renders the `Money`/`Quantity`/`Price` kinds `exact_kind` declares back to text, never touching parsing — that is [`exact_parse`'s](../../exact_parse/docs/readme.md) concern.
