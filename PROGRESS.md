# scraps — Progress

Tracks status against the v0 milestone ladder defined in ARCHITECTURE.md.
Updated as part of the PR that completes (or advances) each milestone.

## v0 — Dispatch + Output

| # | Milestone | Status | PR | Notes |
|---|-----------|--------|----|----|
| 1 | Node daemon skeleton — compiles, runs, logs startup, cross-compiles to `aarch64-linux-musl` | Not started | — | |
| 2 | Capability detection — RAM, arch, KVM, WASM, serialized to proto | Blocked on #1 | — | |
| 3 | Node registration — connects to control plane gRPC server, registers capability profile | Blocked on #2 | — | |
| 4 | Task dispatch + output — `scraps run 'echo hello'` round-trips through a node | Blocked on #3 | — | |

## Status legend

- **Not started** — no branch open
- **In progress** — branch open, work underway
- **In review** — PR open, awaiting review
- **Done** — PR merged to `main`
- **Blocked on #N** — per the milestone ladder, do not start until #N is merged
