pub mod emit;

mod ux;
use ux::u24;

use crate::util::static_assert_size;

/*
codegen notes:
- for constant indices stored as `u8`, the constant can
  first be loaded into a register, and then used, because
  `LoadConst` stores the constant index as `u16`, allowing
  a much greater range
*/

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg<T>(pub T);

impl<T: Into<usize>> Reg<T> {
  pub fn wide(self) -> usize {
    self.0.into()
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Const<T>(pub T);

impl<T: Into<usize>> Const<T> {
  pub fn wide(self) -> usize {
    self.0.into()
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset<T>(pub T);

impl<T: Into<usize>> Offset<T> {
  pub fn wide(self) -> usize {
    self.0.into()
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Count<T>(pub T);

impl<T: Into<usize>> Count<T> {
  pub fn wide(self) -> usize {
    self.0.into()
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mvar<T>(pub T);

impl<T: Into<usize>> Mvar<T> {
  pub fn wide(self) -> usize {
    self.0.into()
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Smi<T>(pub T);

impl<T: Into<i32>> Smi<T> {
  pub fn wide(self) -> i32 {
    self.0.into()
  }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Op {
  Nop,
  Mov {
    src: Reg<u8>,
    dst: Reg<u8>,
  },
  LoadConst {
    dst: Reg<u8>,
    idx: Const<u16>,
  },
  LoadUpvalue {
    dst: Reg<u8>,
    idx: Const<u16>,
  },
  SetUpvalue {
    src: Reg<u8>,
    idx: Const<u16>,
  },
  LoadMvar {
    dst: Reg<u8>,
    idx: Mvar<u16>,
  },
  SetMvar {
    src: Reg<u8>,
    idx: Mvar<u16>,
  },
  LoadGlobal {
    dst: Reg<u8>,
    name: Const<u16>,
  },
  SetGlobal {
    reg: Reg<u8>,
    name: Const<u16>,
  },
  LoadFieldReg {
    obj: Reg<u8>,
    name: Reg<u8>,
    reg: Reg<u8>,
  },
  LoadFieldConst {
    obj: Reg<u8>,
    name: Const<u8>,
    reg: Reg<u8>,
  },
  LoadFieldOptReg {
    obj: Reg<u8>,
    name: Reg<u8>,
    reg: Reg<u8>,
  },
  LoadFieldOptConst {
    obj: Reg<u8>,
    name: Const<u8>,
    reg: Reg<u8>,
  },
  SetField {
    obj: Reg<u8>,
    name: Reg<u8>,
    reg: Reg<u8>,
  },
  LoadIndex {
    obj: Reg<u8>,
    key: Reg<u8>,
    reg: Reg<u8>,
  },
  LoadIndexOpt {
    obj: Reg<u8>,
    key: Reg<u8>,
    reg: Reg<u8>,
  },
  SetIndex {
    obj: Reg<u8>,
    key: Reg<u8>,
    reg: Reg<u8>,
  },
  LoadSuper {
    dst: Reg<u8>,
  },
  LoadNone {
    dst: Reg<u8>,
  },
  LoadTrue {
    dst: Reg<u8>,
  },
  LoadFalse {
    dst: Reg<u8>,
  },
  LoadSmi {
    dst: Reg<u8>,
    value: Smi<i16>,
  },
  MakeFn {
    dst: Reg<u8>,
    desc: Const<u16>,
  },
  MakeClass {
    dst: Reg<u8>,
    desc: Const<u16>,
  },
  MakeClassDerived {
    dst: Reg<u8>,
    desc: Const<u16>,
  },
  MakeList {
    dst: Reg<u8>,
    desc: Const<u16>,
  },
  MakeListEmpty {
    dst: Reg<u8>,
  },
  MakeTable {
    dst: Reg<u8>,
    desc: Const<u16>,
  },
  MakeTableEmpty {
    dst: Reg<u8>,
  },
  Jump {
    offset: Offset<u24>,
  },
  JumpConst {
    offset: Const<u24>,
  },
  JumpLoop {
    offset: Offset<u24>,
  },
  JumpLoopConst {
    offset: Const<u24>,
  },
  JumpIfFalse {
    offset: Offset<u24>,
  },
  JumpIfFalseConst {
    offset: Const<u24>,
  },
  Add {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Sub {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Mul {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Div {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Rem {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Pow {
    dst: Reg<u8>,
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Inv {
    val: Reg<u8>,
  },
  Not {
    val: Reg<u8>,
  },
  CmpEq {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpNe {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpGt {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpGe {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpLt {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpLe {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  CmpType {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  Contains {
    lhs: Reg<u8>,
    rhs: Reg<u8>,
  },
  IsNone {
    val: Reg<u8>,
  },
  Call {
    func: Reg<u8>,
    count: Count<u8>,
  },
  Call0 {
    func: Reg<u8>,
  },
  Import {
    dst: Reg<u8>,
    path: Const<u16>,
  },
  FinalizeModule,
  Ret {
    val: Reg<u8>,
  },
  Yld {
    val: Reg<u8>,
  },
}

const _: () = static_assert_size::<Op>(4, "expected a size of 4 bytes");

#[rustfmt::skip]
pub mod asm;
