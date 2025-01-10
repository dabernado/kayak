use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize, StackContainer};
use crate::bytecode::*;
use crate::context::{Context, ContextStack};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::op::*;
use crate::safeptr::*;

#[derive(PartialEq)]
pub enum EvalStatus {
    Pending,
    Ok,
    Err,
}

pub struct Thread {
    continuation: CellPtr<Continuation>,
    cxt_stack: CellPtr<ContextStack>,
    data: UntypedCellPtr,
}

impl AllocObject for Thread {}

impl Thread {
    pub fn alloc_with_arg<'guard>(
        mem: &'guard MutatorView,
        data: ScopedPtr<'guard, Product<Function, ()>>
    )
        -> Result<ScopedPtr<'guard, Thread>, RuntimeError>
    {
        let cont = Continuation::alloc(mem, data.fst(mem))?;
        let cxts = Array::<Context>::alloc_with_capacity(mem, 256)?;
        cxts.push(mem, Context::Nil)?;

        mem.alloc(Thread {
            continuation: CellPtr::new_with(cont),
            cxt_stack: CellPtr::new_with(cxts),
            data: CellPtr::new_with(data.snd(mem)),
        })
    }

    pub fn call_func<'guard>(
        &self,
        mem: &'guard dyn MutatorScope,
        start: ArraySize,
        end: ArraySize,
        not: bool,
    ) {
        let cont = self.continuation.get(mem);
        let current_dir = cont.direction();

        if !current_dir {
            if not {
                cont.set_ip(end);
                cont.reverse();
            } else {
                cont.set_ip(start);
            }
        } else {
            if not {
                cont.set_ip(start);
                cont.reverse();
            } else {
                cont.set_ip(end);
            }
        }
    }

    fn eval_context<'guard>(&self, mem: &'guard MutatorView)
        -> Result<(), RuntimeError>
    {
        let cxt_stack = self.cxt_stack.get(mem);
        let cont = self.continuation.get(mem);

        match cxt_stack.top(mem)? {
            Context::Nil => {},
            Context::Call { not: _, ret: _ } => {},
            Context::First {
                snd_op_index,
                snd_val,
                root_val
            } => {
                let ip = cont.ip();

                // if executing in reverse, will exit combinator
                // once PRODE is encountered
                // else, check if moving into second part
                if ip == snd_op_index && !cont.direction() {
                    root_val.get(mem).set_fst(self.data.get(mem));
                    self.data.set(snd_val.get(mem));
                    
                    // push Second onto context stack
                    let new_cxt = Context::Second {
                        fst_op_index: ip - 1,
                        fst_val: CellPtr::new_with(self.data.get(mem)),
                        root_val,
                    };

                    cxt_stack.pop(mem)?;
                    cxt_stack.push(mem, new_cxt)?;
                }
            },
            Context::Second {
                fst_op_index,
                fst_val,
                root_val
            } => {
                let ip = cont.ip();

                // if executing forwards, will exit combinator
                // once PRODE is encountered
                // else, check if moving into first part
                if cont.direction() && ip == fst_op_index {
                    root_val.get(mem).set_snd(self.data.get(mem));
                    self.data.set(fst_val.get(mem));
                    
                    // push First onto context stack
                    let new_cxt = Context::First {
                        snd_op_index: ip + 1,
                        snd_val: CellPtr::new_with(self.data.get(mem)),
                        root_val,
                    };

                    cxt_stack.pop(mem)?;
                    cxt_stack.push(mem, new_cxt)?;
                }
            },
            Context::Left {
                right_op_index,
                jump,
                root_val
            } => {
                let ip = cont.ip();

                // if executing backwards, will exit combinator
                // once SUME is encountered
                // else, check if moving out of left part
                if !cont.direction() && ip == right_op_index {
                    // exit combinator
                    cxt_stack.pop(mem)?;
                    cont.jump(jump + 1);
                    root_val.get(mem).set_data(self.data.get(mem));
                    self.data.set(root_val.get(mem).as_untyped(mem));
                }
            },
            Context::Right {
                left_op_index,
                jump,
                root_val
            } => {
                let ip = cont.ip();

                // if executing forwards, will exit combinator
                // once SUME is encountered
                // else, check if moving out of right part
                if cont.direction() && ip == left_op_index {
                    // exit combinator
                    cxt_stack.pop(mem)?;
                    cont.jump(jump + 1);
                    root_val.get(mem).set_data(self.data.get(mem));
                    self.data.set(root_val.get(mem).as_untyped(mem));
                }
            },
        }

        Ok(())
    }

    pub fn eval_next_instr<'guard>(&self, mem: &'guard MutatorView)
        -> Result<EvalStatus, RuntimeError>
    {
        // check the context stack for any necessary state changes
        self.eval_context(mem)?;

        let cont = self.continuation.get(mem)
            .as_ref(mem);
        let cxt_stack = self.cxt_stack.get(mem);
        let data = self.data.get(mem);

        // get instruction
        // TODO: Optimize (way too many indirections)
        let instruction = cont.fetch_instr(mem)?;
        let op = *(instruction.fst(mem));
        let arg = instruction.snd(mem);
        let opcode = get_opcode(op, cont.direction());

        match opcode {
            Id => {}, // identity
            Zeroi => {
                let new_data = mem.alloc(zeroi(data))?;
                self.data.set(new_data.as_untyped(mem));
            },
            Zeroe => {
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };
                let inner = zeroe(cast_ptr, mem);

                self.data.set(inner);
                mem.dealloc(cast_ptr)?;
            },
            Uniti => {
                let new_data = mem.alloc(uniti(data, mem)?)?;
                self.data.set(new_data.as_untyped(mem));
            },
            Unite => {
                let cast_ptr = unsafe { data.cast::<Product<Unit, ()>>(mem) };
                let inner = unite(cast_ptr, mem);

                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr.fst(mem))?;
                mem.dealloc(cast_ptr)?;
            },
            Swapp => {
                let cast_ptr = unsafe { data.cast::<Product<(), ()>>(mem) };

                swapp(&cast_ptr, mem);
            },
            Assrp => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                assrp(&cast_ptr, mem);
            },
            Asslp => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                asslp(&cast_ptr, mem);
            },
            Swaps => {
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<()>>(mem)
                };

                swaps(&cast_ptr, lc, rc, mem);
            },
            Assrs => {}, // TODO: Implement
            Assls => {}, // TODO: Implement
            Dist => {
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Product<Sum<()>, ()>>(mem)
                };

                let sum = dist(cast_ptr, lc, rc, mem)?;
                self.data.set(sum.as_untyped(mem));
            },
            Fact => {
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<Product<(), ()>>>(mem)
                };

                let prod = fact(cast_ptr, lc, rc, mem)?;
                self.data.set(prod.as_untyped(mem));
            },
            Fold => {
                let is_nat = decode_i(op);

                if is_nat == 0 {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<Nat>>(mem)
                    };

                    let new_val = fold_nat(cast_ptr, mem)?;
                    self.data.set(new_val.as_untyped(mem));
                } else {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<()>>(mem)
                    };

                    let new_val = fold(cast_ptr, mem)?;
                    self.data.set(new_val.as_untyped(mem));
                }
            },
            Unfold => {
                let is_nat = decode_i(op);

                if is_nat == 0 {
                    let cast_ptr = unsafe {
                        data.cast::<Nat>(mem)
                    };

                    let new_val = unfold_nat(cast_ptr, mem)?;
                    self.data.set(new_val.as_untyped(mem));
                } else {
                    let cast_ptr = unsafe {
                        data.cast::<Inductive<()>>(mem)
                    };

                    let new_val = unfold(cast_ptr, mem)?;
                    self.data.set(new_val.as_untyped(mem));
                }
            },
            Tx => {}, // TODO: add coinduction
            Rx => {}, // TODO: add coinduction
            Expn => {
                let div = decode_i(op);
                if cont.direction() {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<()>>(mem)
                    };

                    let new = expn(cast_ptr, div, mem)?;
                    self.data.set(new.as_untyped(mem));
                    cont.reverse();
                } else {
                    return Err(RuntimeError::new(ErrorKind::ExpectedZero));
                }
            },
            Coln => {
                let div = decode_i(op);
                if !cont.direction() {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<()>>(mem)
                    };

                    // expn and coln are basically the same function, only
                    // one runs forwards and the other backwards
                    let new = expn(cast_ptr, div, mem)?;
                    self.data.set(new.as_untyped(mem));
                    cont.reverse();
                } else {
                    return Err(RuntimeError::new(ErrorKind::ExpectedZero));
                }
            },
            Expf => {}, // TODO: reimplement
            Colf => {}, // TODO: reimplement
            Call => {
                let dir = cont.direction();
                let not = if !dir { false } else { true };
                let new_cxt = Context::Call {
                    not,
                    ret: if dir { cont.ip() - 1 } else { cont.ip() + 1 },
                };
                
                let cast_arg = unsafe {
                    arg.cast::<Sum<Product<Nat, Nat>>>(mem)
                };
                let start_end = cast_arg.data(mem);
                let start = start_end.fst(mem);
                let end = start_end.snd(mem);

                self.call_func(mem, *start, *end, not);
                cxt_stack.push(mem, new_cxt)?;
            },
            Uncall => {
                let dir = cont.direction();
                let not = if dir { false } else { true };
                let new_cxt = Context::Call {
                    not,
                    ret: if dir { cont.ip() - 1 } else { cont.ip() + 1 },
                };

                let cast_arg = unsafe {
                    arg.cast::<Sum<Product<Nat, Nat>>>(mem)
                };
                let start_end = cast_arg.data(mem);
                let start = start_end.fst(mem);
                let end = start_end.snd(mem);

                self.call_func(mem, *start, *end, not);
                cxt_stack.push(mem, new_cxt)?;
            },
            Start => {}, // op-equivalent to ID
            End => {
                match cxt_stack.top(mem)? {
                    Context::Call { not, ret } => {
                        if not { cont.reverse(); }
                        cont.set_ip(ret);
                    },
                    Context::Nil => return Ok(EvalStatus::Ok),
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            StartSum => {
                let div = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };
                let cast_arg = unsafe {
                    arg.cast::<Sum<Product<Nat, Nat>>>(mem)
                };
                let lc_rc = cast_arg.data(mem);
                let lc = lc_rc.fst(mem);
                let rc = lc_rc.snd(mem);

                if cast_ptr.tag() < div as u32 {
                    if !cont.direction() {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() + (*lc + 1),
                            jump: *rc,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt)?;
                        self.data.set(cast_ptr.data(mem));
                    } else {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() - *rc,
                            jump: *rc,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cont.jump(*rc + 1); // ip - rc+1
                        cxt_stack.push(mem, new_cxt)?;
                        self.data.set(cast_ptr.data(mem));
                    }
                } else {
                    if !cont.direction() {
                        let new_cxt = Context::Right {
                            left_op_index: cont.ip() + *lc,
                            jump: *lc,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cont.jump(*lc + 1); // ip + lc+1
                        cxt_stack.push(mem, new_cxt)?;
                        self.data.set(cast_ptr.data(mem));
                    } else {
                        let new_cxt = Context::Right {
                            left_op_index: cont.ip() - (*rc + 1),
                            jump: *lc,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt)?;
                        self.data.set(cast_ptr.data(mem));
                    }
                }
            }
            SplitSum => {} // TODO: Implement
            EndSum => {
                let sum_cxt = cxt_stack.pop(mem)?;

                match sum_cxt {
                    Context::Left { root_val, .. } => {
                        root_val.get(mem).set_data(self.data.get(mem));
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    Context::Right { root_val, .. } => {
                        root_val.get(mem).set_data(self.data.get(mem));
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            StartProd => {
                let cast_ptr = unsafe { data.cast::<Product<(), ()>>(mem) };
                let cast_arg = unsafe { arg.cast::<Sum<Nat>>(mem) };
                let jmp = cast_arg.data(mem);

                if !cont.direction() {
                    let new_cxt = Context::First {
                        snd_op_index: *jmp + cont.ip(),
                        snd_val: CellPtr::new_with(cast_ptr.snd(mem)),
                        root_val: CellPtr::new_with(cast_ptr),
                    };

                    cxt_stack.push(mem, new_cxt)?;
                    self.data.set(cast_ptr.fst(mem));
                } else {
                    let new_cxt = Context::Second {
                        fst_op_index: *jmp + cont.ip(),
                        fst_val: CellPtr::new_with(cast_ptr.fst(mem)),
                        root_val: CellPtr::new_with(cast_ptr),
                    };

                    cxt_stack.push(mem, new_cxt)?;
                    self.data.set(cast_ptr.snd(mem));
                }
            }
            SplitProd => {} // TODO: Implement
            EndProd => {
                let sum_cxt = cxt_stack.pop(mem)?;

                match sum_cxt {
                    Context::First { root_val, .. } => {
                        root_val.get(mem).set_fst(self.data.get(mem));
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    Context::Second { root_val, .. } => {
                        root_val.get(mem).set_snd(self.data.get(mem));
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            _ => {},
        }

        Ok(EvalStatus::Pending)
    }

    pub fn data(&self) -> &UntypedCellPtr { &self.data }
}
