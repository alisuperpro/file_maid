pub mod rename_file {
    use std::fs;

    pub fn rename_file(path: String, new_name: String) -> std::io::Result<()> {
        fs::rename(path, new_name)?;
        Ok(())
    }
}
