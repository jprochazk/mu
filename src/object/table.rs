use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use indexmap::{Equivalent, IndexMap};

use super::ptr::Ptr;
use super::{Object, String};
use crate::value::Value;

pub struct Table {
  data: RefCell<IndexMap<Ptr<String>, Value>>,
}

impl Table {
  pub fn with_capacity(n: usize) -> Self {
    Self {
      data: RefCell::new(IndexMap::with_capacity(n)),
    }
  }

  pub fn insert(&self, key: Ptr<String>, value: Value) {
    self.data.borrow_mut().insert(key, value);
  }

  pub fn get<K: Equivalent<Ptr<String>> + ?Sized + Hash>(&self, key: &K) -> Option<Value> {
    self.data.borrow().get(key).cloned()
  }
}

impl Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<table>")
  }
}

impl Debug for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = f.debug_struct("Table");
    for (key, value) in self.data.borrow().iter() {
      s.field(key, value);
    }
    s.finish()
  }
}

impl Object for Table {
  fn type_name(&self) -> &'static str {
    "Table"
  }
}
