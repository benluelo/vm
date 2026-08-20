use tracing::trace;

use crate::{Error, Hook, Vm, raw::WRITE1};

type VmResult<H: Hook> = Result<Option<Vec<u8>>, Error<H>>;

type F<H = ()> = fn(&mut Vm<H>) -> VmResult<H>;

impl<H: Hook> Vm<H> {
    pub fn run_tc(&mut self) -> VmResult<H> {
        dispatch(self)
    }
}

fn dispatch<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    const TABLE<H: Hook>: [F<H>; 256] = {
        let mut out = [unknown_op::<H> as F<H>; 256];

        out[0x00] = do_push0::<H> as F<H>;
        out[0x01] = do_push1::<H> as F<H>;
        // out[0x02] = do_push2::<H> as F<H>;
        // out[0x03] = do_push3::<H> as F<H>;
        // out[0x04] = do_push4::<H> as F<H>;
        // out[0x05] = do_push5::<H> as F<H>;
        // out[0x06] = do_push6::<H> as F<H>;
        // out[0x07] = do_push7::<H> as F<H>;
        out[0x08] = do_push8::<H> as F<H>;
        // out[0x09] = do_dup::<H> as F<H>;
        // out[0x0a] = do_dup0::<H> as F<H>;
        // out[0x0b] = do_swap::<H> as F<H>;
        // out[0x0c] = do_swap0::<H> as F<H>;
        // out[0x0d] = do_pop::<H> as F<H>;
        // out[0x20] = do_alloc::<H> as F<H>;
        // out[0x21] = do_write1::<H> as F<H>;
        // out[0x22] = do_write2::<H> as F<H>;
        // out[0x23] = do_write3::<H> as F<H>;
        // out[0x24] = do_write4::<H> as F<H>;
        // out[0x25] = do_write5::<H> as F<H>;
        // out[0x26] = do_write6::<H> as F<H>;
        // out[0x27] = do_write7::<H> as F<H>;
        // out[0x28] = do_write8::<H> as F<H>;
        // out[0x29] = do_read1::<H> as F<H>;
        // out[0x2a] = do_read2::<H> as F<H>;
        // out[0x2b] = do_read3::<H> as F<H>;
        // out[0x2c] = do_read4::<H> as F<H>;
        // out[0x2d] = do_read5::<H> as F<H>;
        // out[0x2e] = do_read6::<H> as F<H>;
        // out[0x2f] = do_read7::<H> as F<H>;
        // out[0x30] = do_read8::<H> as F<H>;
        // out[0x31] = do_dread1::<H> as F<H>;
        // out[0x32] = do_dread2::<H> as F<H>;
        // out[0x33] = do_dread3::<H> as F<H>;
        // out[0x34] = do_dread4::<H> as F<H>;
        // out[0x35] = do_dread5::<H> as F<H>;
        // out[0x36] = do_dread6::<H> as F<H>;
        // out[0x37] = do_dread7::<H> as F<H>;
        // out[0x38] = do_dread8::<H> as F<H>;
        // out[0x39] = do_dcopy::<H> as F<H>;
        // out[0x3a] = do_dlen::<H> as F<H>;
        // out[0x40] = do_add::<H> as F<H>;
        // out[0x41] = do_sub::<H> as F<H>;
        // out[0x42] = do_mul::<H> as F<H>;
        // out[0x43] = do_div::<H> as F<H>;
        // out[0x44] = do_exp::<H> as F<H>;
        // out[0x45] = do_mod::<H> as F<H>;
        // out[0x4a] = do_eq::<H> as F<H>;
        // out[0x4b] = do_neq::<H> as F<H>;
        // out[0x4c] = do_lt::<H> as F<H>;
        // out[0x4d] = do_gt::<H> as F<H>;
        // out[0x4e] = do_not::<H> as F<H>;
        // out[0x4f] = do_shl::<H> as F<H>;
        // out[0x50] = do_shr::<H> as F<H>;
        // out[0x51] = do_neg::<H> as F<H>;
        // out[0x52] = do_or::<H> as F<H>;
        // out[0x53] = do_xor::<H> as F<H>;
        // out[0x54] = do_and::<H> as F<H>;
        // out[0xa0] = do_jump::<H> as F<H>;
        // out[0xa1] = do_jnz::<H> as F<H>;
        // out[0xa2] = do_call::<H> as F<H>;
        // out[0xa4] = do_exit::<H> as F<H>;
        // out[0xa5] = do_trap::<H> as F<H>;

        out
    };

    if let Err(err) = vm.hook.pre_cycle() {
        return Err(Error::Hook(err));
    };

    match vm.code.get(vm.pc) {
        Some(op) => become TABLE::<H>[(*op) as usize](vm),
        None => todo!(),
    }
}

macro_rules! ok_or {
    ($e:expr, $err:expr) => {
        match $e {
            Some(t) => t,
            None => return Err($err),
        }
    };
}

macro_rules! as_ptr {
    ($e:expr) => {{
        #[cfg(target_pointer_width = "64")]
        { $e as usize }
        #[cfg(not(target_pointer_width = "64"))]
        {
            let n = $e;
            if n > (usize::MAX as u64) {
                return Error::PointerTooBig(n);
            } else {
                $e as u64
            }
        }
    }};
}

macro_rules! pop {
    () => {{
        let len = self.stack.len();
        if len == 0 {
            return Err(Error::<H>::StackEmpty);
        } else {
            unsafe {
                let x = *self.stack.get_unchecked(len - 1);
                self.stack.set_len(len - 1);
                x
            }
        }
    }};
}

macro_rules! last {
    ($vm:ident) => {
        ok_or!($vm.stack.last_mut(), Error::<H>::StackEmpty)
    };
}

macro_rules! push_n {
    ($vm:ident, $op:ident, $n:literal) => {{
        $vm.pc += $n;
        let mut v = [0_u8; 8];
        let res = ok_or!($vm.code.get($vm.pc - $n..$vm.pc), Error::<H>::Eof);
        v[8 - $n..].copy_from_slice(res);
        let n = u64::from_be_bytes(v);
        trace!("push{} {n:x}", $n);
        hook!($vm, $op(*v.rsplit_array_ref::<$n>().1));
        $vm.stack.push(n);
    }};
}

macro_rules! write_n {
    ($vm:ident, $op:ident, $n:literal) => {{
        trace!("write{}", $n);
        hook!($vm, $op);
        let len = $vm.stack.len();
        if len < 2 {
            return Err(Error::<H>::StackEmpty);
        };
        unsafe {
            let value_idx = len.unchecked_sub(1);
            let ptr_idx = len.unchecked_sub(2);
            let value = *$vm.stack.get_unchecked(value_idx);
            let ptr = as_ptr!((*$vm.stack.get_unchecked(ptr_idx)));
            $vm.stack.set_len(ptr_idx);
            trace!("{value:x} @ {ptr:x}");
            let bytes = value.to_be_bytes();
            ok_or!($vm.memory.get_mut(ptr..ptr + $n), Error::<H>::Segfault)
                .copy_from_slice(&bytes[8 - $n..]);
        }
    }};
}

macro_rules! read_n {
    ($vm:ident, $op:ident, $n:literal) => {{
        trace!("read{}", $n);
        hook!($op);
        let top = last!();
        let ptr = as_ptr!(*top);
        trace!("ptr: {ptr:x}");
        let res = ok_or!($vm.memory.get(ptr..ptr + $n), Error::<H>::Segfault);
        *top = u64_from_bytes(res);
    }};
}

macro_rules! dread_n {
    ($vm:ident, $op:ident, $n:literal) => {{
        trace!("dread{}", $n);
        hook!($op);
        let top = last!();
        let ptr = as_ptr!(*top);
        trace!("ptr: {ptr:x}");
        let res = ok_or!($vm.data.get(ptr..ptr + $n), Error::<H>::Segfault);
        *top = u64_from_bytes(res);
    }};
}

macro_rules! binop {
    ($vm:ident, $op:ident, $op_name:literal, $f:ident) => {{
        trace!($op_name);
        hook!($op);
        let len = $vm.stack.len();

        if len < 2 {
            return Err(Error::<H>::StackEmpty);
        };

        let b = unsafe { len.unchecked_sub(2) };
        let a = unsafe { len.unchecked_sub(1) };

        unsafe {
            *$vm.stack.get_unchecked_mut(b) =
                op::$f(*$vm.stack.get_unchecked(b), *$vm.stack.get_unchecked(a))
        };

        unsafe { $vm.stack.set_len(a) };
    }};
}

macro_rules! hook {
    ($vm:ident, $($op:tt)+) => {
        match $vm.hook.cycle($vm.pc, crate::Op::$($op)+, &$vm.stack, &$vm.memory) {
            Ok(ok) => ok,
            Err(err) => return Err(Error::Hook(err)),
        }
    };
}

#[inline(always)]
fn u64_from_bytes(arr: &[u8]) -> u64 {
    let mut v = [0; 8];
    v[8 - arr.len()..].copy_from_slice(arr);
    u64::from_be_bytes(v)
}

fn unknown_op<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    Err(Error::<H>::UnknownOp(vm.code[vm.pc]))
}

fn do_push0<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    hook!(vm, PUSH0);
    trace!("push0");
    vm.stack.push(0);
    vm.pc += 1;
    become dispatch(vm)
}

fn do_push1<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    push_n!(vm, PUSH1, 1);
    vm.pc += 1;
    become dispatch(vm)
}

fn do_push2<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    push_n!(vm, PUSH2, 2);
    vm.pc += 1;
    become dispatch(vm)
}

fn do_push8<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    push_n!(vm, PUSH8, 8);
    vm.pc += 1;
    become dispatch(vm)
}

fn do_write1<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    write_n!(vm, WRITE1, 1);
    vm.pc += 1;
    become dispatch(vm)
}

// pub fn step(&mut self) -> Result<StepResult, Error<H>> {

//     if self.pc >= self.code.len() {
//         return Ok(StepResult::Eof);
//     }
//     let op = unsafe { self.code.get_unchecked(self.pc) };

//     self.pc += 1;

//     trace!("");

//     match *op {
//         raw::PUSH0 => {
//             hook!(PUSH0);
//             trace!("push0");
//             self.stack.push(0);
//         }
//         raw::PUSH1 => push_n!(PUSH1, 1),
//         raw::PUSH2 => push_n!(PUSH2, 2),
//         raw::PUSH3 => push_n!(PUSH3, 3),
//         raw::PUSH4 => push_n!(PUSH4, 4),
//         raw::PUSH5 => push_n!(PUSH5, 5),
//         raw::PUSH6 => push_n!(PUSH6, 6),
//         raw::PUSH7 => push_n!(PUSH7, 7),
//         raw::PUSH8 => push_n!(PUSH8, 8),
//         raw::DUP => {
//             trace!("dup");
//             hook!(DUP);
//             let idx = as_ptr!(*last!());
//             trace!("idx = {idx:x}");
//             let stack_idx = ok_or!(
//                 self.stack.len().checked_sub(idx).and_then(|i|
// i.checked_sub(2)),                 Error::<H>::InvalidStackIdx
//             );

//             unsafe {
//                 *self.stack.last_mut().unwrap_unchecked() =
//                     *self.stack.get_unchecked(stack_idx);
//             };
//         }
//         raw::DUP0 => {
//             trace!("dup0");
//             hook!(DUP0);
//             let stack_idx =
//                 ok_or!(self.stack.len().checked_sub(1),
// Error::<H>::InvalidStackIdx);

//             self.stack.push(*ok_or!(self.stack.get(stack_idx),
// Error::<H>::InvalidStackIdx))         }
//         raw::SWAP => {
//             trace!("swap");
//             hook!(SWAP);
//             let idx = as_ptr!(pop!());
//             let idx = ok_or!(idx.checked_add(1),
// Error::<H>::InvalidStackValue);             let len = self.stack.len();
//             if len < idx {
//                 return Err(Error::<H>::InvalidStackIdx);
//             }
//             // SAFETY: Len is at least 1 as per above
//             let a_idx = unsafe { len.unchecked_sub(1) };
//             // SAFETY: Len is at least idx as per above
//             let b_idx = unsafe { a_idx.unchecked_sub(idx) };
//             self.stack.swap(a_idx, b_idx);
//         }
//         raw::SWAP0 => {
//             trace!("swap0");
//             hook!(SWAP0);
//             let b_idx = ok_or!(self.stack.len().checked_sub(2),
// Error::<H>::InvalidStackIdx);             // SAFETY: Len is at least 2 as per
// above             let a_idx = unsafe { self.stack.len().unchecked_sub(1) };
//             self.stack.swap(a_idx, b_idx);
//         }
//         raw::POP => {
//             trace!("pop");
//             hook!(POP);
//             pop!();
//         }
//         raw::ALLOC => {
//             trace!("alloc");
//             hook!(ALLOC);
//             let size = as_ptr!(pop!());
//             self.memory.extend(vec![0; size]);
//         }

//         raw::WRITE1 => write_n!(WRITE1, 1),
//         raw::WRITE2 => write_n!(WRITE2, 2),
//         raw::WRITE3 => write_n!(WRITE3, 3),
//         raw::WRITE4 => write_n!(WRITE4, 4),
//         raw::WRITE5 => write_n!(WRITE5, 5),
//         raw::WRITE6 => write_n!(WRITE6, 6),
//         raw::WRITE7 => write_n!(WRITE7, 7),
//         raw::WRITE8 => write_n!(WRITE8, 8),

//         raw::READ1 => read_n!(READ1, 1),
//         raw::READ2 => read_n!(READ2, 2),
//         raw::READ3 => read_n!(READ3, 3),
//         raw::READ4 => read_n!(READ4, 4),
//         raw::READ5 => read_n!(READ5, 5),
//         raw::READ6 => read_n!(READ6, 6),
//         raw::READ7 => read_n!(READ7, 7),
//         raw::READ8 => read_n!(READ8, 8),

//         raw::DREAD1 => dread_n!(DREAD1, 1),
//         raw::DREAD2 => dread_n!(DREAD2, 2),
//         raw::DREAD3 => dread_n!(DREAD3, 3),
//         raw::DREAD4 => dread_n!(DREAD4, 4),
//         raw::DREAD5 => dread_n!(DREAD5, 5),
//         raw::DREAD6 => dread_n!(DREAD6, 6),
//         raw::DREAD7 => dread_n!(DREAD7, 7),
//         raw::DREAD8 => dread_n!(DREAD8, 8),

//         raw::DCOPY => {
//             trace!("dcopy");
//             hook!(DCOPY);
//             if self.stack.len() < 3 {
//                 return Err(Error::<H>::StackEmpty);
//             };

//             let [src, dst, len] = &self.stack[self.stack.len() - 3..] else {
//                 unsafe { std::hint::unreachable_unchecked() }
//                 // unreachable!();
//             };

//             let len = as_ptr!(*len);
//             let dst = as_ptr!(*dst);
//             let src = as_ptr!(*src);

//             unsafe { self.stack.set_len(self.stack.len() - 3) };
//             // self.stack.truncate(self.stack.len() - 3);

//             trace!("len: {len:x}, dst: {dst:x}, src: {src:x}");

//             ok_or!(self.memory.get_mut(dst..dst + len), Error::<H>::Segfault)
//                 .copy_from_slice(ok_or!(self.data.get(src..src + len),
// Error::<H>::Segfault));         }

//         raw::DLEN => {
//             trace!("dlen");
//             hook!(DLEN);
//             self.stack.push(self.data.len() as u64);
//         }

//         raw::ADD => binop!(ADD, "add", add),
//         raw::SUB => binop!(SUB, "sub", sub),
//         raw::MUL => binop!(MUL, "mul", mul),
//         raw::DIV => {
//             trace!("div");
//             hook!(DIV);
//             let len = self.stack.len();
//             if len < 2 {
//                 return Err(Error::<H>::StackEmpty);
//             };
//             let b = len - 2;
//             let a = len - 1;
//             unsafe {
//                 *self.stack.get_unchecked_mut(b) =
//                     match op::div(*self.stack.get_unchecked(b),
// *self.stack.get_unchecked(a)) {                         Ok(ok) => ok,
//                         Err(err) => return Err(err.widen()),
//                     }
//             };
//             unsafe { self.stack.set_len(a) };
//         }
//         raw::EXP => binop!(EXP, "exp", expmod),
//         raw::MOD => {
//             trace!("mod");
//             hook!(MOD);
//             let a = pop!();
//             let b = last!();
//             trace!("{b:x} % {a:x}");
//             *b = match op::r#mod(*b, a) {
//                 Ok(ok) => ok,
//                 Err(err) => return Err(err.widen()),
//             };
//         }
//         raw::EQ => binop!(EQ, "eq", eq),
//         raw::NEQ => binop!(NEQ, "neq", neq),
//         raw::LT => binop!(LT, "lt", lt),
//         raw::GT => binop!(GT, "gt", gt),
//         raw::NOT => {
//             trace!("not");
//             hook!(NOT);
//             let a = last!();
//             *a = op::not(*a);
//         }
//         raw::SHR => binop!(SHR, "shr", shr),
//         raw::SHL => binop!(SHL, "shl", shl),
//         raw::NEG => {
//             trace!("neg");
//             hook!(NEG);
//             let a = last!();
//             *a = op::neg(*a);
//         }
//         raw::OR => binop!(OR, "or", or),
//         raw::XOR => binop!(XOR, "xor", xor),
//         raw::AND => binop!(AND, "and", and),

//         raw::JUMP => {
//             trace!("jump");
//             hook!(JUMP);
//             let dst = pop!();
//             trace!("dst = {dst:x}");
//             self.pc = as_ptr!(dst);
//         }
//         raw::JNZ => {
//             trace!("jnz");
//             hook!(JNZ);
//             let dst = pop!();
//             trace!("dst = {dst:x}");
//             let value = pop!();
//             trace!("value = {value:x}");
//             if value != 0 {
//                 self.pc = as_ptr!(dst);
//             }
//         }
//         raw::CALL => {
//             trace!("call");
//             hook!(CALL);
//             let top = last!();
//             let address = *top;
//             *top = self.pc as u64;
//             self.pc = as_ptr!(address);
//         }
//         raw::EXIT => {
//             trace!("exit");
//             hook!(EXIT);
//             let len = as_ptr!(pop!());
//             let ptr = as_ptr!(pop!());

//             return Ok(StepResult::Exit(
//                 ok_or!(self.memory.get(ptr..ptr + len),
// Error::<H>::Segfault).to_vec(),             ));
//         }
//         raw::TRAP => {
//             trace!("trap");
//             hook!(TRAP);
//             let value = pop!();
//             return Err(Error::<H>::Trap(value));
//         }
//         op => return Err(Error::<H>::UnknownOp(op)),
//     }

//     trace!("pc: {:x}", self.pc);
//     trace!("stack: {:x?}", self.stack);
//     trace!("memory: {}", self.memory.encode_hex());

//     if let Err(err) = self.hook.post_cycle() {
//         return Err(Error::Hook(err));
//     };

//     Ok(StepResult::Stepped)
// }
