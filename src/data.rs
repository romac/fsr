use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Slug(String);

impl Slug {
    pub fn new(base: impl AsRef<str>) -> Self {
        Self(slug::slugify(base))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Data {
    pub pages: Vector<Page>,
    pub categories: Vector<Category>,
    pub version: usize,
}

impl Data {
    pub fn empty() -> Self {
        Self {
            pages: Vector::new(),
            categories: Vector::new(),
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Page {
    pub index: usize,
    pub title: String,
    pub slug: Slug,
    pub path: PathBuf,
    pub content: String,
    pub html: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Category {
    pub index: usize,
    pub name: String,
    pub slug: Slug,
    pub thumbnail: PathBuf,
    pub sub_categories: Vector<SubCategory>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SubCategory {
    pub index: usize,
    pub name: String,
    pub slug: Slug,
    pub images: Vector<Image>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Image {
    pub src: PathBuf,
    pub size: Size,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
