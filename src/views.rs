use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvolvementFilter {
    Contributor,
    Reviewer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewConfig {
    pub name: CompactString,
    pub key: CompactString,
    #[serde(default)]
    pub search_filter: Option<CompactString>,
    #[serde(default)]
    pub recent_days: Option<u32>,
    #[serde(default)]
    pub involvement: Option<InvolvementFilter>,
}

/// Top-level structure for views.toml — sections as [views.<slug>]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewsFile {
    pub views: BTreeMap<String, ViewConfig>,
}

impl Default for ViewsFile {
    fn default() -> Self {
        let mut views = BTreeMap::new();
        views.insert(
            "mywork".into(),
            ViewConfig {
                name: "My Work".into(),
                key: "1".into(),
                search_filter: None,
                recent_days: Some(14),
                involvement: Some(InvolvementFilter::Contributor),
            },
        );
        views.insert(
            "reviewing".into(),
            ViewConfig {
                name: "Reviewing".into(),
                key: "2".into(),
                search_filter: None,
                recent_days: None,
                involvement: Some(InvolvementFilter::Reviewer),
            },
        );
        views.insert(
            "all".into(),
            ViewConfig {
                name: "All".into(),
                key: "3".into(),
                search_filter: None,
                recent_days: None,
                involvement: None,
            },
        );
        Self { views }
    }
}

impl ViewsFile {
    /// Returns views sorted by their key field (determines tab order and keyboard shortcut)
    pub fn sorted_views(&self) -> Vec<ViewConfig> {
        let mut views: Vec<ViewConfig> = self.views.values().cloned().collect();
        views.sort_by(|a, b| a.key.cmp(&b.key));
        views
    }
}
