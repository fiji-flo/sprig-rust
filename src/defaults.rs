use std::any::Any;
use std::sync::Arc;

use gtmpl_value::Value;

/// Give a default value. Used like this: trim "   "| default "empty".
/// Since trim produces an empty string, the default value is returned. For
/// things with a length (strings, slices, maps), len(0) will trigger the default.
/// For numbers, the value 0 will trigger the default. For booleans, false will
/// trigger the default. For structs, the default is never returned (there is
/// no clear empty condition). For everything else, nil value triggers a default.
pub fn default(args: &[Arc<Any>]) -> Result<Arc<Any>, String> {
    if args.len() != 2 {
        return Err(String::from("two arguments required"));
    }

    let arg1 = args[1]
        .downcast_ref::<Value>()
        .ok_or_else(|| "unable to downcast FOO".to_owned())?;

    if is_empty(arg1) {
        Ok(Arc::clone(&args[0]))
    } else {
        Ok(Arc::clone(&args[1]))
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
    use std::any::Any;
    use std::sync::Arc;
    use gtmpl_value::Value;

    macro_rules! varc(
        ($x:expr) => { { let v: Arc<Any> = Arc::new(Value::from($x)); v } }
    );

    macro_rules! vvarc(
        ($($x:expr),*) => { { let v: Vec<Arc<Any>> = vec![$(varc!($x)),*]; v } }
    );

    macro_rules! test_fn(
        ($func:ident, $args:expr, $exp:expr) => {
            let v = $args;
            let ret = $func(&v).unwrap();
            let ret_ = ret.downcast_ref::<Value>();
            let expected = $exp;
            assert_eq!(ret_, Some(&Value::from(expected)));
        }
    );

    #[test]
    fn test_default() {
        test_fn!(default, vvarc!("foo", ""), "foo");
    }
}
