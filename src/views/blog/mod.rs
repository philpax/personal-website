use super::*;

pub mod partials;
pub mod tags;

pub fn index(context: ViewContext) -> paxhtml::Document {
    let all_posts = context
        .content
        .blog
        .documents
        .iter()
        .map(|doc| partials::post(context, doc, partials::PostBody::Description));
    layout(
        context,
        html! {
            <>
                {partials::header(partials::HeaderFocus::AllPosts)}
                #{all_posts}
            </>
        },
    )
}

pub fn post(context: ViewContext, document: &Document) -> paxhtml::Document {
    layout(
        context,
        partials::post(context, document, partials::PostBody::Full),
    )
}
