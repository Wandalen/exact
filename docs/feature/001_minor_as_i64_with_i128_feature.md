# Feature: Minor As I64 With I128 Feature

### Scope

- **Purpose**: Define the raw subunit's storage width, so every other proposed crate shares one representation.
- **Responsibility**: `exact_minor`'s primary type.
- **In Scope**: The default `i64` width and the optional `i128` widening.
- **Out of Scope**: Scale and kind, which are separate proposed crates (`exact_scale`, `exact_kind`).

**Design status**: implemented in [`exact_minor`](../../module/exact_minor/readme.md) for the `i64` default — the backing width this feature names is exactly `exact_minor::Backing = i64`. The `i128` half does not exist: no feature flag is declared, and no `i128` reference appears anywhere in the crate (verified against its `Cargo.toml` and `src/lib.rs`) — see [`exact_minor` Types](../type/001_exact_minor_types.md) for the full account of the dropped `MinorWide`.

### Statement

The raw subunit is `i64` by default, with `i128` available only behind a feature flag on `exact_minor` — never a separate crate. This keeps the common case cheap while leaving a documented escape hatch for callers whose magnitudes outgrow 64 bits.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:166` | Feature 1 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:384` | "i128 is a feature flag on exact_minor, not a crate" |
