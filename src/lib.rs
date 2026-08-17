pub mod anchor {
    pub mod tst_info;
    pub mod cms;
}

pub fn anchor() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor() {
        assert!(anchor());
    }
}
