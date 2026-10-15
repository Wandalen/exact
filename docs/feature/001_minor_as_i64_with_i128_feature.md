# Feature: Minor As I64 With I128 Feature

### Scope

- **Purpose**: Define the raw subunit's storage width, so every other proposed crate shares one representation.
- **Responsibility**: `exact_minor`'s primary type.
- **In Scope**: The default `i64` width and the optional `i128` widening.
- **Out of Scope**: Scale and kind, which are separate proposed crates (`exact_scale`, `exact_kind`).

**Design status**: implemented as specified in [`exact_minor`](../../module/exact_minor/readme.md) — `Minor` wraps an `i64` by default, and `MinorWide` wraps an `i128` behind the crate's `i128` feature (`cargo build --features i128`), not a separate crate.

### Statement

The raw subunit is `i64` by default, with `i128` available only behind a feature flag on `exact_minor` — never a separate crate. This keeps the common case cheap while leaving a documented escape hatch for callers whose magnitudes outgrow 64 bits.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:166` | Feature 1 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:384` | "i128 is a feature flag on exact_minor, not a crate" |
