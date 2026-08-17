// Module de gestion CMS pour R12-Anchor
pub struct CmsData {
    pub version: u32,
}

impl CmsData {
    pub fn new(version: u32) -> Self {
        Self { version }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cms_new() {
        let cms = CmsData::new(1);
        assert_eq!(cms.version, 1);
    }
}
