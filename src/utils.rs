use std::{
    fs::{copy, create_dir_all, read_dir},
    io::Result,
    path::Path,
};

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    create_dir_all(dst)?;
    for entry in read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
