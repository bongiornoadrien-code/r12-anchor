// Module de gestion CMS pour R12-Anchor
pub struct CmsData {
    pub version: u32,
}

impl CmsData {
    pub fn new(version: u32) -> Self {
        Self { version }
    }
}
