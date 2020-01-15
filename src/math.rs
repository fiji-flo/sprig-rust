gtmpl_fn!(
    #[doc = r#"Add 1 to a value."#]
    fn add1(n: u64) -> Result<String, String> {
        Ok((n + 1).to_string())
    }
);

#[cfg(test)]
mod test {
    use super::*;
    use gtmpl_value::Value;

    #[test]
    fn test_add1() {
        test_fn!(add1, vval!(1), "2");
    }

}