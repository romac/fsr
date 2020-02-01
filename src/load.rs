use crate::data::*;

use std::path::{Path, PathBuf};
use std::{fs, io};

use comrak::{markdown_to_html, ComrakOptions};
use itertools::Itertools;
use yaml_rust::yaml::Yaml;

pub fn load_data<P: AsRef<Path>>(base_dir: P) -> Data {
    let pages = load_pages(base_dir.as_ref().join("pages"));
    let categories = load_gallery(base_dir.as_ref().join("gallery.csv"));

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
        hidden: metadata.hidden,
        path: path.to_owned(),
        content: body.to_string(),
        html: markdown_to_html(body, &opts),
    };

    Some(page)
}

#[derive(Clone, Debug)]
struct Record {
    id: String,
    title: String,
    year: String,
    theme: String,
}

fn parse_row(mut record: csv::StringRecord) -> Option<Record> {
    if record.len() < 3 {
        return None;
    }

    record.trim();

    let theme = record.get(0)?;
    let id = record.get(1)?;
    let year = record.get(2)?;
    let title = record.get(3)?;

    if id.is_empty() {
        return None;
    }

    Some(Record {
        id: id.to_string(),
        title: title.to_string(),
        year: year.to_string(),
        theme: theme.to_string(),
    })
}

pub fn load_gallery<P: AsRef<Path>>(csv_file: P) -> Vec<Category> {
    let mut rdr = csv::Reader::from_path(csv_file).unwrap();
    let records = rdr
        .records()
        .flat_map(|rc| rc.ok().and_then(parse_row))
        .collect::<Vec<_>>();

    let mut prev = Image {
        id: "".to_string(),
        title: "".to_string(),
        year: "".to_string(),
        theme: "".to_string(),
        ext: "".to_string(),
        src: PathBuf::from(""),
    };

    let images = records.into_iter().flat_map(|rec| {
        let (src, ext) = get_src(&rec.id)?;

        let image = Image {
            id: rec.id,
            title: rec.title,
            year: rec.year,
            theme: if rec.theme.is_empty() {
                prev.theme.clone()
            } else {
                rec.theme
            },
            src,
            ext,
        };

        prev = image.clone();

        Some(image)
    });

    images
        .group_by(|i| i.theme.clone())
        .into_iter()
        .map(|(theme, imgs)| {
            let images = imgs.collect::<Vec<_>>();
            Category {
                slug: Slug::new(&theme),
                name: theme,
                thumbnail: images.first().unwrap().clone(),
                images,
            }
        })
        .collect()
}

fn get_src(id: &str) -> Option<(PathBuf, String)> {
    let exts = ["jpg", "tif"];

    for ext in &exts {
        let path = PathBuf::from(format!("content/images/{}.{}", id, ext));

        if path.exists() {
            return Some((path, ext.to_string()));
        }
    }

    None
}
