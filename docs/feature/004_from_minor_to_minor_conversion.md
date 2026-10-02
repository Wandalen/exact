# Feature: From Minor To Minor Conversion

### Scope

- **Purpose**: Give every kind an explicit, symmetric path into and out of the raw subunit.
- **Responsibility**: `exact_kind`'s conversion functions, re-exported by `exact_arith`.
- **In Scope**: `money_from_minor`/`money_to_minor` and the `qty`/`price` equivalents.
- **Out of Scope**: String parsing (→ `exact_parse`); byte encoding (→ `exact_bytes`).

**Design status**: implemented in [`exact_kind`](../../module/exact_kind/readme.md), re-exported by [`exact_arith`](../../module/exact_arith/docs/definition/readme.md). Deviates in surface shape: conversion is `Decimal`/`Qty`'s inherent `from_minor`/`.minor()` methods, not free functions named `money_from_minor`/`money_to_minor` per kind, and `exact_arith` re-exports the types themselves (`Decimal`, `Money`, `Price`, `Qty`, `Quantity`) rather than per-kind conversion functions — see [`exact_kind`'s own type doc](../../module/exact_kind/docs/type/001_conserved_value_type_family.md) for the real constructor/extractor surface.

### Statement

Every kind gets a matched `_from_minor`/`_to_minor` pair, and the facade (`exact_arith`) re-exports them as its one piece of unique surface beyond composition. This is the boundary crossing between the typed value and the raw integer, and it is deliberately the only such crossing.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:172` | Feature 4 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:380` | `exact_arith`: "Features: 4 (from_minor / to_minor re-export), composition" |
