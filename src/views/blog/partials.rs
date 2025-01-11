use super::*;
use crate::markdown;

pub fn tags(document: &Document) -> paxhtml::Element {
    document
        .metadata
        .taxonomies
        .as_ref()
        .map(|t| {
            let tags = t.tags.iter().map(|tag| {
                html! {
                    <li>
                        <a href={Route::BlogTag { tag_id: tag }.url_path()}>{format!("#{tag}")}</a>
                    </li>
                }
            });
            html! { <ul class="tags">#{tags}</ul> }
        })
        .unwrap_or_default()
}

pub fn date(document: &Document) -> paxhtml::Element {
    document
        .metadata
        .datetime()
        .map(|dt| dt.date_naive())
        .map(crate::elements::date_with_chrono)
        .unwrap_or_default()
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PostBody {
    Full,
    Description,
    Short,
}
pub fn post(context: ViewContext, document: &Document, post_body: PostBody) -> paxhtml::Element {
    let url = document.route_path().url_path();
    let short = document.metadata.short();
    let body = match post_body {
        PostBody::Full => &document.content,
        PostBody::Description => document.description.as_ref().unwrap_or(&document.content),
        PostBody::Short => short.as_ref().unwrap_or(&document.content),
    };

    html! {
        <article class="post">
            <header>
                <a href={url} class="post-title">
                    <h2>{break_on_colon(&document.metadata.title)}</h2>
                </a>
                <div class="post-meta">
                    {date(document)}
                    " · "
                    {tags(document)}
                </div>
            </header>
            <div class="post-body">
                {markdown::convert_to_html(context.syntax, body)}
                {if post_body == PostBody::Description {
                    html! {
                        <p>
                            <a href={url}>"Read more"</a>
                        </p>
                    }
                } else {
                    paxhtml::Element::Empty
                }}
            </div>
        </article>
    }
}
