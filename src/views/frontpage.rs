use super::*;

use crate::{markdown::MarkdownConverter, views::blog::partials};

pub fn index(context: ViewContext) -> paxhtml::Document {
    let content = &context.content;
    let list_88x31 = [
        ("philpax.png", "https://philpax.me"),
        ("ackwell.png", "https://ackwell.au"),
        ("arcanedisgea.png", "https://arcanedisgea.com"),
        ("blooym.webp", "https://blooym.dev"),
        ("lun4.gif", "https://l4.pm"),
        ("notnite.png", "https://notnite.com"),
        ("goatcorp.png", "https://goatcorp.github.io"),
        ("88x31.png", "https://eightyeightthirty.one"),
    ];

    layout(
        context,
        html! {
            <div id="home-page-columns">
                <article>
                    <div class="post-body">
                        {MarkdownConverter::new(context.syntax).convert(&content.about.description)}
                    </div>
                </article>
                <div>
                    #{
                        content
                            .blog
                            .documents
                            .iter()
                            .take(5)
                            .map(|doc| partials::post(context, doc, partials::PostBody::Short))
                    }
                    <span>
                        <a href={Route::Blog.url_path()}>"All posts"</a>
                        " · "
                        <a href={Route::BlogTags.url_path()}>"Tags"</a>
                    </span>

                    <div id="list-88x31">
                    #{
                        list_88x31.iter().map(|(img, url)| html! {
                            <a href={url}>
                                <img src={format!("/88x31/{img}")} alt={img} />
                            </a>
                        })
                    }
                    </div>
                </div>
            </div>
        },
    )
}
