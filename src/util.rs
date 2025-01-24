use std::path::Path;

pub use paxhtml::util::slugify;

pub fn pluralize(s: &str, count: usize) -> String {
    if count == 1 {
        s.to_string()
    } else {
        format!("{}s", s)
    }
}

pub fn copy_dir(source: &Path, destination: &Path) -> std::io::Result<()> {
    if !source.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The source path is not a directory.",
        ));
    }

    if !destination.exists() {
        std::fs::create_dir_all(destination)?;
    }

    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let destination = destination.join(file_name);

        let path = entry.path();
        if file_type.is_dir() {
            copy_dir(&path, &destination)?;
        } else {
            std::fs::copy(&path, &destination)?;
        }
    }

    Ok(())
}
