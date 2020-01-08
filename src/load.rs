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
        path: path.to_owned(),
        content: body.to_string(),
        html: markdown_to_html(body, &opts),
    };

    Some(page)
}

#[derive(Clone, Debug)]
struct Record {
    id: String,
    name: String,
    theme: String,
}

fn parse_row(mut record: csv::StringRecord) -> Option<Record> {
    if record.len() < 3 {
        return None;
    }

    record.trim();

    let theme = record.get(0)?;
    let name = record.get(1)?;
    let id = record.get(2)?;

    if theme.is_empty() || name.is_empty() || id.is_empty() {
        return None;
    }

    Some(Record {
        id: id.to_string(),
        name: name.to_string(),
        theme: theme.to_string(),
    })
}

pub fn load_gallery<P: AsRef<Path>>(csv_file: P) -> Vec<Category> {
    let mut rdr = csv::Reader::from_path(csv_file).unwrap();
    let records = rdr
        .records()
        .flat_map(|rc| rc.ok().and_then(parse_row))
        .collect::<Vec<_>>();

    let images = records.into_iter().map(|rec| {
        let src = format!("_content/images/{}.jpg", rec.id);
        Image {
            id: rec.id,
            src: PathBuf::from(src),
            size: Size::default(),
            year: 2020,
            title: rec.name,
            technique: rec.theme,
        }
    });

    let categories = images
        .group_by(|i| i.technique.clone())
        .into_iter()
        .map(|(theme, images)| Category {
            index: 0,
            slug: Slug::new(&theme),
            name: theme,
            thumbnail: PathBuf::new(),
            images: images.collect(),
        })
        .collect();

    dbg!(&categories);

    categories
}
