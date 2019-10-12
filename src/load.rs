use crate::data::*;

use std::path::{Path, PathBuf};
use std::{fs, io};

use comrak::{markdown_to_html, ComrakOptions};
use yaml_rust::yaml::Yaml;

pub fn load_data<P: AsRef<Path>>(base_dir: P) -> io::Result<Data> {
    let pages = load_pages(base_dir.as_ref().join("pages"))?;
    let categories = load_categories(base_dir.as_ref().join("categories"))?;

    Ok(Data {
        pages,
        categories,
        version: 0,
    })
}

pub fn load_pages<P: AsRef<Path>>(dir: P) -> io::Result<Vec<Page>> {
    fs::read_dir(dir)?
        .map(|file| load_page(&file?.path()))
        .collect()
}

pub fn load_page(path: &Path) -> io::Result<Page> {
    let contents = fs::read_to_string(path)?;
    let (front, body) = frontmatter::parse_and_find_content(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let metadata = front.and_then(PageMetadata::from_yaml).unwrap(); // FIXME

    let opts = ComrakOptions {
        hardbreaks: true,
        smart: true,
        ext_autolink: true,
        ..ComrakOptions::default()
    };

    let slug = Slug::new(&metadata.title);

    let page = Page {
        slug: Slug::new(&metadata.title),
        index: metadata.index,
        title: metadata.title,
        path: path.to_owned(),
        content: body.to_string(),
        html: markdown_to_html(body, &opts).to_string(),
    };

    Ok(page)
}

pub fn load_categories<P: AsRef<Path>>(dir: P) -> io::Result<Vec<Category>> {
    Ok(Vec::new())
}
