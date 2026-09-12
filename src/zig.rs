use core::slice;
use std::ffi::c_void;

use crate::{CycleCountVm, VmT};

#[repr(C)]
struct RunResult {
    cycles: u64,
    tag: RunResultTag,
    data: RunResultUnion,
}

#[repr(u8)]
enum RunResultTag {
    Done,
    Eof,
    Trap,
    Exit,
    Error,
}

#[repr(C)]
union RunResultUnion {
    trap: u64,
    exit: ExitData,
    err: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ExitData {
    ptr: *const u8,
    len: usize,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Err {
    OutOfMemory,

    StackEmpty,
    InvalidStackIdx,
    Segfault,
    Eof,
    DivideByZero,
    InvalidStackValue,
    UnknownOp,
    PointerTooBig,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Err(Err),
    #[error("trap: {0:x}")]
    Trap(u64),
}

unsafe extern "C" {
    fn zig_allocator() -> *mut c_void;
    fn zig_init(
        gpa: *mut c_void,
        code: *mut u8,
        code_len: usize,
        data: *const u8,
        data_len: usize,
    ) -> *mut c_void;
    fn zig_run(vm: *mut c_void) -> RunResult;
    fn zig_cycles(vm: *mut c_void) -> u64;
}

pub struct Vm {
    ptr: *mut c_void,
    gpa: *mut c_void,
}

impl Vm {
    pub fn new(code: Vec<u8>, data: Vec<u8>) -> Self {
        let (code_ptr, code_len, _) = code.into_raw_parts();
        let (data_ptr, data_len, _) = data.into_raw_parts();
        let gpa = unsafe { zig_allocator() };
        Self { ptr: unsafe { zig_init(gpa, code_ptr, code_len, data_ptr, data_len) }, gpa }
    }

    pub fn run(&mut self) -> Result<Option<&[u8]>, Error> {
        let res = unsafe { zig_run(self.ptr) };
        match res.tag {
            RunResultTag::Done => Ok(None),
            RunResultTag::Eof => Ok(None),
            RunResultTag::Trap => Err(Error::Trap(unsafe { res.data.trap })),
            RunResultTag::Exit => {
                Ok(Some(unsafe { slice::from_raw_parts(res.data.exit.ptr, res.data.exit.len) }))
            }
            RunResultTag::Error => Err(unsafe { Error::Trap(res.data.err) }),
        }
    }
}

impl VmT for Vm {
    type Error = Error;

    fn run(&mut self) -> anyhow::Result<Option<Vec<u8>>, Self::Error> {
        match Vm::run(self) {
            Ok(Some(res)) => Ok(Some(res.to_owned())),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl CycleCountVm for Vm {
    fn cycles(&mut self) -> u64 {
        unsafe { zig_cycles(self.ptr) }
    }
}
