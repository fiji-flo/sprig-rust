use gtmpl_value::{FuncError, Value};

/// Give a default value. Used like this: trim "   "| default "empty".
/// Since trim produces an empty string, the default value is returned. For
/// things with a length (strings, slices, maps), len(0) will trigger the default.
/// For numbers, the value 0 will trigger the default. For booleans, false will
/// trigger the default. For structs, the default is never returned (there is
/// no clear empty condition). For everything else, nil value triggers a default.
pub fn default(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::ExactlyXArgs("default".into(), 2));
    }

    if is_empty(&args[1]) {
        Ok(args[0].clone())
    } else {
        Ok(args[1].clone())
    }
}

fn is_empty(val: &Value) -> bool {
    match *val {
        Value::String(ref s) => s.is_empty(),
        Value::Array(ref a) => a.is_empty(),
        Value::Bool(ref b) => !b,
        Value::Function(_) => false,
        Value::Nil | Value::NoValue => true,
        Value::Object(ref o) | Value::Map(ref o) => o.is_empty(),
        Value::Number(ref n) => {
            if let Some(u) = n.as_u64() {
                u == 0
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gtmpl_value::Value;

    #[test]
    fn test_default() {
        test_fn!(default, vval!("foo", ""), "foo");
    }
}
