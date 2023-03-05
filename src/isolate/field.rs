use crate::ctx::Context;
use crate::value::handle::Handle;
use crate::value::object::{Access, Method, Str};
use crate::value::Value;
use crate::{Error, Result};

pub fn set(ctx: Context, receiver: &mut Value, key: Handle<Str>, value: Value) -> Result<()> {
  if let Some(mut obj) = receiver.clone().to_object_raw() {
    if obj.field_get(key.as_str())?.is_some() || !obj.is_frozen() {
      obj.field_set(key, value)?;
      return Ok(());
    }
  };

  Err(Error::runtime(format!(
    "cannot set field `{key}` on value `{receiver}`"
  )))
}

pub fn get(ctx: Context, receiver: &Value, key: Handle<Str>) -> Result<Value> {
  if let Some(o) = receiver.clone().to_object_raw() {
    if let Some(value) = o.field_get(key.as_str())? {
      if o.should_bind_methods() && is_fn_like(&value) {
        return Ok(Value::object(
          ctx.alloc(Method::new(receiver.clone(), value)),
        ));
      }
      return Ok(value);
    }
  }

  Err(Error::runtime(format!(
    "cannot get field `{key}` on value `{receiver}`"
  )))
}

pub fn get_opt(ctx: Context, receiver: &Value, key: Handle<Str>) -> Result<Value> {
  // early exit if on `none`
  if receiver.is_none() {
    return Ok(Value::none());
  }

  if let Some(o) = receiver.clone().to_object_raw() {
    if let Some(value) = o.field_get(key.as_str())? {
      if o.should_bind_methods() && is_fn_like(&value) {
        return Ok(Value::object(
          ctx.alloc(Method::new(receiver.clone(), value)),
        ));
      }
      return Ok(value);
    }
  }

  Ok(Value::none())
}

fn is_fn_like(v: &Value) -> bool {
  v.is_function() || v.is_method()
}
