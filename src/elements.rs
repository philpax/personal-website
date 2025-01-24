#![allow(unused)]

pub use paxhtml::{builder::Element, html};

use paxhtml::builder::*;

pub fn date_with_chrono(date: chrono::NaiveDate) -> Element {
    let date = date.to_string();
    time([
        ("datetime", date.as_str()).into(),
        ("title", date.as_str()).into(),
    ])(date.clone())
}

pub fn datetime_with_chrono<TZ: chrono::TimeZone>(date: chrono::DateTime<TZ>) -> Element {
    time([
        ("datetime", date.to_rfc3339()).into(),
        ("title", date.to_rfc2822()).into(),
    ])(date.to_rfc2822())
}

pub fn break_on_colon(value: &str) -> Element {
    Element::from_iter(value.split(": ").enumerate().map(|(i, s)| {
        let s = s.replace("-", "\u{2011}");
        if i > 0 {
            Element::from([
                text(": "),
                span([("style", "display: inline-block").into()])(s),
            ])
        } else {
            span([("style", "display: inline-block").into()])(s)
        }
    }))
}

/// Accepts a `with_link` attribute that will wrap the children in a link to the heading.
pub fn h_with_id<E: Into<Element>>(depth: u8, with_link: bool) -> impl FnOnce(E) -> Element {
    move |children: E| {
        let children = children.into();
        let id = crate::util::slugify(&children.inner_text());

        let children = if with_link {
            a([
                ("href", format!("#{id}")).into(),
                ("class", "no-underline").into(),
            ])(children)
        } else {
            children
        };

        tag(format!("h{depth}"), vec![("id", id).into()], false)(children)
    }
}
macro_rules! generate_hs_with_id {
    ($(($fn_ident:ident, $depth:literal)),*) => {
        $(
        /// Accepts a `with_link` attribute that will wrap the children in a link to the heading.
        pub fn $fn_ident(element: impl Into<Element>) -> Element {
            h_with_id($depth, false)(element)
        }
        )*
    };
}
generate_hs_with_id![
    (h1_with_id, 1),
    (h2_with_id, 2),
    (h3_with_id, 3),
    (h4_with_id, 4),
    (h5_with_id, 5),
    (h6_with_id, 6)
];
