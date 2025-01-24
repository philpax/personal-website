use std::{collections::HashMap, path::Path, str::FromStr};

use syntect::{
    highlighting::ThemeSet,
    parsing::{syntax_definition::Context, Scope, SyntaxDefinition, SyntaxSet},
};

mod sublime_color_scheme;

pub fn build(source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_dir)?;
    build_syntax_set(source_dir, output_dir)?;
    build_theme_set(source_dir, output_dir)?;
    Ok(())
}

fn build_syntax_set(source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let mut builder = SyntaxSet::load_defaults_newlines().into_builder();
    builder.add(SyntaxDefinition::load_from_str(
        &std::fs::read_to_string(&source_dir.join("Pug.sublime-syntax"))?,
        true,
        None,
    )?);
    builder.add(SyntaxDefinition {
        name: "plaintext".to_string(),
        file_extensions: vec![],
        scope: Scope::new("text.plain")?,
        first_line_match: None,
        hidden: false,
        variables: Default::default(),
        contexts: HashMap::from_iter([("__start".to_string(), Context::new(false))]),
    });
    let set = builder.build();
    syntect::dumps::dump_to_file(&set, output_dir.join("syntax_set.packdump"))?;
    Ok(())
}

fn build_theme_set(source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let mut set = ThemeSet::new();
    let ayu_dark = sublime_color_scheme::ColorScheme::from_str(&std::fs::read_to_string(
        &source_dir.join("ayu-dark.sublime-color-scheme"),
    )?)?;
    set.themes
        .insert("ayu-dark".to_string(), ayu_dark.try_into()?);
    syntect::dumps::dump_to_file(&set, output_dir.join("theme_set.packdump"))?;
    Ok(())
}
