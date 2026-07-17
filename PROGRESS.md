# scraps — Progress

Tracks status against the v0 milestone ladder defined in ARCHITECTURE.md.
Updated as part of the PR that completes (or advances) each milestone.

## v0 — Dispatch + Output

| # | Milestone | Status | PR | Notes |
|---|-----------|--------|----|----|
| 1 | Node daemon skeleton — compiles, runs, logs startup, cross-compiles to `aarch64-linux-musl` | In review | feat/node-skeleton | tokio + tracing, cross-compiles to both aarch64/x86_64-unknown-linux-musl via native musl toolchain (messense/macos-cross-toolchains) |
| 2 | Capability detection — RAM, arch, KVM, WASM, serialized to proto | In review | feat/capability-detection | Rust struct only, no `.proto` yet (deferred to #3). RAM/arch/KVM detection is real per-OS logic (Linux `/proc/meminfo` + `/dev/kvm`, macOS `sysctlbyname` via `libc`) since the Mac is meant to be a real node, not just a build machine. No `wasm_capable` field — control plane will threshold on `total_ram_bytes` itself, kept centralized so policy can change without redeploying nodes. |
| 3 | Node registration — connects to control plane gRPC server, registers capability profile | Not started | — | |
| 4 | Task dispatch + output — `scraps run 'echo hello'` round-trips through a node | Blocked on #3 | — | |

## Status legend

- **Not started** — no branch open
- **In progress** — branch open, work underway
- **In review** — PR open, awaiting review
- **Done** — PR merged to `main`
- **Blocked on #N** — per the milestone ladder, do not start until #N is merged
