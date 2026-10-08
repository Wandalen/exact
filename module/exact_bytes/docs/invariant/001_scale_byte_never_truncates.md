# Invariant: Scale Byte Never Truncates

### Scope

- **Purpose**: State that the wire record's one-byte `scale` field can never silently wrap a scale value too large to fit it, so an encoded `scale` byte can always be trusted to equal the real scale it was written at.
- **Responsibility**: The compile-time assertion that `exact_scale::MONEY_SCALE` fits in a `u8`.
- **In Scope**: The one `as u8` cast of `exact_scale::MONEY_SCALE`, `SCALE_BYTE`, which every `*_to_wire` function writes and `check_header` compares on every `*_from_wire`.
- **Out of Scope**: What a decoder does with a `scale` byte once decoded — comparing it against the expected scale and refusing a mismatch (→ `../format/001_wire_record_encoding.md`'s "Round-Trip Guarantee" section, which this invariant's guarantee is a precondition for, not a restatement of).

### Statement

`src/lib.rs:48` declares `const _ : () = assert!( exact_scale::MONEY_SCALE <=
u8::MAX as u32 );`. `SCALE_BYTE` (`src/lib.rs:52`) casts `exact_scale::MONEY_SCALE`
(a `u32`) down to the record's one-byte `scale` field via `as u8`, once; every
`*_to_wire` function writes it and every `*_from_wire` function checks against
it. An `as` cast from a wider integer to a narrower one truncates silently rather than
erroring when the value does not fit — this assertion is what rules that out
at compile time, before any cast runs, rather than leaving it to be noticed
only if a future scale change happened to also produce a decode-time symptom.

### Rationale

An `as u8` truncation is exactly the kind of failure that would not announce
itself: `MONEY_SCALE` growing past 255 would silently wrap the encoded
`scale` byte to some smaller value, and every `*_to_wire` call would keep
succeeding, producing wire records whose `scale` field no longer means what
the rest of the family assumes — only a decoder's `BadScale` check downstream
would have any chance of catching the resulting mismatch, and only if the
wrapped value happened to disagree with what that decoder expects. Pinning
this as a compile-time assertion, co-located with the cast it protects,
converts an silent, data-level corruption risk into a build failure at the
one crate where the cast actually happens — the same shape of protection
`exact_parse`'s own cross-crate scale assertion provides for a different pair
of constants.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:48` | The compile-time assertion itself |
| `src/lib.rs:50-52` | `SCALE_BYTE` — the one `exact_scale::MONEY_SCALE as u8` cast this assertion protects |
| `src/lib.rs:198, 218, 239` | The three `*_to_wire` functions writing `SCALE_BYTE` into a record |
| `src/lib.rs:104-115` | `check_header` — comparing a decoded record's `scale` byte against `SCALE_BYTE` |

### Tests

| File | Relationship |
|------|--------------|
| `tests/wire_roundtrip_test.rs` | Round-trip coverage that exercises the scale byte this assertion protects, indirectly — the assertion's own enforcement is compile-time and has no corresponding runtime test |
