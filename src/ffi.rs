use core::slice;

use crate::VmT;

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Vm {
    vm: bindings::Vm,
}

impl VmT for Vm {
    type Error = Error;

    fn run(&mut self) -> anyhow::Result<Option<Vec<u8>>, Self::Error> {
        Vm::run(self).map_err(Error)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(bindings::VmResult);

impl Vm {
    pub fn new(code: Vec<u8>, data: Vec<u8>) -> Self {
        let (code_ptr, code_len, _) = code.into_raw_parts();
        let (data_ptr, data_len, _) = data.into_raw_parts();
        Self {
            vm: unsafe {
                bindings::new_vm(
                    bindings::new_fat(code_ptr, code_len),
                    bindings::new_fat(data_ptr, data_len),
                )
            },
        }
    }

    pub fn run(&mut self) -> Result<Option<Vec<u8>>, bindings::VmResult> {
        match unsafe { bindings::step_vm(&mut self.vm) } {
            bindings::VmResult_VM_OK => Ok(None),
            bindings::VmResult_VM_STEP_RESULT_EXIT => Ok(Some(unsafe {
                slice::from_raw_parts(self.vm.out.exit.ptr, self.vm.out.exit.len).to_vec()
            })),
            res => Err(res),
        }
    }
}
