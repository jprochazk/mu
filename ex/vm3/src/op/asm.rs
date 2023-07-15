use super::{u24, Const, Count, Mvar, Offset, Reg, Smi};

macro_rules! _asm {
  ($inst:ident $(, $($arg:ident : $ty:ident<$g:ident>),*)?) => {
    #[allow(non_camel_case_types)]
    pub fn $inst ($($($arg : $ty<impl Into<$g>>),*)?) -> super::Op {
      $( $( let $arg = $ty($arg.0.into()); )* )?

      paste::paste!(
        super::Op::[<$inst:camel>] $({
          $($arg),*
        })?
      )
    }
  };
}

_asm!(nop);
_asm!(mov,                   src: Reg<u8>, dst: Reg<u8>);
_asm!(load_const,            dst: Reg<u8>, idx: Const<u16>);
_asm!(load_upvalue,          dst: Reg<u8>, idx: Const<u16>);
_asm!(set_upvalue,           src: Reg<u8>, idx: Const<u16>);
_asm!(load_mvar,            dst: Reg<u8>, idx: Mvar<u16>);
_asm!(set_mvar,             src: Reg<u8>, idx: Mvar<u16>);
_asm!(load_global,           dst: Reg<u8>, name: Const<u16>);
_asm!(set_global,            reg: Reg<u8>, name: Const<u16>);
_asm!(load_field_reg,        obj: Reg<u8>, name: Reg<u8>, reg: Reg<u8>);
_asm!(load_field_const,      obj: Reg<u8>, name: Const<u8>, reg: Reg<u8>);
_asm!(load_field_opt_reg,    obj: Reg<u8>, name: Reg<u8>, reg: Reg<u8>);
_asm!(load_field_opt_const,  obj: Reg<u8>, name: Const<u8>, reg: Reg<u8>);
_asm!(set_field,             obj: Reg<u8>, name: Reg<u8>, reg: Reg<u8>);
_asm!(load_index,            obj: Reg<u8>, key: Reg<u8>, reg: Reg<u8>);
_asm!(load_index_opt,        obj: Reg<u8>, key: Reg<u8>, reg: Reg<u8>);
_asm!(set_index,             obj: Reg<u8>, key: Reg<u8>, reg: Reg<u8>);
_asm!(load_super,            dst: Reg<u8>);
_asm!(load_none,             dst: Reg<u8>);
_asm!(load_true,             dst: Reg<u8>);
_asm!(load_false,            dst: Reg<u8>);
_asm!(load_smi,              dst: Reg<u8>, value: Smi<i16>);
_asm!(make_fn,               dst: Reg<u8>, desc: Const<u16>);
_asm!(make_class,            dst: Reg<u8>, desc: Const<u16>);
_asm!(make_class_derived,    dst: Reg<u8>, desc: Const<u16>);
_asm!(make_list,             dst: Reg<u8>, desc: Const<u16>);
_asm!(make_list_empty,       dst: Reg<u8>);
_asm!(make_table,            dst: Reg<u8>, desc: Const<u16>);
_asm!(make_table_empty,      dst: Reg<u8>);
_asm!(jump,                  offset: Offset<u24>);
_asm!(jump_const,            offset: Const<u24>);
_asm!(jump_loop,             offset: Offset<u24>);
_asm!(jump_loop_const,       offset: Const<u24>);
_asm!(jump_if_false,         offset: Offset<u24>);
_asm!(jump_if_false_const,   offset: Const<u24>);
_asm!(add,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(sub,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(mul,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(div,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(rem,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(pow,                   dst: Reg<u8>, lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(inv,                   val: Reg<u8>);
_asm!(not,                   val: Reg<u8>);
_asm!(cmp_eq,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_ne,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_gt,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_ge,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_lt,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_le,                lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(cmp_type,              lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(contains,              lhs: Reg<u8>, rhs: Reg<u8>);
_asm!(is_none,               val: Reg<u8>);
_asm!(call,                  func: Reg<u8>, count: Count<u8>);
_asm!(call0,                 func: Reg<u8>);
_asm!(import,                dst: Reg<u8>, path: Const<u16>);
_asm!(finalize_module);
_asm!(ret,                   val: Reg<u8>);
_asm!(yld,                   val: Reg<u8>);
