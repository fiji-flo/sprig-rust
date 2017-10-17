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
    use serde_json::Value;

    macro_rules! varc(
        ($x:expr) => { Arc::new(Value::from($x)) }
    );

    #[test]
    fn test_split() {
        let vals: Vec<Arc<Any>> = vec![varc!(" "), varc!("foo bar")];
        let ret = split(&vals).unwrap();
        let ret_ = ret.downcast_ref::<Value>();
        let expected = vec!["foo".to_owned(), "bar".to_owned()];
        assert_eq!(ret_, Some(&Value::from(expected)));
    }
}
