// Data structures for requirements

use serde::Serialize;

/// A requirement with index and title (for listing)
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[cfg_attr(test, allow(dead_code))]
pub struct RequirementSummary {
    pub index: String,
    pub title: String,
}

/// Deleted requirement info (for delete response) (G.TOOLREQLIXD.4)
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DeletedRequirement {
    pub index: String,
    pub title: String,
    pub category: String,
    pub chapter: String,
}

/// A full requirement with all data
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[cfg_attr(test, allow(dead_code))]
pub struct RequirementFull {
    pub index: String,
    pub title: String,
    pub text: String,
    pub category: String,
    pub chapter: String,
}
