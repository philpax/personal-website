use crate::{content::Document, elements::*, Route, ViewContext};

pub mod blog;
pub mod frontpage;

fn layout(_context: ViewContext, inner: Element) -> paxhtml::Document {
    let links = [
        (Route::Index.url_path(), "Home"),
        (Route::Blog.url_path(), "Blog"),
    ];

    paxhtml::Document::new([
        paxhtml::builder::doctype(["html".into()]),
        html! {
            <html lang="en-AU">
                <head>
                    <title>"Philpax"</title>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <link rel="alternate" href={Route::BlogRss.url_path()} r#type="application/rss+xml" title="Philpax's Blog" />
                    <link rel="stylesheet" href={Route::Styles.url_path()} />
                    <script src={Route::Scripts.url_path()}></script>
                </head>
                <body>
                    <header>
                        <div id="top-bar">
                            <div class="flair" id="flair-left"></div>
                            <img src={Route::Icon.url_path()} alt="Philpax icon" />
                            <h1>"Philpax"</h1>
                            <div class="flair" id="flair-right"></div>
                        </div>
                        <nav>
                            <ul id="header-links">
                            #{
                                links.iter().map(|(url, label)| { html! {
                                    <li>
                                        <a class="no-underline" href={url}>{*label}</a>
                                    </li>
                                }})
                            }
                            </ul>
                        </nav>
                    </header>
                    <main>{inner}</main>
                </body>
            </html>
        },
    ])
}

pub fn redirect(to_url: &str) -> paxhtml::Document {
    paxhtml::Document::new([
        paxhtml::builder::doctype(["html".into()]),
        html! {
            <html lang="en-AU">
                <head>
                    <title>"Redirecting..."</title>
                    <meta charset="utf-8" />
                    <meta httpEquiv="refresh" content={format!("0; url={to_url}")} />
                </head>
                <body>
                    <p>"Redirecting..."</p>
                    <p>
                        <a href={to_url} title="Click here if you are not redirected">
                            "Click here"
                        </a>
                    </p>
                </body>
            </html>
        },
    ])
}
