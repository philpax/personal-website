use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

pub struct SyntaxHighlighter {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            syntax_set: syntect::dumps::from_binary(include_bytes!(
                "../../assets/baked/syntax/syntax_set.packdump"
            )),
            theme_set: syntect::dumps::from_binary(include_bytes!(
                "../../assets/baked/syntax/theme_set.packdump"
            )),
        }
    }
}
impl SyntaxHighlighter {
    pub fn theme(&self) -> &str {
        "ayu-dark"
    }

    pub fn theme_css(&self) -> String {
        css_for_theme_with_class_style(&self.theme_set.themes[self.theme()], ClassStyle::Spaced)
            .unwrap()
    }

    pub fn lookup_language(&self, language: Option<&str>) -> &SyntaxReference {
        language
            .and_then(|l| self.syntax_set.find_syntax_by_token(l))
            .unwrap_or_else(|| self.syntax_set.find_syntax_by_name("plaintext").unwrap())
    }
    pub fn highlight_code(
        &self,
        language: Option<&str>,
        code: &str,
    ) -> Result<paxhtml::Element, syntect::Error> {
        let syntax = self.lookup_language(language);
        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &self.syntax_set,
            ClassStyle::Spaced,
        );
        for line in LinesWithEndings::from(code) {
            html_generator.parse_html_for_line_which_includes_newline(line)?;
        }
        Ok(paxhtml::Element::Raw {
            html: html_generator.finalize(),
        })
    }
}
