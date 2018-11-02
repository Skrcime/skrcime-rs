use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::io::Cursor;

#[derive(Debug)]
pub struct Metadata {
    pub title: String,
    pub description: String,
}

pub fn from_url(url: &str) -> Option<Metadata> {
    let html = reqwest::get(url).and_then(|mut res| res.text());

    match html {
        Ok(html) => get_metadata(html),
        Err(_) => None,
    }
}

fn get_metadata(html: String) -> Option<Metadata> {
    match Document::from_read(Cursor::new(html)) {
        Ok(document) => {
            let title: String = document
                .find(Name("title"))
                .next()
                .map(|node| node.text())
                .unwrap_or_else(|| "No title".to_string());

            let description: String = document
                .find(Attr("name", "description"))
                .next()
                .and_then(|node| node.attr("content"))
                .map(|content| content.to_string())
                .unwrap_or_else(|| "No description".to_string());

            Some(Metadata {
                title: title.to_string(),
                description: description.to_string(),
            })
        }
        Err(_) => None,
    }
}
