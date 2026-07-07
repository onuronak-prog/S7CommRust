# Family0 golden vectors

These binary fixtures are golden byte-vectors captured from
[`bonk-dev/HarpoS7`](https://github.com/bonk-dev/HarpoS7) (`HarpoS7.Family0.Tests/Blobs`),
which is MIT-licensed (see `LICENSE-HarpoS7` at the repo root). They pin the Rust port of the
legacy S7-1200/1500 (`00:`/`01:` key family) auth to the reference implementation, byte-for-byte.

- `bitops/` — `BigIntOperations` `Prepare`/`Finalize`/`PrepareFinalize` (`prep_*`, `finalize_*`, `mixed_*`).
- `transforms/` — the `BigInt{Add,Sub,Mul,Square}` field ops (`transform8/9/10/11`),
  `LutGenerator` (`transform3`), and `ChecksumTransform` (`transform4`).

Each is consumed by a `#[cfg(test)]` `include_bytes!` in the corresponding `src/legacy/family0/`
module. The end-to-end `AuthenticateRealPlc` acceptance vectors live inline in HarpoS7's
`LegacyAuthenticationSchemeTests.cs` and are transcribed directly into the test modules.
