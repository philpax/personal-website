use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::Deserialize;

use crate::{util, Route, RoutePath};

pub type DocumentId = String;
pub type Tag = String;

#[derive(Debug)]
pub struct Content {
    pub path: PathBuf,
    pub blog: Blog,
    pub about: Document,
}
impl Content {
    pub fn read() -> anyhow::Result<Self> {
        let path = PathBuf::from("content");

        Ok(Content {
            path: path.clone(),
            blog: Blog::read(&path.join("blog"))?,
            about: Document::read(&path.join("about.md"), "about".to_string())?,
        })
    }
}

#[derive(Debug)]
pub struct Blog {
    pub tags: HashMap<Tag, Vec<DocumentId>>,
    pub documents: Vec<Document>,
    pub document_key_to_id: HashMap<DocumentId, usize>,
}
impl Blog {
    fn read(blog_path: &Path) -> anyhow::Result<Self> {
        let documents = {
            let mut documents = vec![];
            for entry in std::fs::read_dir(blog_path)? {
                let path = entry?.path();
                if !path.is_dir() {
                    continue;
                }

                let id = path
                    .file_name()
                    .context("no post id for directory")?
                    .to_string_lossy()
                    .to_string();

                let index = path.join("index.md");
                if !index.exists() {
                    anyhow::bail!("{index:?} does not exist");
                }

                documents.push(Document::read(&index, id)?);
            }
            documents.sort_by_key(|d| d.metadata.datetime());
            documents.reverse();
            documents
        };

        let document_key_to_id = {
            let mut document_key_to_id = HashMap::new();
            for (i, document) in documents.iter().enumerate() {
                document_key_to_id.insert(document.id.clone(), i);
                if let Some(alternate_id) = &document.alternate_id {
                    document_key_to_id.insert(alternate_id.clone(), i);
                }
            }
            document_key_to_id
        };

        let tags = {
            let mut tags = HashMap::new();
            for document in &documents {
                let Some(taxonomies) = &document.metadata.taxonomies else {
                    continue;
                };
                for tag in &taxonomies.tags {
                    tags.entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(document.id.clone());
                }
            }
            tags
        };

        Ok(Blog {
            tags,
            documents,
            document_key_to_id,
        })
    }

    pub fn document_by_id(&self, id: &str) -> Option<&Document> {
        self.document_key_to_id
            .get(id)
            .and_then(|&i| self.documents.get(i))
    }
}

#[derive(Debug)]
pub struct Document {
    pub id: DocumentId,
    pub alternate_id: Option<DocumentId>,
    pub metadata: DocumentMetadata,
    pub description: markdown::mdast::Node,
    pub rest_of_content: Option<markdown::mdast::Node>,
    pub files: Vec<PathBuf>,
}
impl Document {
    fn read(path: &Path, id: String) -> anyhow::Result<Self> {
        let file = std::fs::read_to_string(path)?;
        let parts: Vec<_> = file.splitn(3, "+++").collect();

        if parts.len() != 3 {
            return Err(anyhow::anyhow!("invalid markdown file"));
        }

        let metadata: DocumentMetadata = toml::from_str(parts[1])?;
        let content_raw = parts[2];

        let (description, rest_of_content) = match content_raw.split_once("<!-- more -->") {
            Some((description, rest_of_content)) => (
                parse_markdown(description.trim()),
                Some(parse_markdown(rest_of_content.trim())),
            ),
            None => (parse_markdown(content_raw), None),
        };

        let mut files = vec![];
        for entry in std::fs::read_dir(path.parent().unwrap())? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "md") {
                continue;
            }
            files.push(path);
        }

        let alternate_id = util::slugify(&metadata.title);
        let alternate_id = if alternate_id == id {
            None
        } else {
            Some(alternate_id)
        };

        Ok(Document {
            id,
            alternate_id,
            metadata,
            description,
            rest_of_content,
            files,
        })
    }

    pub fn route_path(&self) -> RoutePath {
        Route::BlogPost { post_id: &self.id }.route_path()
    }

    pub fn alternate_route_path(&self) -> Option<RoutePath> {
        self.alternate_id
            .as_ref()
            .map(|post_id| Route::BlogPost { post_id }.route_path())
    }
}

#[derive(Debug, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    short: Option<String>,
    #[serde(with = "toml_datetime_compat", default)]
    date: Option<chrono::NaiveDate>,
    pub taxonomies: Option<DocumentTaxonomies>,
}
impl DocumentMetadata {
    pub fn short(&self) -> Option<markdown::mdast::Node> {
        self.short.as_deref().map(parse_markdown)
    }

    pub fn datetime(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.date.map(|d| d.and_time(Default::default()).and_utc())
    }
}

#[derive(Debug, Deserialize)]
pub struct DocumentTaxonomies {
    pub tags: Vec<Tag>,
}

pub fn parse_markdown(md: &str) -> markdown::mdast::Node {
    markdown::to_mdast(md, &markdown::ParseOptions::gfm()).unwrap()
}
