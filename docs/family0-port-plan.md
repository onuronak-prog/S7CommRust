<!-- SPDX-License-Identifier: LGPL-3.0-or-later -->
# Family0 (real S7-1200/1500 legacy auth) — Rust port plan

Port target: `bonk-dev/HarpoS7` `HarpoS7.Family0` (MIT) — the non-TLS auth for **real**
S7-1200/1500 hardware below the TLS firmware floor (public-key families `00:` = S7-1500,
`01:` = S7-1200). Complements the shipped PLCSIM family-`03:` path in `src/legacy/`.

This plan was produced by a deep-mapping pass over the reference C# and verified against
the .NET golden-vector oracle. The full map (six subsystem deep-dives + synthesis) lives in
the session artifact `…/tasks/wuuugt3n5.output` (unescaped copy: scratchpad `family0_plan.md`).

## The field & the blocker
- **Field:** `GF(p)`, `p = 2¹⁶⁰ − 47` (pseudo-Mersenne, Solinas fold 47). Two reps: packed
  radix-2³⁰ (6×u32, 24 B) ⇄ canonical 160-bit (5×u32, 20 B) via `Prepare`/`Finalize`.
- **The EC seed is the one hard part.** The 60-byte `SeedTransform` output is an ECIES-style
  encryption over a **custom ~160-bit curve**; the reference implements it as ~50k lines of
  decompiled "Monolith" point-ops. Everything else (metadata, LUT, checksum, AES/IV chain,
  session key) is pure integer/AES code, portable immediately.
- **R1 — SOLVED (approach A unblocked).** The curve is short-Weierstrass over `GF(2¹⁶⁰−47)`:
  `a = p−1 (−1)`, `b = 0xFDEC56A0F1A148A7CA6F04463A24F5F56C3F3A4F`,
  `G = (0x7d2ed8e2…a39d0c8e, 0x57a735e9…cd8c673b)`, wire encoding **2×20-byte LE, X‖Y**.
  Recovered via a known-point oracle experiment and verified against the real driver (200/200
  on-curve, 380/380 group-law closure, comb-composition), plus reimplemented + validated in
  `ec.rs`. So the ~50k Monolith point-op lines are replaced by a generic curve. **Still open:**
  the group order `n` (needs one SEA run — deferrable), and decoding the `Transform7` comb
  recoding so a generic scalar-mult reproduces `SeedTransform` (`transform6`) byte-for-byte.
  Monolith 9/10 (a keyed hash for `PreSeed`/`KeyDerivation`) still get transcribed — no curve shortcut.

## Module layout (`src/legacy/family0/`)
| File | Status | Contents |
|---|---|---|
| `field.rs` | ✅ done, vector-tested | `BigIntOperations` codec + `GF(2¹⁶⁰−47)` add/sub/mul/square |
| `checksum.rs` | ✅ done, vector-tested | `LutGenerator` (GF(2¹²⁸) table) + `ChecksumTransform` |
| `blob.rs` | ✅ metadata done, vector-tested | `PublicKeyFamily` flags + 48-byte metadata writer |
| `data.rs` | todo | const tables (`Transform7Data`, `Transform1Data`, `SharedData`, …) |
| `ec.rs` | ✅ done, tested | generic short-Weierstrass curve (a,b,G) + affine add/double/scalar-mul |
| `monolith/` | todo (B) | transcribe M9/M10 (keyed hash for PreSeed/KeyDerivation) — no curve shortcut |
| `transforms.rs` | todo | `Transform7/12/13`, `PreSeedTransform`, `KeyDerivationTransform` |
| `seed.rs` | blocked on R1 | `SeedTransform` → 60-byte EC seed (validates vs `transform6`) |
| `auth.rs` | todo | `authenticate_real_plc` + AES/IV/checksum chain + `DeriveSessionKey(key2)` |

Reuse from `src/legacy/` unchanged: `keys::derive_session_key` (pass **key2**),
`blob::derive_key_id`, `fingerprint::*`, `digest`/`session`. Do **not** reuse `aes.rs`
(`HarpoAesCtr`) — family-0 uses a different cipher/MAC (AES-ECB + `RotateLeft31` IV chain +
`ChecksumTransform` LUT MAC).

## Ordered work-list (each step offline-checkable against a golden vector)
1–11. Field codec + arithmetic + LUT + checksum — **✅ done** (`bitops`, `transform8/9/10/11/3/4`).
12. Blob metadata writer — **✅ done** (first 48 B of the S71500 + S71200 auth blobs).
13–14. Monolith10 → Monolith9 (transcribe) — `monolith10/9-{src,dst}.bin`.
15–18. Monolith 1(+Loop), 2, 3–7(+WithCopy), 8, 11 — `monolith*-{src,dst}.bin`.
19–22. Transform12, Transform7, PreSeedTransform, Transform13 — `transform12/7/1/13`.
23. KeyDerivationTransform (needs M9/M10) — `transform2`.
24. **SeedTransform** — `transform6-{publicKey,t1,dst}.bin`, prng fill `[0x2D]` (oracle-confirmed).
25. AES/IV/checksum chain (no isolated vector).
26–27. **AuthenticateRealPlc S71500 + S71200×2** — the acceptance gate: byte-exact 180-B blob
    + 24-B session key from `HarpoS7.Tests/Auth/LegacyAuthenticationSchemeTests.cs`.

## Deterministic-fill order (for the golden tests)
`RealPlcAuthenticator` consumes the fill sequence as: **key2(24), key1(24), IV(16), then inside
`SeedTransform` prng1(20), prng2(20)** — i.e. `[0x35, 0x35/0x99, 0x25, 0x2D, 0x2D]`. Note the
session key is derived from **key2** (PLCSIM used key1), and the metadata key-id is key2's.

## Oracle recipe
dotnet 8 SDK (local install, not on PATH). Golden suite:
`dotnet test C:/harpo/HarpoS7.Family0.Tests` (44/44). Reusable dump harness: `C:\oracle\`.
`SpanExtensions.StaticFillSequence` (internal, via reflection) makes the PRNG deterministic;
every `Transform*`/`Monolith*` `.Execute` is public-static over `Span<byte>`.

## Open questions for the human
1. **Group order `n`:** likely unneeded on the happy path (fixed ephemeral scalars) — defer unless the PLC range-checks.
2. **Scope:** both S71500 + S71200 are implemented (only 2 flag dwords differ). Confirm both for v1, or 1500-first.
3. **Live validation:** the 3 golden vectors validate the full blob offline; the *live* handshake still needs a real S7-1200/1500 on legacy firmware.
