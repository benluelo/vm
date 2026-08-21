use tracing::trace;

use crate::{Error, Hook, Vm, raw};

// type VmResult<H: Hook> = Result<Option<Vec<u8>>, Error<H>>;
type VmResult<H> = Result<Option<Vec<u8>>, Error<H>>;

type F<H = ()> = fn(&mut Vm<H>) -> VmResult<H>;

impl<H: Hook> Vm<H> {
    pub fn run_tc(&mut self) -> VmResult<H> {
        dispatch(self)
    }
}

const TABLE<H: Hook>: [F<H>; 256] = {
    let mut out = [unknown_op::<H> as F<H>; 256];

    out[raw::PUSH0 as usize]  = do_push0::<H>  as F<H>;
    out[raw::PUSH1 as usize]  = do_push1::<H>  as F<H>;
    out[raw::PUSH2 as usize]  = do_push2::<H>  as F<H>;
    out[raw::PUSH3 as usize]  = do_push3::<H>  as F<H>;
    out[raw::PUSH4 as usize]  = do_push4::<H>  as F<H>;
    out[raw::PUSH5 as usize]  = do_push5::<H>  as F<H>;
    out[raw::PUSH6 as usize]  = do_push6::<H>  as F<H>;
    out[raw::PUSH7 as usize]  = do_push7::<H>  as F<H>;
    out[raw::PUSH8 as usize]  = do_push8::<H>  as F<H>;

    out[raw::DUP as usize]    = do_dup::<H>    as F<H>;
    out[raw::DUP0 as usize]   = do_dup0::<H>   as F<H>;
    out[raw::SWAP as usize]   = do_swap::<H>   as F<H>;
    out[raw::SWAP0 as usize]  = do_swap0::<H>  as F<H>;
    out[raw::POP as usize]    = do_pop::<H>    as F<H>;

    out[raw::ALLOC as usize]  = do_alloc::<H>  as F<H>;

    out[raw::WRITE1 as usize] = do_write1::<H> as F<H>;
    out[raw::WRITE2 as usize] = do_write2::<H> as F<H>;
    out[raw::WRITE3 as usize] = do_write3::<H> as F<H>;
    out[raw::WRITE4 as usize] = do_write4::<H> as F<H>;
    out[raw::WRITE5 as usize] = do_write5::<H> as F<H>;
    out[raw::WRITE6 as usize] = do_write6::<H> as F<H>;
    out[raw::WRITE7 as usize] = do_write7::<H> as F<H>;
    out[raw::WRITE8 as usize] = do_write8::<H> as F<H>;

    out[raw::READ1 as usize]  = do_read1::<H>  as F<H>;
    out[raw::READ2 as usize]  = do_read2::<H>  as F<H>;
    out[raw::READ3 as usize]  = do_read3::<H>  as F<H>;
    out[raw::READ4 as usize]  = do_read4::<H>  as F<H>;
    out[raw::READ5 as usize]  = do_read5::<H>  as F<H>;
    out[raw::READ6 as usize]  = do_read6::<H>  as F<H>;
    out[raw::READ7 as usize]  = do_read7::<H>  as F<H>;
    out[raw::READ8 as usize]  = do_read8::<H>  as F<H>;

    out[raw::DREAD1 as usize] = do_dread1::<H> as F<H>;
    out[raw::DREAD2 as usize] = do_dread2::<H> as F<H>;
    out[raw::DREAD3 as usize] = do_dread3::<H> as F<H>;
    out[raw::DREAD4 as usize] = do_dread4::<H> as F<H>;
    out[raw::DREAD5 as usize] = do_dread5::<H> as F<H>;
    out[raw::DREAD6 as usize] = do_dread6::<H> as F<H>;
    out[raw::DREAD7 as usize] = do_dread7::<H> as F<H>;
    out[raw::DREAD8 as usize] = do_dread8::<H> as F<H>;

    out[raw::DCOPY as usize]  = do_dcopy::<H>  as F<H>;
    out[raw::DLEN as usize]   = do_dlen::<H>   as F<H>;

    out[raw::ADD as usize]    = do_add::<H>    as F<H>;
    out[raw::SUB as usize]    = do_sub::<H>    as F<H>;
    out[raw::MUL as usize]    = do_mul::<H>    as F<H>;
    out[raw::DIV as usize]    = do_div::<H>    as F<H>;
    out[raw::EXP as usize]    = do_exp::<H>    as F<H>;
    out[raw::MOD as usize]    = do_mod::<H>    as F<H>;

    out[raw::EQ as usize]     = do_eq::<H>     as F<H>;
    out[raw::NEQ as usize]    = do_neq::<H>    as F<H>;
    out[raw::LT as usize]     = do_lt::<H>     as F<H>;
    out[raw::GT as usize]     = do_gt::<H>     as F<H>;
    out[raw::NOT as usize]    = do_not::<H>    as F<H>;
    out[raw::SHL as usize]    = do_shl::<H>    as F<H>;
    out[raw::SHR as usize]    = do_shr::<H>    as F<H>;
    out[raw::NEG as usize]    = do_neg::<H>    as F<H>;
    out[raw::OR as usize]     = do_or::<H>     as F<H>;
    out[raw::XOR as usize]    = do_xor::<H>    as F<H>;
    out[raw::AND as usize]    = do_and::<H>    as F<H>;

    out[raw::JUMP as usize]   = do_jump::<H>   as F<H>;
    out[raw::JNZ as usize]    = do_jnz::<H>    as F<H>;
    out[raw::CALL as usize]   = do_call::<H>   as F<H>;
    out[raw::EXIT as usize]   = do_exit::<H>   as F<H>;
    out[raw::TRAP as usize]   = do_trap::<H>   as F<H>;

    out
};

fn dispatch<H: Hook>(vm: &mut Vm<H>) -> VmResult<H> {
    let table = const {
        let mut out = [unknown_op::<H> as F<H>; 256];

        out[raw::PUSH0 as usize] = do_push0::<H> as F<H>;
        out[raw::PUSH1 as usize] = do_push1::<H> as F<H>;
        out[raw::PUSH2 as usize] = do_push2::<H> as F<H>;
        out[raw::PUSH3 as usize] = do_push3::<H> as F<H>;
        out[raw::PUSH4 as usize] = do_push4::<H> as F<H>;
        out[raw::PUSH5 as usize] = do_push5::<H> as F<H>;
        out[raw::PUSH6 as usize] = do_push6::<H> as F<H>;
        out[raw::PUSH7 as usize] = do_push7::<H> as F<H>;
        out[raw::PUSH8 as usize] = do_push8::<H> as F<H>;

        out[raw::DUP as usize] = do_dup::<H> as F<H>;
        out[raw::DUP0 as usize] = do_dup0::<H> as F<H>;
        out[raw::SWAP as usize] = do_swap::<H> as F<H>;
        out[raw::SWAP0 as usize] = do_swap0::<H> as F<H>;
        out[raw::POP as usize] = do_pop::<H> as F<H>;

        out[raw::ALLOC as usize] = do_alloc::<H> as F<H>;

        out[raw::WRITE1 as usize] = do_write1::<H> as F<H>;
        out[raw::WRITE2 as usize] = do_write2::<H> as F<H>;
        out[raw::WRITE3 as usize] = do_write3::<H> as F<H>;
        out[raw::WRITE4 as usize] = do_write4::<H> as F<H>;
        out[raw::WRITE5 as usize] = do_write5::<H> as F<H>;
        out[raw::WRITE6 as usize] = do_write6::<H> as F<H>;
        out[raw::WRITE7 as usize] = do_write7::<H> as F<H>;
        out[raw::WRITE8 as usize] = do_write8::<H> as F<H>;

        out[raw::READ1 as usize] = do_read1::<H> as F<H>;
        out[raw::READ2 as usize] = do_read2::<H> as F<H>;
        out[raw::READ3 as usize] = do_read3::<H> as F<H>;
        out[raw::READ4 as usize] = do_read4::<H> as F<H>;
        out[raw::READ5 as usize] = do_read5::<H> as F<H>;
        out[raw::READ6 as usize] = do_read6::<H> as F<H>;
        out[raw::READ7 as usize] = do_read7::<H> as F<H>;
        out[raw::READ8 as usize] = do_read8::<H> as F<H>;

        out[raw::DREAD1 as usize] = do_dread1::<H> as F<H>;
        out[raw::DREAD2 as usize] = do_dread2::<H> as F<H>;
        out[raw::DREAD3 as usize] = do_dread3::<H> as F<H>;
        out[raw::DREAD4 as usize] = do_dread4::<H> as F<H>;
        out[raw::DREAD5 as usize] = do_dread5::<H> as F<H>;
        out[raw::DREAD6 as usize] = do_dread6::<H> as F<H>;
        out[raw::DREAD7 as usize] = do_dread7::<H> as F<H>;
        out[raw::DREAD8 as usize] = do_dread8::<H> as F<H>;

        out[raw::DCOPY as usize] = do_dcopy::<H> as F<H>;
        out[raw::DLEN as usize] = do_dlen::<H> as F<H>;

        out[raw::ADD as usize] = do_add::<H> as F<H>;
        out[raw::SUB as usize] = do_sub::<H> as F<H>;
        out[raw::MUL as usize] = do_mul::<H> as F<H>;
        out[raw::DIV as usize] = do_div::<H> as F<H>;
        out[raw::EXP as usize] = do_exp::<H> as F<H>;
        out[raw::MOD as usize] = do_mod::<H> as F<H>;

        out[raw::EQ as usize] = do_eq::<H> as F<H>;
        out[raw::NEQ as usize] = do_neq::<H> as F<H>;
        out[raw::LT as usize] = do_lt::<H> as F<H>;
        out[raw::GT as usize] = do_gt::<H> as F<H>;
        out[raw::NOT as usize] = do_not::<H> as F<H>;
        out[raw::SHL as usize] = do_shl::<H> as F<H>;
        out[raw::SHR as usize] = do_shr::<H> as F<H>;
        out[raw::NEG as usize] = do_neg::<H> as F<H>;
        out[raw::OR as usize] = do_or::<H> as F<H>;
        out[raw::XOR as usize] = do_xor::<H> as F<H>;
        out[raw::AND as usize] = do_and::<H> as F<H>;

        out[raw::JUMP as usize] = do_jump::<H> as F<H>;
        out[raw::JNZ as usize] = do_jnz::<H> as F<H>;
        out[raw::CALL as usize] = do_call::<H> as F<H>;
        out[raw::EXIT as usize] = do_exit::<H> as F<H>;
        out[raw::TRAP as usize] = do_trap::<H> as F<H>;

        out
    };

    trace!("");
    trace!("pc: {}", vm.pc);
    trace!("stack: {:x?}", vm.stack);
    // trace!("memory: {}", vm.memory.encode_hex());

    trace!("stack: {}, memory: {}", vm.stack.len(), vm.memory.len());

    std::thread::sleep(std::time::Duration::from_millis(20));

    // trace!("pre_cycle");
    if let Err(err) = vm.hook.pre_cycle() {
        return Err(Error::Hook(err));
    };

    let index = vm.pc;
    vm.pc += 1;
    match vm.code.get(index) {
        Some(op) => become table[(*op) as usize](vm),
        None => Ok(None),
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
    ($vm:ident) => {{
        let len = $vm.stack.len();
        if len == 0 {
            return Err(Error::<H>::StackEmpty);
        } else {
            unsafe {
                let x = *$vm.stack.get_unchecked(len - 1);
                $vm.stack.set_len(len - 1);
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
        hook!($vm, $op);
        let top = last!($vm);
        let ptr = as_ptr!(*top);
        trace!("ptr: {ptr:x}");
        let res = ok_or!($vm.memory.get(ptr..ptr + $n), Error::<H>::Segfault);
        *top = u64_from_bytes(res);
    }};
}

macro_rules! dread_n {
    ($vm:ident, $op:ident, $n:literal) => {{
        trace!("dread{}", $n);
        hook!($vm, $op);
        let top = last!($vm);
        let ptr = as_ptr!(*top);
        trace!("ptr: {ptr:x}");
        let res = ok_or!($vm.data.get(ptr..ptr + $n), Error::<H>::Segfault);
        *top = u64_from_bytes(res);
    }};
}

macro_rules! binop {
    ($vm:ident, $op:ident, $op_name:literal, $f:ident) => {{
        trace!($op_name);
        hook!($vm, $op);
        let len = $vm.stack.len();

        if len < 2 {
            return Err(Error::<H>::StackEmpty);
        };

        let b = unsafe { len.unchecked_sub(2) };
        let a = unsafe { len.unchecked_sub(1) };

        unsafe {
            *$vm.stack.get_unchecked_mut(b) =
                crate::op::$f(*$vm.stack.get_unchecked(b), *$vm.stack.get_unchecked(a))
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

macro_rules! do_op {
    ($(fn $f:ident<$H:ident>($vm:ident) $body:block)*) => {
        $(fn $f<$H: Hook>($vm: &mut Vm<$H>) -> VmResult<$H> $body)*
    };
}

do_op! {
    fn unknown_op<H>(vm) {
        return Err(Error::<H>::UnknownOp(vm.code[vm.pc]))
    }

    fn do_push0<H>(vm) {
        hook!(vm, PUSH0);
        trace!("push0");
        vm.stack.push(0);

        become dispatch(vm)
    }

    fn do_push1<H>(vm) {
        push_n!(vm, PUSH1, 1);

        become dispatch(vm)
    }

    fn do_push2<H>(vm) {
        push_n!(vm, PUSH2, 2);

        become dispatch(vm)
    }

    fn do_push3<H>(vm) {
        push_n!(vm, PUSH3, 3);

        become dispatch(vm)
    }

    fn do_push4<H>(vm) {
        push_n!(vm, PUSH4, 4);

        become dispatch(vm)
    }

    fn do_push5<H>(vm) {
        push_n!(vm, PUSH5, 5);

        become dispatch(vm)
    }

    fn do_push6<H>(vm) {
        push_n!(vm, PUSH6, 6);

        become dispatch(vm)
    }

    fn do_push7<H>(vm) {
        push_n!(vm, PUSH7, 7);

        become dispatch(vm)
    }

    fn do_push8<H>(vm) {
        push_n!(vm, PUSH8, 8);

        become dispatch(vm)
    }

    fn do_dup<H>(vm) {
        trace!("dup");
        hook!(vm, DUP);
        let idx = as_ptr!(*last!(vm));
        trace!("idx = {idx:x}");
        let stack_idx = ok_or!(
            vm.stack.len().checked_sub(idx).and_then(|i| i.checked_sub(2)),
            Error::<H>::InvalidStackIdx
        );

        unsafe {
            *vm.stack.last_mut().unwrap_unchecked() =
                *vm.stack.get_unchecked(stack_idx);
        };

        become dispatch(vm)
    }

    fn do_dup0<H>(vm) {
        trace!("dup0");
        hook!(vm, DUP0);
        let stack_idx = ok_or!(vm.stack.len().checked_sub(1), Error::<H>::InvalidStackIdx);

        vm.stack.push(*ok_or!(vm.stack.get(stack_idx), Error::<H>::InvalidStackIdx));

        become dispatch(vm)
    }

    fn do_swap<H>(vm) {
        trace!("swap");
        hook!(vm, SWAP);
        let idx = as_ptr!(pop!(vm));
        let idx = ok_or!(idx.checked_add(1), Error::<H>::InvalidStackValue);
        let len = vm.stack.len();
        if len < idx {
            return Err(Error::<H>::InvalidStackIdx);
        }
        // SAFETY: Len is at least 1 as per above
        let a_idx = unsafe { len.unchecked_sub(1) };
        // SAFETY: Len is at least idx as per above
        let b_idx = unsafe { a_idx.unchecked_sub(idx) };
        vm.stack.swap(a_idx, b_idx);

        become dispatch(vm)
    }

    fn do_write1<H>(vm) {
        write_n!(vm, WRITE1, 1);

        become dispatch(vm)
    }

    fn do_write2<H>(vm) {
        write_n!(vm, WRITE2, 2);

        become dispatch(vm)
    }

    fn do_write3<H>(vm) {
        write_n!(vm, WRITE3, 3);

        become dispatch(vm)
    }

    fn do_write4<H>(vm) {
        write_n!(vm, WRITE4, 4);

        become dispatch(vm)
    }

    fn do_write5<H>(vm) {
        write_n!(vm, WRITE5, 5);

        become dispatch(vm)
    }

    fn do_write6<H>(vm) {
        write_n!(vm, WRITE6, 6);

        become dispatch(vm)
    }

    fn do_write7<H>(vm) {
        write_n!(vm, WRITE7, 7);

        become dispatch(vm)
    }

    fn do_write8<H>(vm) {
        write_n!(vm, WRITE8, 8);

        become dispatch(vm)
    }

    fn do_read1<H>(vm) {
        read_n!(vm, READ1, 1);

        become dispatch(vm)
    }

    fn do_read2<H>(vm) {
        read_n!(vm, READ2, 2);

        become dispatch(vm)
    }

    fn do_read3<H>(vm) {
        read_n!(vm, READ3, 3);

        become dispatch(vm)
    }

    fn do_read4<H>(vm) {
        read_n!(vm, READ4, 4);

        become dispatch(vm)
    }

    fn do_read5<H>(vm) {
        read_n!(vm, READ5, 5);

        become dispatch(vm)
    }

    fn do_read6<H>(vm) {
        read_n!(vm, READ6, 6);

        become dispatch(vm)
    }

    fn do_read7<H>(vm) {
        read_n!(vm, READ7, 7);

        become dispatch(vm)
    }

    fn do_read8<H>(vm) {
        read_n!(vm, READ8, 8);

        become dispatch(vm)
    }

    fn do_dread1<H>(vm) {
        dread_n!(vm, DREAD1, 1);

        become dispatch(vm)
    }

    fn do_dread2<H>(vm) {
        dread_n!(vm, DREAD2, 2);

        become dispatch(vm)
    }

    fn do_dread3<H>(vm) {
        dread_n!(vm, DREAD3, 3);

        become dispatch(vm)
    }

    fn do_dread4<H>(vm) {
        dread_n!(vm, DREAD4, 4);

        become dispatch(vm)
    }

    fn do_dread5<H>(vm) {
        dread_n!(vm, DREAD5, 5);

        become dispatch(vm)
    }

    fn do_dread6<H>(vm) {
        dread_n!(vm, DREAD6, 6);

        become dispatch(vm)
    }

    fn do_dread7<H>(vm) {
        dread_n!(vm, DREAD7, 7);

        become dispatch(vm)
    }

    fn do_dread8<H>(vm) {
        dread_n!(vm, DREAD8, 8);

        become dispatch(vm)
    }

    fn do_swap0<H>(vm) {
        trace!("swap0");
        hook!(vm, SWAP0);
        let b_idx = ok_or!(vm.stack.len().checked_sub(2), Error::<H>::InvalidStackIdx);
        // SAFETY: Len is at least 2 as per above
        let a_idx = unsafe { vm.stack.len().unchecked_sub(1) };
        vm.stack.swap(a_idx, b_idx);

        become dispatch(vm)
    }

    fn do_pop<H>(vm) {
        trace!("pop");
        hook!(vm, POP);
        let value = pop!(vm);
        trace!("{value:x}");

        become dispatch(vm)
    }

    fn do_alloc<H>(vm) {
        trace!("alloc");
        hook!(vm, ALLOC);
        let size = as_ptr!(pop!(vm));
        trace!("size: {size}");
        vm.memory.extend(vec![0; size]);

        become dispatch(vm)
    }
    fn do_dcopy<H>(vm) {
        trace!("dcopy");
        hook!(vm, DCOPY);
        if vm.stack.len() < 3 {
            return Err(Error::<H>::StackEmpty);
        };

        let [src, dst, len] = &vm.stack[vm.stack.len() - 3..] else {
            unsafe { std::hint::unreachable_unchecked() }
            // unreachable!();
        };

        let len = as_ptr!(*len);
        let dst = as_ptr!(*dst);
        let src = as_ptr!(*src);

        unsafe { vm.stack.set_len(vm.stack.len() - 3) };
        // vm.stack.truncate(vm.stack.len() - 3);

        trace!("len: {len:x}, dst: {dst:x}, src: {src:x}");

        ok_or!(vm.memory.get_mut(dst..dst + len), Error::<H>::Segfault)
            .copy_from_slice(ok_or!(vm.data.get(src..src + len), Error::<H>::Segfault));

        become dispatch(vm)
    }

    fn do_dlen<H>(vm) {
        trace!("dlen");
        hook!(vm, DLEN);
        vm.stack.push(vm.data.len() as u64);

        become dispatch(vm)
    }

    fn do_add<H>(vm) {
        binop!(vm, ADD, "add", add);

        become dispatch(vm)
    }
    fn do_sub<H>(vm) {
        binop!(vm, SUB, "sub", sub);

        become dispatch(vm)
    }
    fn do_mul<H>(vm) {
        binop!(vm, MUL, "mul", mul);

        become dispatch(vm)
    }
    fn do_div<H>(vm) {
        trace!("div");
        hook!(vm, DIV);
        let len = vm.stack.len();
        if len < 2 {
            return Err(Error::<H>::StackEmpty);
        };
        let b = len - 2;
        let a = len - 1;
        unsafe {
            *vm.stack.get_unchecked_mut(b) =
                match crate::op::div(*vm.stack.get_unchecked(b), *vm.stack.get_unchecked(a)) {
                    Ok(ok) => ok,
                    Err(err) => return Err(err.widen()),
                }
        };
        unsafe { vm.stack.set_len(a) };

        become dispatch(vm)
    }
    fn do_exp<H>(vm) {
        binop!(vm, EXP, "exp", expmod);

        become dispatch(vm)
    }
    fn do_mod<H>(vm) {
        trace!("mod");
        hook!(vm, MOD);
        let a = pop!(vm);
        let b = last!(vm);
        trace!("{b:x} % {a:x}");
        *b = match crate::op::r#mod(*b, a) {
            Ok(ok) => ok,
            Err(err) => return Err(err.widen()),
        };

        become dispatch(vm)
    }
    fn do_eq<H>(vm) {
        binop!(vm, EQ, "eq", eq);

        become dispatch(vm)
    }
    fn do_neq<H>(vm) {
        binop!(vm, NEQ, "neq", neq);

        become dispatch(vm)
    }
    fn do_lt<H>(vm) {
        binop!(vm, LT, "lt", lt);

        become dispatch(vm)
    }
    fn do_gt<H>(vm) {
        binop!(vm, GT, "gt", gt);

        become dispatch(vm)
    }
    fn do_not<H>(vm) {
        trace!("not");
        hook!(vm, NOT);
        let a = last!(vm);
        *a = crate::op::not(*a);

        become dispatch(vm)
    }
    fn do_shr<H>(vm) {
        binop!(vm, SHR, "shr", shr);

        become dispatch(vm)
    }
    fn do_shl<H>(vm) {
        binop!(vm, SHL, "shl", shl);

        become dispatch(vm)
    }
    fn do_neg<H>(vm) {
        trace!("neg");
        hook!(vm, NEG);
        let a = last!(vm);
        *a = crate::op::neg(*a);

        become dispatch(vm)
    }
    fn do_or<H>(vm) {
        binop!(vm, OR, "or", or);

        become dispatch(vm)
    }
    fn do_xor<H>(vm) {
        binop!(vm, XOR, "xor", xor);

        become dispatch(vm)
    }
    fn do_and<H>(vm) {
        binop!(vm, AND, "and", and);

        become dispatch(vm)
    }

    fn do_jump<H>(vm) {
        trace!("jump");
        hook!(vm, JUMP);
        let dst = pop!(vm);
        trace!("dst = {dst:x}");
        vm.pc = as_ptr!(dst);

        become dispatch(vm)
    }
    fn do_jnz<H>(vm) {
        trace!("jnz");
        hook!(vm, JNZ);
        let dst = pop!(vm);
        trace!("dst = {dst:x}");
        let value = pop!(vm);
        trace!("value = {value:x}");
        if value != 0 {
            vm.pc = as_ptr!(dst);
        }

        become dispatch(vm)
    }

    fn do_call<H>(vm) {
        trace!("call");
        hook!(vm, CALL);
        let top = last!(vm);
        let address = *top;
        *top = vm.pc as u64;
        vm.pc = as_ptr!(address);

        become dispatch(vm)
    }

    fn do_exit<H>(vm) {
        trace!("exit");
        hook!(vm, EXIT);
        let len = as_ptr!(pop!(vm));
        let ptr = as_ptr!(pop!(vm));

        return Ok(Some(
            ok_or!(vm.memory.get(ptr..ptr + len), Error::<H>::Segfault).to_vec(),
        ));
    }

    fn do_trap<H>(vm) {
        trace!("trap");
        hook!(vm, TRAP);
        let value = pop!(vm);
        return Err(Error::<H>::Trap(value));
    }
}

// pub fn step(&mut self) -> Result<StepResult, Error<H>> {
//     if self.pc >= self.code.len() {
//         return Ok(StepResult::Eof);
//     }
//     let op = unsafe { self.code.get_unchecked(self.pc) };

//     self.pc += 1;

//     trace!("");

//     trace!("pc: {:x}", self.pc);
//     trace!("stack: {:x?}", self.stack);
//     trace!("memory: {}", self.memory.encode_hex());

//     if let Err(err) = self.hook.post_cycle() {
//         return Err(Error::Hook(err));
//     };

//     Ok(StepResult::Stepped)
// }
