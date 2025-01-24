use crate::{elements as e, syntax::SyntaxHighlighter};
use paxhtml::builder as b;

pub use markdown::mdast::Node;

pub struct MarkdownConverter<'a> {
    pub syntax: &'a SyntaxHighlighter,
}
impl<'a> MarkdownConverter<'a> {
    pub fn new(syntax: &'a SyntaxHighlighter) -> Self {
        Self { syntax }
    }

    pub fn convert(&self, node: &Node) -> paxhtml::Element {
        match node {
            Node::Root(r) => {
                paxhtml::Element::from_iter(r.children.iter().map(|n| self.convert(n)))
            }

            Node::Heading(h) => {
                e::h_with_id((h.depth + 2).min(6), true)(self.convert_many(&h.children))
            }
            Node::Text(t) => b::text(&t.value),
            Node::Paragraph(p) => b::p([])(self.convert_many(&p.children)),
            Node::Strong(s) => b::strong([])(self.convert_many(&s.children)),
            Node::Emphasis(e) => b::em([])(self.convert_many(&e.children)),
            Node::List(l) => {
                let children = self.convert_many(&l.children);
                if l.ordered {
                    b::ol([])(children)
                } else {
                    b::ul([])(children)
                }
            }
            Node::ListItem(li) => {
                // hack: if the only child of this list item is a paragraph, drop the paragraph
                // and use the raw content instead
                if li.children.len() == 1 {
                    if let Node::Paragraph(p) = &li.children[0] {
                        return b::li([])(self.convert_many(&p.children));
                    }
                }

                b::li([])(self.convert_many(&li.children))
            }
            Node::Code(c) => b::pre([("class", "code").into()])(b::code([])(
                self.syntax
                    .highlight_code(c.lang.as_deref(), &c.value)
                    .unwrap(),
            )),
            Node::BlockQuote(b) => b::blockquote([])(self.convert_many(&b.children)),
            Node::Break(_) => b::br([]),
            Node::InlineCode(c) => b::code([("class", "code").into()])(
                self.syntax.highlight_code(None, &c.value).unwrap(),
            ),
            Node::Image(i) => {
                b::img([("src", i.url.clone()).into(), ("alt", i.alt.clone()).into()])
            }
            Node::Link(l) => {
                let mut attrs = vec![("href", l.url.clone()).into()];
                if let Some(title) = &l.title {
                    attrs.push(("title", title.clone()).into());
                }

                b::a(attrs)(paxhtml::Element::from_iter(
                    l.children.iter().map(|n| self.convert(n)),
                ))
            }
            Node::Html(h) => {
                // HACK: Strip comments from Markdown HTML. This won't work if the comment is closed
                // in the middle of the string and actual content follows, but it's good enough for now.
                if h.value.starts_with("<!--") && h.value.ends_with("-->") {
                    return paxhtml::Element::Empty;
                }

                paxhtml::Element::Raw {
                    html: h.value.clone(),
                }
            }

            // Not supported yet
            Node::FootnoteDefinition(_)
            | Node::InlineMath(_)
            | Node::Delete(_)
            | Node::FootnoteReference(_)
            | Node::ImageReference(_)
            | Node::LinkReference(_)
            | Node::Math(_)
            | Node::Table(_)
            | Node::ThematicBreak(_)
            | Node::TableRow(_)
            | Node::TableCell(_)
            | Node::Definition(_) => paxhtml::Element::Empty,

            // Never supported
            Node::Toml(_)
            | Node::Yaml(_)
            | Node::MdxJsxFlowElement(_)
            | Node::MdxjsEsm(_)
            | Node::MdxTextExpression(_)
            | Node::MdxJsxTextElement(_)
            | Node::MdxFlowExpression(_) => paxhtml::Element::Empty,
        }
    }

    fn convert_many(&self, nodes: &[Node]) -> paxhtml::Element {
        paxhtml::Element::from_iter(nodes.iter().map(|n| self.convert(n)))
    }
}

fn inner_text(node: &Node) -> String {
    if let Node::Text(text) = node {
        text.value.clone()
    } else {
        node.children()
            .map(|c| c.iter().map(inner_text).collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeadingHierarchy {
    pub heading: String,
    pub children: Vec<HeadingHierarchy>,
}
impl HeadingHierarchy {
    pub fn new(
        heading: impl Into<String>,
        children: impl IntoIterator<Item = HeadingHierarchy>,
    ) -> Self {
        Self {
            heading: heading.into(),
            children: children.into_iter().collect(),
        }
    }
    pub fn from_node(node: &Node) -> Vec<HeadingHierarchy> {
        let mut headings = Vec::new();
        collect_headings(node, &mut headings);

        let mut result = Vec::new();
        let mut stack: Vec<(u8, HeadingHierarchy)> = Vec::new();

        for (depth, heading) in headings {
            while let Some((prev_depth, _)) = stack.last() {
                if *prev_depth >= depth {
                    let (_prev_depth, finished_heading) = stack.pop().unwrap();
                    if let Some((_, parent_heading)) = stack.last_mut() {
                        parent_heading.children.push(finished_heading);
                    } else {
                        // If there's no parent, this is a root-level heading
                        result.push(finished_heading);
                    }
                } else {
                    break;
                }
            }
            stack.push((depth, HeadingHierarchy::new(heading, [])));
        }

        // Clear any remaining headings in the stack
        while let Some((_, finished_heading)) = stack.pop() {
            if let Some((_, parent_heading)) = stack.last_mut() {
                parent_heading.children.push(finished_heading);
            } else {
                result.push(finished_heading);
            }
        }

        fn collect_headings(node: &Node, headings: &mut Vec<(u8, String)>) {
            if let Some(children) = node.children() {
                for child in children {
                    if let Node::Heading(heading) = child {
                        headings.push((heading.depth, inner_text(child)));
                    }
                    collect_headings(child, headings); // Recurse into all children
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::parse_markdown;

    #[test]
    fn test_heading_hierarchy() {
        use HeadingHierarchy as HH;

        let input = r#"
# test
## test123
### test456
## test789
# test2
"#
        .trim();

        let ast = parse_markdown(input);

        assert_eq!(
            HH::from_node(&ast),
            vec![
                HH::new(
                    "test",
                    [
                        HH::new("test123", [HH::new("test456", [])]),
                        HH::new("test789", []),
                    ],
                ),
                HH::new("test2", []),
            ]
        )
    }
}
