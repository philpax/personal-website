use crate::{
    content::{Collection, Document},
    elements::*,
    markdown,
    syntax::SyntaxHighlighter,
    util,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PostBody {
    Full,
    Description,
    Short,
}

pub fn post(
    collection: &Collection,
    document: &Document,
    post_body: PostBody,
    syntax_highlighter: &SyntaxHighlighter,
) -> paxhtml::Element {
    let mut post_body_html = markdown::convert_to_html(
        syntax_highlighter,
        &match post_body {
            PostBody::Full => None,
            PostBody::Description => document.description.clone(),
            PostBody::Short => document.metadata.short(),
        }
        .unwrap_or(document.content.clone()),
    );

    if post_body == PostBody::Description {
        post_body_html.push(html! {
            <p>
                <a href={document.route_path(collection).url_path()}>"Read more"</a>
            </p>
        });
    }

    let tag_list = document
        .metadata
        .taxonomies
        .as_ref()
        .map(|t| {
            html! {
                <ul class="tags">
                    {
                        t.tags
                            .iter()
                            .map(|tag| html! {
                                <li>
                                    <a href={format!("/tags/{tag}")}>{format!("#{tag}")}</a>
                                </li>
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
            }
        })
        .unwrap_or_default();

    let article = html! {
        <article class="post">
            <header>
                {h2_with_id(html! {
                    <a href={document.route_path(collection).url_path()}>
                        {document.metadata.title.clone()}
                    </a>
                })}
                {tag_list}
                {document
                    .metadata
                    .datetime()
                    .map(|dt| dt.date_naive())
                    .map(crate::elements::date_with_chrono)
                    .unwrap_or_default()}
            </header>
            <div class="post-body">
                {post_body_html}
            </div>
        </article>
    };

    if post_body != PostBody::Full {
        return vec![article].into();
    }

    let heading_hierarchy = markdown::heading_hierarchy(&document.content);
    if heading_hierarchy.is_empty() {
        return vec![article].into();
    }

    fn build_list_recursively(hierarchy: &markdown::HeadingHierarchy) -> paxhtml::Element {
        let markdown::HeadingHierarchy(text, children) = hierarchy;
        let link = html! {
            <a href={format!("#{}", util::slugify(text))}>{text}</a>
        };

        let mut body = vec![link];
        if !children.is_empty() {
            body.push(html! {
                <ul>
                    {children.iter().map(build_list_recursively).collect::<Vec<_>>()}
                </ul>
            });
        }
        html! {
            <li>
                {body}
            </li>
        }
    }

    let aside = html! {
        <aside>
            {h2_with_id("Contents")}
            <ul>
                {
                    heading_hierarchy
                    .iter()
                    .map(build_list_recursively)
                    .collect::<Vec<_>>()
                }
            </ul>
        </aside>
    };

    vec![article, aside].into()
}
