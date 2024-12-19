use std::path::Path;

pub fn build(input_image: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let icon = image::open(input_image)?;

    icon.resize(128, 128, image::imageops::FilterType::Lanczos3)
        .save(output_dir.join("icon.png"))?;

    icon.resize(32, 32, image::imageops::FilterType::Lanczos3)
        .save(output_dir.join("favicon.ico"))?;

    Ok(())
}
