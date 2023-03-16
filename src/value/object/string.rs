use std::borrow::Borrow;
use std::fmt::Display;

use beef::lean::Cow;
use derive::delegate_to_handle;
use indexmap::Equivalent;

use super::Access;
use crate::ctx::Context;
use crate::value::handle::Handle;
use crate::value::Value;
use crate::Result;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Str(String);

#[delegate_to_handle]
impl Str {
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}

impl<'a> From<Cow<'a, str>> for Str {
  fn from(value: Cow<'a, str>) -> Self {
    Self(value.to_string())
  }
}

impl From<String> for Str {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl<'a> From<&'a str> for Str {
  fn from(value: &'a str) -> Self {
    Self(value.to_string())
  }
}

impl Access for Str {
  fn field_get(&self, _: &Context, key: &str) -> Result<Option<Value>> {
    Ok(match key {
      "len" => Some(Value::int(self.0.len() as i32)),
      _ => None,
    })
  }
}

impl Display for Str {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"{}\"", self.0)
  }
}

impl Borrow<str> for Str {
  fn borrow(&self) -> &str {
    self.as_str()
  }
}

impl Borrow<str> for Handle<Str> {
  fn borrow(&self) -> &str {
    unsafe { self._get() }.as_str()
  }
}

impl<'a> Equivalent<&'a str> for Handle<Str> {
  fn equivalent(&self, key: &&'a str) -> bool {
    unsafe { self._get() }.as_str() == *key
  }
}

#[derive::builtin]
impl Str {
  pub fn upper(&self) -> String {
    self.0.to_uppercase()
  }

  pub fn lower(&self) -> String {
    self.0.to_lowercase()
  }

  pub fn concat(&self, other: &str) -> String {
    let mut out = String::with_capacity(self.as_str().len() + other.len());
    out.push_str(self.as_str());
    out.push_str(other);
    out
  }
}
