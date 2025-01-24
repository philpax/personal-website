use super::*;
use crate::{
    markdown::{HeadingHierarchy, MarkdownConverter},
    util,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HeaderFocus<'a> {
    AllPosts,
    Tags,
    Tag(&'a str),
}
pub fn header(focus: HeaderFocus) -> paxhtml::Element {
    fn class(is_active: bool) -> &'static str {
        if is_active {
            "no-underline active"
        } else {
            "no-underline"
        }
    }

    html! {
        <header id="posts-header">
            <a href={Route::Blog.url_path()} class={class(focus == HeaderFocus::AllPosts)}>"All posts"</a>
            " · "
            <a href={Route::BlogTags.url_path()} class={class(focus == HeaderFocus::Tags)}>"Tags"</a>
            {match focus {
                HeaderFocus::AllPosts => html! {
                    <>
                        " · "
                        <a href={Route::BlogRss.url_path()} class={class(false)}>
                            "RSS"
                        </a>
                    </>
                },
                HeaderFocus::Tag(tag) => html! {
                    <>
                        " · "
                        <a href={Route::BlogTag { tag_id: tag }.url_path()} class={class(true)}>
                            "#"{tag}
                        </a>
                    </>
                },
                _ => paxhtml::Element::Empty,
            }}
        </header>
    }
}

pub fn tags(document: &Document) -> paxhtml::Element {
    document
        .metadata
        .taxonomies
        .as_ref()
        .map(|t| {
            let tags = t.tags.iter().map(|tag| {
                html! {
                    <li>
                        <a class="no-underline" href={Route::BlogTag { tag_id: tag }.url_path()}>{format!("#{tag}")}</a>
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

#[derive(Clone, PartialEq, Eq)]
pub enum PostBody {
    Full,
    Description,
    Short,
}
pub fn post(context: ViewContext, document: &Document, post_body: PostBody) -> paxhtml::Element {
    let url = document.route_path().url_path();
    let post_body = match post_body {
        PostBody::Full => {
            let mut elements = vec![];

            let toc = document_to_html_list(document);
            if let Some(hierarchy_list) = toc.clone() {
                elements.push(html! {
                    <aside id="toc-sticky" hidden>
                        <h3>"Table of Contents"</h3>
                        {hierarchy_list}
                    </aside>
                });
            }

            elements.push(MarkdownConverter::new(context.syntax).convert(&document.description));

            if let Some(hierarchy_list) = toc {
                elements.push(html! {
                    <aside id="toc-inline">
                        <h3>"Table of Contents"</h3>
                        {hierarchy_list}
                    </aside>
                });
            }

            if let Some(content) = document.rest_of_content.as_ref() {
                elements.push(MarkdownConverter::new(context.syntax).convert(content));
            }

            paxhtml::Element::from(elements)
        }
        PostBody::Description => html! {
            <>
                {MarkdownConverter::new(context.syntax).convert(&document.description)}
                <p>
                    <a href={url}>"Read more"</a>
                </p>
            </>
        },
        PostBody::Short => MarkdownConverter::new(context.syntax).convert(
            document
                .metadata
                .short()
                .as_ref()
                .unwrap_or(&document.description),
        ),
    };

    html! {
        <article class="post">
            <header>
                <div class="post-meta">
                    {date(document)}
                    " · "
                    {tags(document)}
                </div>
                <a href={url} class="post-title no-underline">
                    <h2>{break_on_colon(&document.metadata.title)}</h2>
                </a>
            </header>
            <div class="post-body">
                {post_body}
            </div>
        </article>
    }
}

fn document_to_html_list(document: &Document) -> Option<paxhtml::Element> {
    let heading_hierarchy = HeadingHierarchy::from_node(document.rest_of_content.as_ref()?);

    fn build_list_recursively(children: &[HeadingHierarchy], toplevel: bool) -> paxhtml::Element {
        if children.is_empty() {
            return paxhtml::Element::Empty;
        }

        html! {
            <ul>
                {if toplevel {
                    html! {
                        <li><a href="#">"Introduction"</a></li>
                    }
                } else {
                    paxhtml::Element::Empty
                }}
                #{children.iter().map(build_list_item_recursively)}
            </ul>
        }
    }

    fn build_list_item_recursively(
        HeadingHierarchy { heading, children }: &HeadingHierarchy,
    ) -> paxhtml::Element {
        html! {
            <li>
                <a href={format!("#{}", util::slugify(heading))}>{heading}</a>
                {build_list_recursively(children, false)}
            </li>
        }
    }

    Some(build_list_recursively(&heading_hierarchy, true)).filter(|e| !e.is_empty())
}
