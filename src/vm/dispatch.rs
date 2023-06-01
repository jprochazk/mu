#[macro_use]
mod macros;

use std::error::Error as StdError;
use std::ptr::NonNull;

use crate::bytecode::opcode as op;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::operands::Width;
use crate::Result;

use super::thread::AsyncFrame;

pub fn dispatch<T: Handler>(
  handler: &mut T,
  bytecode: NonNull<[u8]>,
  pc: usize,
) -> Result<ControlFlow, T::Error> {
  let mut bytecode = bytecode;
  let mut pc = pc;

  'load_frame: loop {
    let ip = bytecode.as_ptr() as *mut u8;
    if pc >= bytecode.len() {
      panic!(
        "unexpected end of bytecode stream (pc={}, len={})",
        pc,
        bytecode.len()
      );
    }
    let end = unsafe { ip.add(bytecode.len()) };
    let mut ip = unsafe { ip.add(pc) };
    let mut width = Width::Normal;

    loop {
      handler.print_stack();
      let start = ip;
      match read_opcode!(ip, end) {
        Opcode::Nop => {
          println!("nop");
          continue;
        }
        Opcode::Wide16 => {
          print!("wide16.");
          width = Width::Wide16;
          continue;
        }
        Opcode::Wide32 => {
          print!("wide32.");
          width = Width::Wide32;
          continue;
        }
        Opcode::Load => {
          let (reg,) = read_operands!(Load, ip, end, width);
          println!("load {reg}");
          handler.op_load(reg)?;
          continue;
        }
        Opcode::Store => {
          let (reg,) = read_operands!(Store, ip, end, width);
          println!("store {reg}");
          handler.op_store(reg)?;
          continue;
        }
        Opcode::LoadConst => {
          let (idx,) = read_operands!(LoadConst, ip, end, width);
          println!("load_const {idx}");
          handler.op_load_const(idx)?;
          continue;
        }
        Opcode::LoadUpvalue => {
          let (idx,) = read_operands!(LoadUpvalue, ip, end, width);
          println!("load_upvalue {idx}");
          handler.op_load_upvalue(idx)?;
          continue;
        }
        Opcode::StoreUpvalue => {
          let (idx,) = read_operands!(StoreUpvalue, ip, end, width);
          println!("store_upvalue {idx}");
          handler.op_store_upvalue(idx)?;
          continue;
        }
        Opcode::LoadModuleVar => {
          let (idx,) = read_operands!(LoadModuleVar, ip, end, width);
          println!("load_module_var {idx}");
          handler.op_load_module_var(idx)?;
          continue;
        }
        Opcode::StoreModuleVar => {
          let (idx,) = read_operands!(StoreModuleVar, ip, end, width);
          println!("store_module_var {idx}");
          handler.op_store_module_var(idx)?;
          continue;
        }
        Opcode::LoadGlobal => {
          let (name,) = read_operands!(LoadGlobal, ip, end, width);
          println!("load_global {name}");
          handler.op_load_global(name)?;
          continue;
        }
        Opcode::StoreGlobal => {
          let (name,) = read_operands!(StoreGlobal, ip, end, width);
          println!("store_global {name}");
          handler.op_store_global(name)?;
          continue;
        }
        Opcode::LoadField => {
          let (name,) = read_operands!(LoadField, ip, end, width);
          println!("load_field {name}");
          handler.op_load_field(name)?;
          continue;
        }
        Opcode::LoadFieldOpt => {
          let (name,) = read_operands!(LoadFieldOpt, ip, end, width);
          println!("load_field_opt {name}");
          handler.op_load_field_opt(name)?;
          continue;
        }
        Opcode::StoreField => {
          let (obj, name) = read_operands!(StoreField, ip, end, width);
          println!("store_field {obj} {name}");
          handler.op_store_field(obj, name)?;
          continue;
        }
        Opcode::LoadIndex => {
          let (name,) = read_operands!(LoadIndex, ip, end, width);
          println!("load_index {name}");
          handler.op_load_index(name)?;
          continue;
        }
        Opcode::LoadIndexOpt => {
          let (name,) = read_operands!(LoadIndexOpt, ip, end, width);
          println!("load_index_opt {name}");
          handler.op_load_index_opt(name)?;
          continue;
        }
        Opcode::StoreIndex => {
          let (obj, key) = read_operands!(StoreIndex, ip, end, width);
          println!("store_index {obj} {key}");
          handler.op_store_index(obj, key)?;
          continue;
        }
        Opcode::LoadSelf => {
          let () = read_operands!(LoadSelf, ip, end, width);
          println!("load_self");
          handler.op_load_self()?;
          continue;
        }
        Opcode::LoadSuper => {
          let () = read_operands!(LoadSuper, ip, end, width);
          println!("load_super");
          handler.op_load_super()?;
          continue;
        }
        Opcode::LoadNone => {
          let () = read_operands!(LoadNone, ip, end, width);
          println!("load_none");
          handler.op_load_none()?;
          continue;
        }
        Opcode::LoadTrue => {
          let () = read_operands!(LoadTrue, ip, end, width);
          println!("load_true");
          handler.op_load_true()?;
          continue;
        }
        Opcode::LoadFalse => {
          let () = read_operands!(LoadFalse, ip, end, width);
          println!("load_false");
          handler.op_load_false()?;
          continue;
        }
        Opcode::LoadSmi => {
          let (smi,) = read_operands!(LoadSmi, ip, end, width);
          println!("load_smi {smi}");
          handler.op_load_smi(smi)?;
          continue;
        }
        Opcode::MakeFn => {
          let (desc,) = read_operands!(MakeFn, ip, end, width);
          println!("make_fn {desc}");
          handler.op_make_fn(desc)?;
          continue;
        }
        Opcode::MakeClass => {
          let (desc,) = read_operands!(MakeClass, ip, end, width);
          println!("make_class {desc}");
          handler.op_make_class(desc)?;
          continue;
        }
        Opcode::MakeClassDerived => {
          let (desc,) = read_operands!(MakeClassDerived, ip, end, width);
          println!("make_class_derived {desc}");
          handler.op_make_class_derived(desc)?;
          continue;
        }
        Opcode::MakeDataClass => {
          let (desc, parts) = read_operands!(MakeDataClass, ip, end, width);
          println!("make_data_class {desc} {parts}");
          handler.op_make_data_class(desc, parts)?;
          continue;
        }
        Opcode::MakeDataClassDerived => {
          let (desc, parts) = read_operands!(MakeDataClassDerived, ip, end, width);
          println!("make_data_class_derived {desc} {parts}");
          handler.op_make_data_class_derived(desc, parts)?;
          continue;
        }
        Opcode::FinalizeClass => {
          let () = read_operands!(FinalizeClass, ip, end, width);
          println!("finalize_class");
          handler.op_finalize_class()?;
          continue;
        }
        Opcode::MakeList => {
          let (start, count) = read_operands!(MakeList, ip, end, width);
          println!("make_list {start} {count}");
          handler.op_make_list(start, count)?;
          continue;
        }
        Opcode::MakeListEmpty => {
          let () = read_operands!(MakeListEmpty, ip, end, width);
          println!("make_list_empty");
          handler.op_make_list_empty()?;
          continue;
        }
        Opcode::MakeTable => {
          let (start, count) = read_operands!(MakeTable, ip, end, width);
          println!("make_table {start} {count}");
          handler.op_make_table(start, count)?;
          continue;
        }
        Opcode::MakeTableEmpty => {
          let () = read_operands!(MakeTableEmpty, ip, end, width);
          println!("make_table_empty");
          handler.op_make_table_empty()?;
          continue;
        }
        Opcode::Jump => {
          #[allow(unused_assignments)] // ip is overwritten by start+offset
          let (offset,) = read_operands!(Jump, ip, end, width);
          println!("jump {offset}");
          let offset = handler.op_jump(offset)?;
          unsafe { ip = start.add(offset.value()) };
          continue;
        }
        Opcode::JumpConst => {
          #[allow(unused_assignments)] // ip is overwritten by start+offset
          let (idx,) = read_operands!(JumpConst, ip, end, width);
          println!("jump_const {idx}");
          let offset = handler.op_jump_const(idx)?;
          unsafe { ip = start.add(offset.value()) };
          continue;
        }
        Opcode::JumpLoop => {
          #[allow(unused_assignments)] // ip is overwritten by start-offset
          let (offset,) = read_operands!(JumpLoop, ip, end, width);
          println!("jump_loop {offset}");
          let offset = handler.op_jump_loop(offset)?;
          unsafe { ip = start.sub(offset.value()) }
          continue;
        }
        Opcode::JumpIfFalse => {
          let (offset,) = read_operands!(JumpIfFalse, ip, end, width);
          println!("jump_if_false {offset}");
          let offset = handler.op_jump_if_false(offset)?;
          match offset {
            Jump::Move(offset) => unsafe { ip = start.add(offset.value()) },
            Jump::Skip => {}
          }
          continue;
        }
        Opcode::JumpIfFalseConst => {
          let (idx,) = read_operands!(JumpIfFalseConst, ip, end, width);
          println!("jump_if_false {idx}");
          let offset = handler.op_jump_if_false_const(idx)?;
          match offset {
            Jump::Move(offset) => unsafe { ip = start.add(offset.value()) },
            Jump::Skip => {}
          }
          continue;
        }
        Opcode::Add => {
          let (lhs,) = read_operands!(Add, ip, end, width);
          println!("add {lhs}");
          handler.op_add(lhs)?;
          continue;
        }
        Opcode::Sub => {
          let (lhs,) = read_operands!(Sub, ip, end, width);
          println!("sub {lhs}");
          handler.op_sub(lhs)?;
          continue;
        }
        Opcode::Mul => {
          let (lhs,) = read_operands!(Mul, ip, end, width);
          println!("mul {lhs}");
          handler.op_mul(lhs)?;
          continue;
        }
        Opcode::Div => {
          let (lhs,) = read_operands!(Div, ip, end, width);
          println!("div {lhs}");
          handler.op_div(lhs)?;
          continue;
        }
        Opcode::Rem => {
          let (lhs,) = read_operands!(Rem, ip, end, width);
          println!("rem {lhs}");
          handler.op_rem(lhs)?;
          continue;
        }
        Opcode::Pow => {
          let (lhs,) = read_operands!(Pow, ip, end, width);
          println!("pow {lhs}");
          handler.op_pow(lhs)?;
          continue;
        }
        Opcode::Inv => {
          let () = read_operands!(Inv, ip, end, width);
          println!("inv");
          handler.op_inv()?;
          continue;
        }
        Opcode::Not => {
          let () = read_operands!(Not, ip, end, width);
          println!("not");
          handler.op_not()?;
          continue;
        }
        Opcode::CmpEq => {
          let (lhs,) = read_operands!(CmpEq, ip, end, width);
          println!("cmp_eq {lhs}");
          handler.op_cmp_eq(lhs)?;
          continue;
        }
        Opcode::CmpNe => {
          let (lhs,) = read_operands!(CmpNe, ip, end, width);
          println!("cmp_ne {lhs}");
          handler.op_cmp_ne(lhs)?;
          continue;
        }
        Opcode::CmpGt => {
          let (lhs,) = read_operands!(CmpGt, ip, end, width);
          println!("cmp_gt {lhs}");
          handler.op_cmp_gt(lhs)?;
          continue;
        }
        Opcode::CmpGe => {
          let (lhs,) = read_operands!(CmpGe, ip, end, width);
          println!("cmp_ge {lhs}");
          handler.op_cmp_ge(lhs)?;
          continue;
        }
        Opcode::CmpLt => {
          let (lhs,) = read_operands!(CmpLt, ip, end, width);
          println!("cmp_lt {lhs}");
          handler.op_cmp_lt(lhs)?;
          continue;
        }
        Opcode::CmpLe => {
          let (lhs,) = read_operands!(CmpLe, ip, end, width);
          println!("cmp_le {lhs}");
          handler.op_cmp_le(lhs)?;
          continue;
        }
        Opcode::CmpType => {
          let (lhs,) = read_operands!(CmpType, ip, end, width);
          println!("cmp_type {lhs}");
          handler.op_cmp_type(lhs)?;
          continue;
        }
        Opcode::Contains => {
          let (lhs,) = read_operands!(Contains, ip, end, width);
          println!("contains {lhs}");
          handler.op_contains(lhs)?;
          continue;
        }
        Opcode::IsNone => {
          let () = read_operands!(IsNone, ip, end, width);
          println!("is_none");
          handler.op_is_none()?;
          continue;
        }
        Opcode::Print => {
          let () = read_operands!(Print, ip, end, width);
          println!("print");
          handler.op_print()?;
          continue;
        }
        Opcode::PrintN => {
          let (start, count) = read_operands!(PrintN, ip, end, width);
          println!("print_n {start} {count}");
          handler.op_print_n(start, count)?;
          continue;
        }
        Opcode::Call => {
          // frame is reloaded so neither `ip` nor `width` are read
          #[allow(unused_assignments)]
          let (callee, args) = read_operands!(Call, ip, end, width);
          println!("call {callee} {args}");
          let return_addr = get_pc!(ip, bytecode);
          match handler.op_call(return_addr, callee, args)? {
            Call::LoadFrame(new_frame) => {
              bytecode = new_frame.bytecode;
              pc = new_frame.pc;
              continue 'load_frame;
            }
            Call::Continue => continue,
            Call::Poll(frame) => {
              return Ok(ControlFlow::Poll(Poll {
                frame,
                pc: get_pc!(ip, bytecode),
              }))
            }
          }
        }
        Opcode::Call0 => {
          // frame is reloaded so neither `ip` nor `width` are read
          #[allow(unused_assignments)]
          let () = read_operands!(Call0, ip, end, width);
          println!("call0");
          let return_addr = get_pc!(ip, bytecode);
          match handler.op_call0(return_addr)? {
            Call::LoadFrame(new_frame) => {
              bytecode = new_frame.bytecode;
              pc = new_frame.pc;
              continue 'load_frame;
            }
            Call::Continue => continue,
            Call::Poll(frame) => {
              return Ok(ControlFlow::Poll(Poll {
                frame,
                pc: get_pc!(ip, bytecode),
              }))
            }
          }
        }
        Opcode::Import => {
          let (path,) = read_operands!(Import, ip, end, width);
          println!("import {path}");
          let return_addr = get_pc!(ip, bytecode);
          match handler.op_import(path, return_addr)? {
            Call::LoadFrame(new_frame) => {
              bytecode = new_frame.bytecode;
              pc = new_frame.pc;
              continue 'load_frame;
            }
            Call::Continue => continue,
            Call::Poll(frame) => {
              return Ok(ControlFlow::Poll(Poll {
                frame,
                pc: get_pc!(ip, bytecode),
              }))
            }
          }
        }
        Opcode::FinalizeModule => {
          let () = read_operands!(FinalizeModule, ip, end, width);
          println!("finalize_module");
          handler.op_finalize_module()?;
          continue;
        }
        Opcode::Return => {
          #[allow(unused_assignments)] // ip is overwritten by start+offset
          let () = read_operands!(Return, ip, end, width);
          println!("return");
          match handler.op_return()? {
            Return::LoadFrame(new_frame) => {
              bytecode = new_frame.bytecode;
              pc = new_frame.pc;
              continue 'load_frame;
            }
            Return::Yield => return Ok(ControlFlow::Return),
          };
        }
        Opcode::Yield => {
          #[allow(unused_assignments)] // ip is overwritten by start+offset
          let () = read_operands!(Yield, ip, end, width);
          println!("yield");
          handler.op_yield()?;
          return Ok(ControlFlow::Yield(get_pc!(ip, bytecode)));
        }
      }
    }
  }
}

pub struct Poll {
  pub frame: AsyncFrame,
  pub pc: usize,
}

pub enum ControlFlow {
  Yield(usize),
  Poll(Poll),
  // TODO: is a separate `Return` needed?
  Return,
}

#[must_use]
pub enum Jump {
  Skip,
  Move(op::Offset),
}

pub struct LoadFrame {
  pub bytecode: NonNull<[u8]>,
  pub pc: usize,
}

#[must_use]
pub enum Call {
  LoadFrame(LoadFrame),
  Poll(AsyncFrame),
  Continue,
}

#[must_use]
pub enum Return {
  LoadFrame(LoadFrame),
  Yield,
}

impl From<LoadFrame> for Return {
  fn from(value: LoadFrame) -> Self {
    Self::LoadFrame(value)
  }
}

pub trait Handler {
  type Error: StdError;

  fn print_stack(&self);

  fn op_load(&mut self, reg: op::Register) -> Result<(), Self::Error>;
  fn op_store(&mut self, reg: op::Register) -> Result<(), Self::Error>;
  fn op_load_const(&mut self, idx: op::Constant) -> Result<(), Self::Error>;
  fn op_load_upvalue(&mut self, idx: op::Upvalue) -> Result<(), Self::Error>;
  fn op_store_upvalue(&mut self, idx: op::Upvalue) -> Result<(), Self::Error>;
  fn op_load_module_var(&mut self, idx: op::ModuleVar) -> Result<(), Self::Error>;
  fn op_store_module_var(&mut self, idx: op::ModuleVar) -> Result<(), Self::Error>;
  fn op_load_global(&mut self, name: op::Constant) -> Result<(), Self::Error>;
  fn op_store_global(&mut self, name: op::Constant) -> Result<(), Self::Error>;
  fn op_load_field(&mut self, name: op::Constant) -> Result<(), Self::Error>;
  fn op_load_field_opt(&mut self, name: op::Constant) -> Result<(), Self::Error>;
  fn op_store_field(&mut self, obj: op::Register, name: op::Constant) -> Result<(), Self::Error>;
  fn op_load_index(&mut self, obj: op::Register) -> Result<(), Self::Error>;
  fn op_load_index_opt(&mut self, obj: op::Register) -> Result<(), Self::Error>;
  fn op_store_index(&mut self, obj: op::Register, key: op::Register) -> Result<(), Self::Error>;
  fn op_load_self(&mut self) -> Result<(), Self::Error>;
  fn op_load_super(&mut self) -> Result<(), Self::Error>;
  fn op_load_none(&mut self) -> Result<(), Self::Error>;
  fn op_load_true(&mut self) -> Result<(), Self::Error>;
  fn op_load_false(&mut self) -> Result<(), Self::Error>;
  fn op_load_smi(&mut self, smi: op::Smi) -> Result<(), Self::Error>;
  fn op_make_fn(&mut self, desc: op::Constant) -> Result<(), Self::Error>;
  fn op_make_class(&mut self, desc: op::Constant) -> Result<(), Self::Error>;
  fn op_make_class_derived(&mut self, desc: op::Constant) -> Result<(), Self::Error>;
  fn op_make_data_class(
    &mut self,
    desc: op::Constant,
    parts: op::Register,
  ) -> Result<(), Self::Error>;
  fn op_make_data_class_derived(
    &mut self,
    desc: op::Constant,
    parts: op::Register,
  ) -> Result<(), Self::Error>;
  fn op_finalize_class(&mut self) -> Result<(), Self::Error>;
  fn op_make_list(&mut self, start: op::Register, count: op::Count) -> Result<(), Self::Error>;
  fn op_make_list_empty(&mut self) -> Result<(), Self::Error>;
  fn op_make_table(&mut self, start: op::Register, count: op::Count) -> Result<(), Self::Error>;
  fn op_make_table_empty(&mut self) -> Result<(), Self::Error>;
  fn op_jump(&mut self, offset: op::Offset) -> Result<op::Offset, Self::Error>;
  fn op_jump_const(&mut self, idx: op::Constant) -> Result<op::Offset, Self::Error>;
  fn op_jump_loop(&mut self, offset: op::Offset) -> Result<op::Offset, Self::Error>;
  fn op_jump_if_false(&mut self, offset: op::Offset) -> Result<Jump, Self::Error>;
  fn op_jump_if_false_const(&mut self, idx: op::Constant) -> Result<Jump, Self::Error>;
  fn op_add(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_sub(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_mul(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_div(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_rem(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_pow(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_inv(&mut self) -> Result<(), Self::Error>;
  fn op_not(&mut self) -> Result<(), Self::Error>;
  fn op_cmp_eq(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_ne(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_gt(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_ge(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_lt(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_le(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_cmp_type(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_contains(&mut self, lhs: op::Register) -> Result<(), Self::Error>;
  fn op_is_none(&mut self) -> Result<(), Self::Error>;
  fn op_print(&mut self) -> Result<(), Self::Error>;
  fn op_print_n(&mut self, start: op::Register, count: op::Count) -> Result<(), Self::Error>;
  fn op_call(
    &mut self,
    return_addr: usize,
    callee: op::Register,
    args: op::Count,
  ) -> Result<Call, Self::Error>;
  fn op_call0(&mut self, return_addr: usize) -> Result<Call, Self::Error>;
  fn op_import(&mut self, path: op::Constant, return_addr: usize) -> Result<Call, Self::Error>;
  fn op_finalize_module(&mut self) -> Result<(), Self::Error>;
  fn op_return(&mut self) -> Result<Return, Self::Error>;
  fn op_yield(&mut self) -> Result<(), Self::Error>;
}
