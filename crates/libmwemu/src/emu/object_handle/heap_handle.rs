/// Emulated Win32 heap object backing a handle from `HeapCreate`/`GetProcessHeap`.
///
/// `arena` is an index into `Emu::heap_arenas`; arena 0 is always the
/// process heap (the one `Emu::heap_mut()` returns).
pub struct HeapHandle {
    pub initSZ: usize,
    pub maxSZ: usize,
    pub arena: usize,
}

impl HeapHandle {
    pub fn new(_opt: u32, initSZ: usize, maxSZ: usize, arena: usize) -> Self {
        Self {
            initSZ,
            maxSZ,
            arena,
        }
    }
}
