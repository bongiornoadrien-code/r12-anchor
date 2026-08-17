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
