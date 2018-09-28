use rand::distributions::{Alphanumeric, Standard};
use rand::{self, Rng};

pub fn random_utf8(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(&Standard)
        .take(len)
        .collect()
}

pub fn random_ascii(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect()
}

pub fn random_alpha(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_alphabetic())
        .take(len)
        .collect()
}

pub fn random_numeric(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_numeric())
        .take(len)
        .collect()
}

#[cfg(test)]
#[macro_export]
macro_rules! val(
        ($x:expr) => { { Value::from($x) } }
    );

#[cfg(test)]
#[macro_export]
macro_rules! vval(
        ($($x:expr),*) => { { vec![$(val!($x)),*] } }
    );

#[cfg(test)]
#[macro_export]
macro_rules! test_fn_assert(
        ($func:ident, $args:expr, $typ:ident, $ass:ident) => {
            let v = $args;
            let ret = $func(&v);
            if let Ok(Value::$typ(ref x)) = ret {
                return assert!($ass(x));
            }
            assert!(false);
        }
    );

#[cfg(test)]
#[macro_export]
macro_rules! test_fn(
        ($func:ident, $args:expr, $exp:expr) => {
            let v = $args;
            let ret = $func(&v);
            let expected = $exp;
            assert_eq!(ret, Ok(Value::from(expected)));
        }
    );

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random_utf8() {
        let s = random_utf8(20);
        assert_eq!(s.chars().count(), 20);
    }

    #[test]
    fn test_random_alpha() {
        let s = random_alpha(20);
        assert_eq!(s.len(), 20);
    }

    #[test]
    fn test_random_acii() {
        let s = random_ascii(20);
        assert_eq!(s.len(), 20);
    }

    #[test]
    fn test_random_numeric() {
        let s = random_numeric(10);
        assert_eq!(s.len(), 10);
        let i = s.parse::<i64>();
        assert!(i.is_ok());
    }
}
