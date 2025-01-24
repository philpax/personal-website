use super::*;
use crate::util;

pub fn index(context: ViewContext) -> paxhtml::Document {
    let mut tag_keys = context.content.blog.tags.keys().collect::<Vec<_>>();
    tag_keys.sort();

    layout(
        context,
        html! {
            <>
                {partials::header(partials::HeaderFocus::Tags)}
                <div>
                    <ul>
                    #{
                        tag_keys.iter().map(|tag| {
                            let post_count = context.content.blog.tags[*tag].len();
                            html! {
                                <li>
                                    <a href={Route::BlogTag { tag_id: tag }.url_path()}>{format!("#{tag}")}</a>
                                    {format!(
                                        " ({} {})",
                                        post_count,
                                        util::pluralize("post", post_count)
                                    )}
                                </li>
                            }
                        })
                    }
                    </ul>
                </div>
            </>
        },
    )
}

pub fn tag(context: ViewContext, tag_id: &str) -> paxhtml::Document {
    let blog = &context.content.blog;
    layout(
        context,
        html! {
            <>
                {partials::header(partials::HeaderFocus::Tag(tag_id))}
                #{
                    blog.tags[tag_id].iter().map(|t| {
                        let doc = blog.document_by_id(t).unwrap();
                        partials::post(context, doc, partials::PostBody::Description)
                    })
                }
            </>
        },
    )
}
