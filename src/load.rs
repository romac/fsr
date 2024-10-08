use std::path::{Path, PathBuf};

use futures::StreamExt;

use comrak::{markdown_to_html, ComrakOptions};
use itertools::Itertools;
use tokio::fs::{self, File};
use tracing::info;

use crate::data::*;

pub async fn load_data<P: AsRef<Path>>(base_dir: P) -> Data {
    let base_dir = base_dir.as_ref();

    let pages = load_pages(base_dir.join("pages")).await;
    let categories = load_gallery(base_dir, &base_dir.join("gallery.csv")).await;
    let virtual_expo = load_virtual_expo(base_dir, &base_dir.join("virtual.csv")).await;

    info!("Loaded {} pages", pages.len());
    info!("Loaded {} categories", categories.len());
    info!("Loaded {} virtual images", virtual_expo.len());

    Data {
        pages,
        categories,
        virtual_expo,
        version: 0,
    }
}

pub async fn load_pages(dir: impl AsRef<Path>) -> Vec<Page> {
    let mut read_dir = fs::read_dir(dir).await.unwrap();

    let mut pages = vec![];
    while let Ok(Some(entry)) = read_dir.next_entry().await {
        if !entry.path().is_file() {
            continue;
        }

        let page = load_page(&entry.path()).await;
        if let Some(page) = page {
            pages.push(page);
        }
    }

    pages
}

pub async fn load_page(path: &Path) -> Option<Page> {
    let contents = fs::read_to_string(path).await.ok()?;
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
pub async fn load_gallery(base_dir: &Path, csv_file: &Path) -> Vec<Category> {
    let file = File::open(csv_file).await.unwrap();

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
            let (src, ext) = get_src(base_dir, &rec.id).await?;

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
        .chunk_by(|i| i.theme.clone())
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

pub async fn load_virtual_expo(base_dir: &Path, csv_file: &Path) -> Vec<VirtualImage> {
    let file = File::open(csv_file).await.unwrap();
    let rdr = csv_async::AsyncReaderBuilder::new()
        .trim(csv_async::Trim::All)
        .quoting(false)
        .delimiter(b',')
        .has_headers(false)
        .create_reader(file);

    rdr.into_records()
        .filter_map(|rc| async { rc.ok().and_then(parse_virtual_row) })
        .filter_map(|rec| async {
            let (src, ext) = get_src(base_dir, &rec.id).await?;

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

async fn get_src(base_dir: &Path, id: &str) -> Option<(PathBuf, &'static str)> {
    let path = base_dir.join(format!("images/{}.jpg", id));
    path.exists().then_some((path, "jpg"))
}
