Do this tests and indicate if passes all the tests, if one test fail dont continue and notify it.
If a step is commented '#' dont do that step

1. cargo test
2. cargo test --release 
3. make test_linux   --> you have to see the result of ls and good termination
4. make test_windows --> (take time) you have to see that arrives at least to 231067159 instructions emulated.
5. make test_inception --> arrives to exit: ** syscall exit()  1
6. make test_syscall --> (take a lot of time) you have to see yellow syscalls  and you have to see that LdrInitializeThunk is completed: ntdll!LdrInitializeThunk emulated completely.
7. check that format is ok, otherwise fixe it. cargo fmt --all -- --check

8. check clippy is clean (denies warnings): make clippy
   --> or: cargo clippy --locked --package mwemu --package libmwemu --package rs-header --package cmwemu --all-targets --target x86_64-unknown-linux-gnu -- -D warnings
   Must exit 0. The pre-existing backlog is grandfathered via crate-level
   `#![allow(...)]` blocks marked "clippy v1 burn-down backlog"; new warnings
   outside that set fail the build. MSRV is 1.95.
