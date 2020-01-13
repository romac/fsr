use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(base: impl AsRef<str>) -> Self {
        Self(slug::slugify(base))
    }
}

impl fmt::Debug for Slug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Data {
    pub pages: Vec<Page>,
    pub categories: Vec<Category>,
    pub version: usize,
}

impl Data {
    pub fn empty() -> Self {
        Self {
            pages: Vec::new(),
            categories: Vec::new(),
            version: 0,
        }
    }

    pub fn find_page(&self, slug: &str) -> Option<&Page> {
        self.pages.iter().find(|p| p.slug == Slug::new(slug))
    }

    pub fn find_category(&self, slug: &str) -> Option<&Category> {
        self.categories.iter().find(|c| c.slug == Slug::new(slug))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct PageMetadata {
    pub index: usize,
    pub title: String,
}

impl PageMetadata {
    pub fn from_yaml(yaml: Yaml) -> Option<Self> {
        let hash = yaml.into_hash()?;
        let index = hash.get(&Yaml::from_str("index"))?.as_i64()? as usize;
        let title = hash.get(&Yaml::from_str("title"))?.as_str()?.to_string();
        Some(Self { index, title })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Page {
    pub index: usize,
    pub title: String,
    pub slug: Slug,
    pub path: PathBuf,
    pub content: String,
    pub html: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Category {
    pub name: String,
    pub slug: Slug,
    pub images: Vec<Image>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Image {
    pub id: String,
    pub title: String,
    pub theme: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
