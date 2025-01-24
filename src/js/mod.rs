use crate::Route;

#[cfg(feature = "serve")]
fn reload() -> anyhow::Result<String> {
    Ok(include_str!("reload.js").to_string())
}

pub fn generate() -> anyhow::Result<String> {
    Ok([
        #[cfg(feature = "serve")]
        reload()?,
        include_str!("website.js")
            .replace("DARK_MODE_ICON", &Route::DarkModeIcon.url_path())
            .replace("LIGHT_MODE_ICON", &Route::LightModeIcon.url_path()),
    ]
    .join("\n"))
}
