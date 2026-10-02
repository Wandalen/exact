# docs

Design documentation for `exact_parse`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `algorithm/` | Per-kind parsing dispatch, the inherited grammar, and the round-trip's entry half |
| `invariant/` | The compile-time guard against `exact_kind`/`exact_scale` scale drift |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

Tier 2 of the family: parses text into the `Money`/`Quantity`/`Price` kinds `exact_kind` declares, never touching rendering — that is [`exact_fmt`'s](../../exact_fmt/docs/readme.md) concern.
