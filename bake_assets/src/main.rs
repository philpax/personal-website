use std::path::Path;

mod syntax;

fn main() -> anyhow::Result<()> {
    let static_dir = Path::new("assets/baked/static");
    std::fs::create_dir_all(static_dir)?;

    syntax::build(
        Path::new("assets/source/syntax"),
        Path::new("assets/baked/syntax"),
    )?;
    icon::build(Path::new("assets/source/icon.png"), static_dir)?;
    Ok(())
}

mod icon {
    use std::path::Path;

    pub fn build(input_image: &Path, output_dir: &Path) -> anyhow::Result<()> {
        let icon = image::open(input_image)?;

        icon.resize(128, 128, image::imageops::FilterType::Lanczos3)
            .save(output_dir.join("icon.png"))?;

        icon.resize(32, 32, image::imageops::FilterType::Lanczos3)
            .save(output_dir.join("favicon.ico"))?;

        Ok(())
    }
}
