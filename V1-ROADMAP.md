# libmwemu v1.0 Roadmap

What needs to happen before cutting a stable 1.0 release.

Current state: 560 tests, multi-platform (PE/ELF/Mach-O), x86/x64/AArch64,
kernel-mode driver emulation, syscall mode, and Windows system simulation
(PEB/TEB/LDR, heap, TLS, IAT binding, SEH/VEH).

---

## P0 — Must-have (stability contract)

### 1. No panics on guest code

The emulator must never `panic!` on any guest binary, no matter how
malformed. Currently there are **48 `unimplemented!()` panics** across DLL
gateways and **9 `panic!`** calls in the emulator core. Every one of these
is a crash-on-unknown-sample bug.

**Work:**
- [ ] Replace every `unimplemented!()` in API gateways with a log + skip
      (or return-error when `skip_unimplemented` is false). Affected DLLs
      (64-bit): advapi32, comctl32, comctl64, dnsapi, gdi32, kernelbase,
      ole32, oleaut32, shell32, shlwapi, urlmon, user32, uxtheme, version,
      wincrt, winhttp, wininet, ws2_32.
- [ ] Audit `panic!` / `expect()` / `unwrap()` in `emu/`, `engine/`,
      `maps/` on paths reachable from guest code. Convert to `Result` or
      exception.
- [ ] Unimplemented x86 instructions should raise `#UD` (exception), not
      `return false` + silent stop.

### 2. Public API stability

The `libmwemu` crate API is what downstream users (pymwemu, cmwemu, custom
tools) depend on. A v1 promises it won't break without a major bump.

**Work:**
- [ ] Review `pub` visibility — internals that leaked as `pub` should be
      `pub(crate)`.
- [ ] Stable entry points: `Emu::new`, `load_code`, `run`, `run_to`,
      `set_hook`, config fields. Document with `///` doc comments.
- [ ] Stabilize the `Hooks` callback signatures.
- [ ] `cfg` fields that control behavior should have builder methods or
      a `Config` struct with `Default`.

### 3. Error handling

- [ ] `run()` should propagate every failure as `Err`, never `panic`.
- [ ] Malformed PE/ELF/Mach-O input: return `Err` from `load_code`, not
      panic on short reads or missing sections.
- [ ] OOM in `maps.alloc()` / `create_map()`: propagate, don't `expect`.

---

## P1 — Should-have (completeness)

### 4. Core Windows API coverage

Triage the top unimplemented APIs hit by real samples and implement stubs.
Priority list (based on Enigma, Themida, and common malware):

- [ ] **ole32**: `CoInitialize`, `CoCreateInstance`, `CoTaskMemAlloc/Free`
- [ ] **advapi32**: `RegOpenKeyExA/W`, `RegQueryValueExA/W`, `RegCloseKey`
- [ ] **crypt32**: `CryptDecodeObjectEx`, `CertOpenStore` (stub)
- [ ] **user32**: `wsprintfA/W`, `LoadStringA/W`, `CharNextA`
- [ ] **kernel32**: `CreateFileMappingA/W`, `MapViewOfFile` (basic)
- [ ] **ntdll**: `RtlGetVersion`, `NtQueryInformationProcess` (anti-debug)

### 5. 32-bit / 64-bit parity

- [ ] ntdll cross-module call interception (done for 64-bit, needs 32-bit)
- [ ] `_HEAP` structure emulation for 32-bit
- [ ] VirtualAllocEx for 32-bit (same commit-without-reserve fix)
- [ ] Test coverage: every 64-bit winapi test should have a 32-bit mirror

### 6. Syscall mode (`--syscall-mode`) robustness

- [ ] `_HEAP` struct: populate `BlocksIndex` function pointers or intercept
      CFG dispatch so ntdll heap code doesn't crash (see `docs/ANTI_EMU_HEAP.md`)
- [ ] TEB fields: `+0x1480` (FlsData), `+0x1858` (heap-related) used by
      ntdll internals
- [ ] More syscall stubs: `NtQueryVirtualMemory`, `NtProtectVirtualMemory`,
      `NtAllocateVirtualMemory`

---

## P2 — Nice-to-have (polish)

### 7. Documentation

- [ ] `///` doc comments on all public API types and methods
- [ ] `examples/` directory with common use cases:
      - Load and run a shellcode
      - Load and run a PE with hooks
      - Use from Python (pymwemu)
      - Use from C (cmwemu)
- [ ] `CHANGELOG.md`

### 8. CI improvements

- [ ] Add integration tests (test_linux, test_windows, test_syscall) to CI
      matrix
- [ ] Clippy clean: `cargo clippy -- -D warnings`

### 9. Performance

- [ ] Profile hot paths (instruction decode + dispatch, memory read/write)
- [ ] Consider caching `get_addr_name()` lookups (called on every RIP change)
- [ ] Benchmark: instructions-per-second on reference samples

---

## Non-goals for v1

- Full Windows kernel emulation (kernel-mode is experimental, driver-focused)
- GUI / debugger frontend (the console debugger `-c` is enough)
- Network I/O emulation (ws2_32 stubs are sufficient)
- Multi-threaded guest execution (single-threaded scheduler is fine)

---

## How to track progress

Each item above can be a GitHub issue. The TEST.md integration tests are the
gate: nothing ships if any step fails. The Enigma binary
(`test/exe64win_enigma.bin`) is the canary — it exercises heap, TLS, API
resolution, and anti-emulation checks harder than anything else in the test
suite.
