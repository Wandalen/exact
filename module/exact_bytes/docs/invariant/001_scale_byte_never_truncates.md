# Invariant: Scale Byte Never Truncates

### Scope

- **Purpose**: State that the wire record's one-byte `scale` field can never silently wrap a scale value too large to fit it, so an encoded `scale` byte can always be trusted to equal the real scale it was written at.
- **Responsibility**: The compile-time assertion inside `scale_byte` that a value's scale `S` fits in a `u8`.
- **In Scope**: The one `as u8` cast, inside `scale_byte::< S >()`, that every `*_to_wire` function writes into a record and every `*_from_wire` function compares a decoded record against.
- **Out of Scope**: What a decoder does with a `scale` byte once decoded — comparing it against the expected scale and refusing a mismatch (→ `../format/001_wire_record_encoding.md`'s "Round-Trip Guarantee" section, which this invariant's guarantee is a precondition for, not a restatement of).

### Statement

`scale_byte::< S >()` (`src/lib.rs:50-54`) is the only place a scale becomes
the record's one-byte `scale` field:

```rust
const fn scale_byte< const S : u32 >() -> u8
{
  const { assert!( S <= u8::MAX as u32 ) };
  S as u8
}
```

An `as` cast from a wider integer to a narrower one truncates silently rather
than erroring when the value does not fit. The inline `const` block is
evaluated at compile time, once for every scale `S` a program encodes or
decodes at, so a scale the cast would cut fails the build before any cast
runs. Every `*_to_wire` function writes `scale_byte::< S >()`, and every
`*_from_wire` function compares a decoded record's `scale` byte against it.

In practice no reachable scale comes close: a `Decimal< S >` with `S > 18`
cannot be built at all — every constructor fails the build for it
(→ [`exact_kind`'s Every Scale Is At Most 18](../../../exact_kind/docs/invariant/003_every_scale_is_at_most_18.md)).
The assertion here guards the cast itself, so the guarantee does not depend
on that other limit.

### Rationale

An `as u8` truncation is exactly the kind of failure that would not announce
itself: a scale past 255 would silently wrap the encoded `scale` byte to some
smaller value, and every `*_to_wire` call would keep succeeding, producing
wire records whose `scale` field no longer means what the rest of the family
assumes. Since every operation became generic over the scale
(→ [ADR-002](../decisions/002_scale_byte_from_the_type.md)), the byte is no
longer one constant checked once: it is `S`, for every `S` in use, and the
assertion sits inside the generic function so it is checked for each of them.
Before that change the same guarantee was a top-level assertion on
`exact_scale::MONEY_SCALE` beside a fixed `SCALE_BYTE` constant.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:50-54` | `scale_byte` — the cast and the compile-time assertion that protects it |
| `src/lib.rs:201, 221, 242` | The three `*_to_wire` functions writing `scale_byte::< S >()` into a record |
| `src/lib.rs:107-118` | `check_header` — comparing a decoded record's `scale` byte against the expected one |

### Tests

| File | Relationship |
|------|--------------|
| `tests/wire_roundtrip_test.rs` | Round-trip coverage that exercises the scale byte this assertion protects, indirectly — the assertion's own enforcement is compile-time and has no corresponding runtime test |
