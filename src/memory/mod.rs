use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::ReadProcessMemory,
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

pub struct MemoryManager {
    process_handle: HANDLE,
    module_base: usize,
}

impl MemoryManager {
    pub fn new(process_name: &str) -> Option<Self> {
        unsafe {
            let handle = OpenProcess(PROCESS_ALL_ACCESS, false, 1234).ok()?;
            Some(Self {
                process_handle: handle,
                module_base: 0x140000000,
            })
        }
    }

    pub fn read<T>(&self, address: usize) -> Option<T> {
        let mut buffer = std::mem::MaybeUninit::<T>::uninit();
        unsafe {
            ReadProcessMemory(
                self.process_handle,
                address as _,
                buffer.as_mut_ptr() as _,
                std::mem::size_of::<T>(),
                None,
            ).ok()?;
            Some(buffer.assume_init())
        }
    }

    pub fn scan_pattern(&self, pattern: &[u8], mask: &str) -> Option<usize> {
        let mut buffer = vec![0u8; 0x1000];
        for offset in (0..0x7FFFFFFF).step_by(0x1000) {
            if self.read_bytes(self.module_base + offset, &mut buffer).is_ok() {
                for i in 0..buffer.len() - pattern.len() {
                    if Self::pattern_match(&buffer[i..], pattern, mask) {
                        return Some(self.module_base + offset + i);
                    }
                }
            }
        }
        None
    }

    fn pattern_match(data: &[u8], pattern: &[u8], mask: &str) -> bool {
        pattern.iter().enumerate().all(|(i, &byte)| {
            mask.as_bytes()[i] == b'?' || byte == data[i]
        })
    }
}