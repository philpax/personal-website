use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
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
    pub fn theme_dark(&self) -> &str {
        "ayu-dark"
    }

    pub fn theme_light(&self) -> &str {
        "ayu-light"
    }

    fn theme_css_for(&self, theme: &str) -> String {
        css_for_theme_with_class_style(&self.theme_set.themes[theme], ClassStyle::Spaced).unwrap()
    }

    pub fn theme_dark_css(&self) -> String {
        self.theme_css_for(self.theme_dark())
    }

    pub fn theme_light_css(&self) -> String {
        self.theme_css_for(self.theme_light())
    }

    pub fn highlight_code(
        &self,
        language: Option<&str>,
        code: &str,
    ) -> Result<paxhtml::Element, syntect::Error> {
        let syntax = language
            .and_then(|l| self.syntax_set.find_syntax_by_token(l))
            .unwrap_or_else(|| self.syntax_set.find_syntax_by_name("plaintext").unwrap());
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
