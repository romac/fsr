use async_std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use futures::StreamExt;

use comrak::{markdown_to_html, ComrakOptions};
use itertools::Itertools;

use crate::data::*;

pub async fn load_data<P: AsRef<Path>>(base_dir: P) -> Data {
    let pages = load_pages(base_dir.as_ref().join("pages")).await;
    let categories = load_gallery(base_dir.as_ref().join("gallery.csv")).await;
    let virtual_expo = load_virtual_expo(base_dir.as_ref().join("virtual.csv")).await;

    Data {
        pages,
        categories,
        virtual_expo,
        version: 0,
    }
}

pub async fn load_pages<P: AsRef<Path>>(dir: P) -> Vec<Page> {
    let mut read_dir = fs::read_dir(dir).await.unwrap();

    let mut pages = vec![];
    while let Some(entry) = read_dir.next().await {
        if let Ok(entry) = entry {
            if !entry.path().is_file().await {
                continue;
            }

            let page = load_page(&entry.path()).await;
            if let Some(page) = page {
                pages.push(page);
            }
        }
    }

    pages
}

pub async fn load_page(path: impl AsRef<Path>) -> Option<Page> {
    let contents = fs::read_to_string(path.as_ref()).await.ok()?;
    let (front, body) = frontmatter::parse_and_find_content(&contents).ok()?;
    let metadata = PageMetadata::from_yaml(front?)?;

    let mut opts = ComrakOptions::default();
    opts.parse.smart = true;
    opts.render.hardbreaks = true;
    opts.extension.autolink = true;
    opts.extension.header_ids = Some("header-".to_string());

    let page = Page {
        slug: Slug::new(&metadata.title),
        index: metadata.index,
        title: metadata.title,
        hidden: metadata.hidden,
        path: path.as_ref().to_owned(),
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

fn parse_row(record: csv_async::StringRecord) -> Option<Record> {
    if record.len() < 3 {
        return None;
    }

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

// Warning: this is an abomination
pub async fn load_gallery<P: AsRef<Path>>(csv_file: P) -> Vec<Category> {
    let file = File::open(csv_file.as_ref()).await.unwrap();
    let rdr = csv_async::AsyncReaderBuilder::new()
        .trim(csv_async::Trim::All)
        .quoting(false)
        .delimiter(b';')
        .has_headers(true)
        .create_reader(file);

    let records = rdr
        .into_records()
        .filter_map(|rc| async { rc.ok().and_then(parse_row) });

    let (images, _) = records
        .filter_map(|rec| async {
            let (src, ext) = get_src(&rec.id).await?;

            Some(Image {
                id: rec.id,
                title: rec.title,
                year: rec.year,
                theme: rec.theme,
                ext,
                src,
            })
        })
        .fold(
            (Vec::new(), "".to_string()),
            |(mut images, prev_theme), mut image| async move {
                if image.theme.is_empty() {
                    image.theme = prev_theme.clone();
                };

                let theme = image.theme.clone();
                images.push(image);
                (images, theme)
            },
        )
        .await;

    images
        .into_iter()
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

#[derive(Clone, Debug)]
struct VirtualRecord {
    id: String,
    title: String,
    technique: String,
    dimensions: String,
    price: String,
}

fn parse_virtual_row(record: csv_async::StringRecord) -> Option<VirtualRecord> {
    if record.len() < 5 {
        return None;
    }

    let id = record.get(0)?;
    let title = record.get(1)?;
    let technique = record.get(2)?;
    let dimensions = record.get(3)?;
    let price = record.get(4)?;

    if id.is_empty() {
        return None;
    }

    Some(VirtualRecord {
        id: id.to_string(),
        title: title.to_string(),
        technique: technique.to_string(),
        dimensions: dimensions.to_string(),
        price: price.to_string(),
    })
}

pub async fn load_virtual_expo<P: AsRef<Path>>(csv_file: P) -> Vec<VirtualImage> {
    let file = File::open(csv_file.as_ref()).await.unwrap();
    let rdr = csv_async::AsyncReaderBuilder::new()
        .trim(csv_async::Trim::All)
        .quoting(false)
        .delimiter(b',')
        .has_headers(false)
        .create_reader(file);

    rdr.into_records()
        .filter_map(|rc| async { rc.ok().and_then(parse_virtual_row) })
        .filter_map(|rec| async {
            let (src, ext) = get_src(&rec.id).await?;

            Some(VirtualImage {
                id: rec.id,
                title: rec.title,
                technique: rec.technique,
                dimensions: rec.dimensions,
                price: rec.price,
                ext,
                src,
            })
        })
        .collect()
        .await
}

async fn get_src(id: &str) -> Option<(PathBuf, &'static str)> {
    let path = PathBuf::from(format!("content/images/{}.jpg", id));

    if path.exists().await {
        Some((path, "jpg"))
    } else {
        None
    }
}
