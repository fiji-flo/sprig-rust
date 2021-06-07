use std::collections::HashMap;
use std::time::SystemTime;

use gtmpl::gtmpl_fn;
use gtmpl_value::{FuncError, Value};

gtmpl_fn!(
    #[doc = r#"Similar to golangs current time.Time. Not fully supported."#]
    fn now() -> Result<Value, FuncError> {
        let mut map = HashMap::new();
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| FuncError::Generic(e.to_string()))?
            .as_secs();
        map.insert(String::from("Unix"), Value::from(ts));
        Ok(map.into())
    }
);

#[cfg(test)]
mod test {
    use super::*;
    use gtmpl_value::FromValue;

    #[test]
    fn test_now_unix() {
        let ts1 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| format!("{}", e))
            .unwrap()
            .as_secs();
        let no_arg: Vec<Value> = vec![];
        let n = now(&no_arg).unwrap();
        let h: HashMap<String, u64> = HashMap::from_value(&n).unwrap();
        let ts2 = *h.get("Unix").unwrap();
        let ts3 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| format!("{}", e))
            .unwrap()
            .as_secs();
        assert!(ts2 >= ts1);
        assert!(ts3 >= ts2);
    }
}
