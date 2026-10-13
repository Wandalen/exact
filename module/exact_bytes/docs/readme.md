# docs

Design documentation for `exact_bytes`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `format/` | `Wire`'s 10-byte minor/scale/kind layout and its round-trip guarantee |
| `invariant/` | Compile-time safety of the scale byte's width, for every scale a record is written at |
| `decisions/` | Why decoding needs two more failure variants than the preferred design named, and why the scale byte is the type's own scale |
| `definition/` | Module Index — every public item in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

Tier 2 of the family: encodes and decodes the conserved value types
[`exact_kind`](../../exact_kind/readme.md) declares, checked against the
scale [`exact_scale`](../../exact_scale/readme.md) declares.
