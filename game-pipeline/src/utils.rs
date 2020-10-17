use std::{error::Error, fs, path::Path};

pub(crate) fn write_file_if_changed(
    path: impl AsRef<Path>,
    content: impl AsRef<str>,
) -> Result<(), Box<dyn Error>> {
    let s = match fs::read_to_string(path.as_ref()) {
        Ok(s) => s,
        Err(_) => {
            return fs::write(path.as_ref(), content.as_ref())
                .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
        }
    };

    if s != content.as_ref() {
        return fs::write(path.as_ref(), content.as_ref())
            .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
    }

    Ok(())
}

pub(crate) fn write_binary_file_if_changed(
    path: impl AsRef<Path>,
    content: impl AsRef<[u8]>,
) -> Result<(), Box<dyn Error>> {
    let s = match fs::read(path.as_ref()) {
        Ok(s) => s,
        Err(_) => {
            return fs::write(path.as_ref(), content.as_ref())
                .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
        }
    };

    if s != content.as_ref() {
        return fs::write(path.as_ref(), content.as_ref())
            .map_err(|e| format!("Unable to write {:?}: {}", path.as_ref(), e).into());
    }

    Ok(())
}
