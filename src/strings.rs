use std::str;

use data_encoding::{BASE32, BASE64};

gtmpl_fn!(
    fn base64encode(s: String) -> Result<String, String> {
        Ok(BASE64.encode(&s.into_bytes()))
    }
);

gtmpl_fn!(
    fn base64decode(s: String) -> Result<String, String> {
        BASE64.decode(&s.into_bytes())
            .map_err(|e| format!("unable to decode {}", e))
            .and_then(|v| str::from_utf8(&v)
                      .map_err(|e| format!("unable to decode: {}", e))
                      .map(|s| s.to_owned()))
    }
);

gtmpl_fn!(
    fn base32encode(s: String) -> Result<String, String> {
        Ok(BASE32.encode(&s.into_bytes()))
    }
);

gtmpl_fn!(
    fn base32decode(s: String) -> Result<String, String> {
        BASE32.decode(&s.into_bytes())
            .map_err(|e| format!("unable to decode {}", e))
            .and_then(|v| str::from_utf8(&v)
                      .map_err(|e| format!("unable to decode: {}", e))
                      .map(|s| s.to_owned()))
    }
);

gtmpl_fn!(
    fn abbrev(width: i64, s: String) -> Result<String, String> {
        if width < 4 || s.len() < width as usize {
            Ok(s)
        } else {
                Ok(format!("{}...", &s[..(width as usize - 3)]))
        }
    }
);

gtmpl_fn!(
    fn split(sep: String, orig: String) -> Result<Vec<String>, String> {
        Ok(orig.split(&sep).map(|s| s.to_owned()).collect())
    }
);

#[cfg(test)]
mod tests_mocked {
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
    fn test_base64encode() {
        test_fn!(base64encode, vvarc!("Hello World!"), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_base64decode() {
        test_fn!(base64decode, vvarc!("SGVsbG8gV29ybGQh"), "Hello World!");
    }

    #[test]
    fn test_base32encode() {
        test_fn!(base32encode, vvarc!("Hello World!"), "JBSWY3DPEBLW64TMMQQQ====");
    }

    #[test]
    fn test_base32decode() {
        test_fn!(base32decode, vvarc!("JBSWY3DPEBLW64TMMQQQ===="), "Hello World!");
    }

    #[test]
    fn test_abbrv() {
        test_fn!(abbrev, vvarc!(4, "foobar"), "f...");
    }

    #[test]
    fn test_split() {
        test_fn!(split, vvarc!(" ", "foo bar"), vec!["foo".to_owned(), "bar".to_owned()]);
    }
}
