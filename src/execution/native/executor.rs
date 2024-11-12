use super::{state::State, RuntimeResultCode};
use std::fs::File;
use std::io::Write;
use std::mem;

pub struct NativeExecutor {
    code: dynasmrt::ExecutableBuffer,
    code_start: dynasmrt::AssemblyOffset,
}

impl NativeExecutor {
    pub fn new(code: dynasmrt::ExecutableBuffer, code_start: dynasmrt::AssemblyOffset) -> Self {
        Self { code, code_start }
    }

    pub fn run(self, state: &mut State) -> RuntimeResultCode {
        #[cfg(target_os = "windows")]
        let native_code: extern "win64" fn(
            state: *mut State,
            tape_start: *mut u8,
            tape_end: *mut u8,
        ) -> u8 = unsafe { mem::transmute(self.code.ptr(self.code_start)) };

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let native_code: extern "sysv64" fn(
            state: *mut State,
            tape_start: *mut u8,
            tape_end: *mut u8,
        ) -> u8 = unsafe { mem::transmute(self.code.ptr(self.code_start)) };

        let tape_start = state.tape.as_mut_ptr();
        let tape_end = unsafe { tape_start.add(state.tape.len()) };
        let result = native_code(state, tape_start, tape_end);
        RuntimeResultCode::try_from(result).unwrap()
    }

    pub fn dump_binary(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(&self.code).unwrap();
    }
}
