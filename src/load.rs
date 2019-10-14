use crate::data::*;

use std::path::{Path, PathBuf};
use std::{fs, io};

use comrak::{markdown_to_html, ComrakOptions};
use yaml_rust::yaml::Yaml;

pub fn load_data<P: AsRef<Path>>(base_dir: P) -> Data {
    let pages = load_pages(base_dir.as_ref().join("pages"));
    let categories = load_categories(base_dir.as_ref().join("categories"));

    Data {
        pages,
        categories,
        version: 0,
    }
}

pub fn load_pages<P: AsRef<Path>>(dir: P) -> Vec<Page> {
    fs::read_dir(dir)
        .map(|r| {
            r.flatten()
                .filter(|file| file.path().is_file())
                .flat_map(|file| load_page(&file.path()))
                .collect()
        })
        .unwrap_or_default()
}

pub fn load_page(path: &Path) -> Option<Page> {
    let contents = fs::read_to_string(path).ok()?;
    let (front, body) = frontmatter::parse_and_find_content(&contents).ok()?;

    let metadata = front.and_then(PageMetadata::from_yaml)?;

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
        html: markdown_to_html(body, &opts),
    };

    Some(page)
}

pub fn load_categories<P: AsRef<Path>>(dir: P) -> Vec<Category> {
    if let Ok(entries) = fs::read_dir(dir) {
        entries
            .flatten()
            .filter(|e| e.path().is_dir())
            .flat_map(|e| load_category(&e.path()))
            .collect()
    } else {
        vec![]
    }
}

pub fn is_image(p: &Path) -> bool {
    p.is_file()
        && match p.extension().map(|e| e.to_string_lossy().to_lowercase()) {
            Some(ext) => ext == "jpg" || ext == "png" || ext == "jpeg",
            _ => false,
        }
}

pub fn load_category<P: AsRef<Path>>(dir: P) -> Option<Category> {
    if let Ok(entries) = fs::read_dir(&dir) {
        let images = entries
            .flatten()
            .filter(|e| is_image(&e.path()))
            // .flat_map(|e| image::open(&e.path()))
            .map(|e| Image {
                src: e.path(),
                size: Size {
                    width: 0,
                    height: 0,
                },
                year: 2019,
                title: e.path().to_string_lossy().to_string(),
                technique: "mixte".to_string(),
            })
            .collect::<Vec<_>>();

        Some(Category {
            index: 0,
            name: dir.as_ref().to_string_lossy().to_string(),
            slug: Slug::new(&dir.as_ref().to_string_lossy()),
            thumbnail: dir.as_ref().to_path_buf(),
            images,
        })
    } else {
        None
    }
}
