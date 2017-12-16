use rand::{self, Rng};

pub fn random_utf8(len: usize) -> String {
    rand::thread_rng().gen_iter::<char>().take(len).collect()
}

pub fn random_ascii(len: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(len).collect()
}

pub fn random_alpha(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .filter(|c| c.is_alphabetic())
        .take(len)
        .collect()
}

pub fn random_numeric(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .filter(|c| c.is_numeric())
        .take(len)
        .collect()
}

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