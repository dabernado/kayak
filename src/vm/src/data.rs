use std::fmt;
use std::cell::Cell;

use crate::alloc::api::AllocObject;
use crate::array::Array;
use crate::memory::MutatorScope;
use crate::safeptr::{CellPtr, ScopedPtr, UntypedCellPtr};
use crate::printer::*;

/*
 * Primitive Types
 */

// for rust typechecking
impl AllocObject for () {}

// This type should NEVER be instantiated
pub struct Zero;
impl AllocObject for Zero {}

impl Print for Zero {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "*") }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Unit(u32); // has to represent some kind of data or alloc freaks out
impl AllocObject for Unit {}

impl Unit {
    pub fn new() -> Unit { Unit(0) }
}

impl Print for Unit {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "()") }
}

pub type Nat = u32;
impl AllocObject for Nat {}

impl Print for Nat {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * int is implemented in IRIS via the
 * nat + nat type
*/
pub type Int = i32;
impl AllocObject for Int {}

impl Print for Int {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * bool is implemented in IRIS via the
 * 1 + 1 type, this is only for internal use
*/
pub type Bool = bool;
impl AllocObject for Bool {}

impl Print for Bool {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * Algebraic Data Types
 */

// Negative Types
#[derive(Clone, Debug)]
pub struct Negative<O: AllocObject>(CellPtr<O>);
impl<O: AllocObject> AllocObject for Negative<O> {}

impl<O: AllocObject> Negative<O> {
    pub fn new(data: CellPtr<O>) -> Negative<O> { Negative(data) }

    pub fn data<'guard>(&self, guard: &'guard dyn MutatorScope)
        -> ScopedPtr<'guard, O>
    { self.0.get(guard) }
}

impl<O: AllocObject + Print> Print for Negative<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "-{}", self.0.get(guard)) }
}

// Sum Types
#[derive(Clone, Debug)]
pub enum Sum<L: AllocObject, R: AllocObject> {
    Left(CellPtr<L>),
    Right(CellPtr<R>),
}
impl<L: AllocObject, R: AllocObject> AllocObject for Sum<L, R> {}

impl<L: AllocObject, R: AllocObject> Sum<L, R> {
    // No need for new() method; just initialize as one of the enum options

    pub fn set_data<'guard>(&self, ptr: ScopedPtr<'guard, L>) {
        self.0.set(ptr);
    }
    
    pub fn set_data<'guard>(&self, ptr: ScopedPtr<'guard, R>) {
        self.0.set(ptr);
    }

    pub fn data<'guard>(&self, guard: &'guard dyn MutatorScope)
        -> ScopedPtr<'guard, L>
    {
        self.0.get(guard)
    }

    pub fn data<'guard>(&self, guard: &'guard dyn MutatorScope)
        -> ScopedPtr<'guard, R>
    {
        self.0.get(guard)
    }
}

impl<L: AllocObject + Print, R: AllocObject + Print> Print for Sum<L, R> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Left::(data) => write!(f, "left ({})", data.get(guard)),
            Right::(data) => write!(f, "right ({})", data.get(guard)),
        }
    }
}

// Product Types
#[derive(Clone, Debug)]
pub struct Product<F: AllocObject, S: AllocObject> {
    fst: CellPtr<F>,
    snd: CellPtr<S>,
}
impl<F: AllocObject, S: AllocObject> AllocObject for Product<F, S> {}

impl<F: AllocObject, S: AllocObject> Product<F, S> {
    pub fn new(fst: CellPtr<F>, snd: CellPtr<S>) -> Product<F, S> {
        Product { fst, snd }
    }

    pub fn fst<'guard>(
        &self,
        guard: &'guard dyn MutatorScope
    ) -> ScopedPtr<'guard, F> { self.fst.get(guard) }

    pub fn set_fst<'guard>(
        &self,
        ptr: ScopedPtr<'guard, F>
    ) { self.fst.set(ptr) }
    
    pub fn snd<'guard>(
        &self,
        guard: &'guard dyn MutatorScope
    ) -> ScopedPtr<'guard, S> { self.snd.get(guard) }

    pub fn set_snd<'guard>(
        &self,
        ptr: ScopedPtr<'guard, S>
    ) { self.snd.set(ptr) }
}

impl<F: AllocObject + Print, S: AllocObject + Print> Print for Product<F, S> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let prod = ScopedPtr::new(guard, self);

        write!(f, "({}", prod.fst.get(guard))?;
        write!(f, ", {})", prod.snd.get(guard))
    }
}

// Inductive Types
// TODO: Switch to packed implementation
pub type Inductive<O> = Array<CellPtr<O>>;

impl<O: AllocObject> AllocObject for Negative<O> {}