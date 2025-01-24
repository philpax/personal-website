use crate::{
    content::{Blog, Document},
    markdown::MarkdownConverter,
    syntax::SyntaxHighlighter,
};

pub struct RssConfig<'a> {
    pub base_url: &'a str,
    pub rss_title: &'a str,
    pub rss_author: &'a str,
    pub rss_description: &'a str,
    pub syntax: &'a SyntaxHighlighter,
}

pub fn generate(config: RssConfig, blog: &Blog, relative_path: &str) -> anyhow::Result<String> {
    let RssConfig {
        base_url,
        rss_title,
        rss_author,
        rss_description,
        syntax,
    } = config;

    let items = blog
        .documents
        .iter()
        .map(|doc| build_item(base_url, rss_author, doc, syntax))
        .collect::<Vec<_>>();

    let atom_ext = rss::extension::atom::AtomExtensionBuilder::default()
        .link(rss::extension::atom::Link {
            rel: "self".into(),
            href: format!("{base_url}/{relative_path}"),
            mime_type: Some("application/rss+xml".to_string()),
            ..Default::default()
        })
        .build();

    let rss_channel = rss::ChannelBuilder::default()
        .title(rss_title)
        .link(base_url)
        .atom_ext(atom_ext)
        .description(rss_description)
        .language("en-AU".to_string())
        .last_build_date(chrono::Utc::now().to_rfc2822())
        .generator("paxgen".to_string())
        .items(items)
        .build();

    Ok(String::from_utf8(rss_channel.pretty_write_to(
        Vec::new(),
        b' ',
        2,
    )?)?)
}

fn build_item(
    base_url: &str,
    author: &str,
    doc: &Document,
    syntax: &SyntaxHighlighter,
) -> rss::Item {
    let url = format!("{base_url}{}", doc.route_path().url_path());

    let guid = rss::GuidBuilder::default()
        .value(url.clone())
        .permalink(false)
        .build();

    let description =
        paxhtml::Document::new([MarkdownConverter::new(syntax).convert(&doc.description)])
            .write_to_string()
            .unwrap();

    rss::ItemBuilder::default()
        .title(doc.metadata.title.clone())
        .link(url)
        .guid(guid)
        .description(description)
        .author(author.to_string())
        .pub_date(doc.metadata.datetime().map(|d| d.to_rfc2822()))
        .build()
}
