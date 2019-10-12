use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Slug(String);

impl Slug {
    pub fn new(base: impl AsRef<str>) -> Self {
        Self(slug::slugify(base))
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
        self.pages.iter().find(|p| {
            dbg!((&p.slug.0, slug));
            p.slug.0 == slug
        })
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
    pub index: usize,
    pub name: String,
    pub slug: Slug,
    pub thumbnail: PathBuf,
    pub images: Vec<Image>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Image {
    pub src: PathBuf,
    pub size: Size,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
