use std::cell::Cell;

use crate::alloc::api::AllocObject;
use crate::array::{ArraySize, IndexedContainer};
use crate::data::{Nat, Bool, Product, Sum, Inductive};
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorScope, MutatorView};
use crate::safeptr::{ScopedPtr, CellPtr};

/*
 * Instructions
 */
pub enum Instruction {
    Id,
    Zeroi,
    Zeroe,
    Swaps,
    Assls,
    Assrs,
    Uniti,
    Unite,
    Swapp,
    Asslp,
    Assrp,
    Dist,
    Fact,
    Expn,
    Coln,
    Start,
    End,
    StartSum,
    SplitSum,
    EndSum,
    StartProd,
    SplitProd,
    EndProd,
    Fold(Bool),
    Ufold(Bool),
    Call(Nat),
    Uncall(Nat),
    Release(&str),
    Bind(&str),
    Spawn((Nat, &str)),
    Return((Nat, &str))
}

/*
 * Iris Datatypes
 */
pub type Function = Product<Metadata, Inductive<Opcode>>;

#[derive(Clone)]
pub struct Continuation {
    function: CellPtr<Function>,
    ip: Cell<ArraySize>,
    direction: Cell<bool>,
}
impl AllocObject for Continuation {}

impl Continuation {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
        func: ScopedPtr<'guard, Function>
    ) -> Result<ScopedPtr<'guard, Continuation>, RuntimeError> {
        mem.alloc(Continuation {
            function: CellPtr::new_with(func),
            ip: Cell::new(0),
            direction: Cell::new(false),
        })
    }

    // TODO: Optimize (way too many indirections)
    pub fn fetch_instr<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
    ) -> Result<ScopedPtr<'guard, Instruction<()>>, RuntimeError> {
        let ptr = self.function.get(guard)
            .get(guard, self.ip.get())?;
        Ok(ptr.get(guard))
    }

    pub fn set_ip(&self, i: ArraySize) { self.ip.set(i); }
    pub fn jump(&self, jmp: ArraySize) {
        if !self.direction() {
            self.set_ip(self.ip() + jmp);
        } else {
            self.set_ip(self.ip() - jmp);
        }
    }

    pub fn reset(&self, jmp: ArraySize) {
        if !self.direction() {
            self.ip.set(0);
        } else {
            self.ip.set(jmp);
        }
    }

    pub fn ip(&self) -> ArraySize { self.ip.get() }
    pub fn direction(&self) -> bool { self.direction.get() }
    pub fn reverse(&self) { self.direction.set(!self.direction()) }
}

// Decoding Functions
pub fn get_opcode(instr: Opcode, dir: bool) -> u8 {
    if !dir {
        (instr & OP_MASK) as u8
    } else {
        (!instr & OP_MASK) as u8
    }
}

pub fn decode_i(instr: Opcode) -> u32 {
    (instr & I_MASK) >> 5
}

// Encoding Functions
pub fn encode_i(op: u8, imm: u32) -> Result<Opcode, RuntimeError> {
    // check if within bounds
    if imm <= MAX_ITYPE_FIELD {
        Ok((imm << 5) ^ (op as u32))
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}
