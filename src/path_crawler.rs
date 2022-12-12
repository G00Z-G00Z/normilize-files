use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

// pub enum PathCrawlerError {
//     PathDoesNotExists,
// }

/// Path crawler that goes into a dir, and changes the contents
pub struct PathCrawler {
    /// Transform function to change the file names
    transform_function: fn(&str) -> String,
}

impl PathCrawler {
    /// Builds the object
    pub fn build(transform_function: fn(&str) -> String) -> PathCrawler {
        PathCrawler {
            transform_function: transform_function,
        }
    }

    /// Applies the transformation
    pub fn apply_transformations(&self, path: &str) -> Result<(), std::io::Error> {
        let path = Path::new(path);

        for file_result in path.read_dir()? {
            if let Err(_) = file_result {
                continue;
            }

            let file_result = file_result.unwrap();

            if self.is_hidden(file_result.file_name()) {
                continue;
            }

            let new_name = (&self.transform_function)(file_result.file_name().to_str().unwrap());

            let new_path = self.rename_file(file_result.path(), &new_name);

            fs::rename(file_result.path(), new_path.as_path())?;
        }

        Ok(())
    }

    /// Checks if file is hidden
    fn is_hidden(&self, file_os_ref: OsString) -> bool {
        let cosa = file_os_ref.into_string();

        if let Ok(s) = cosa {
            return s.starts_with(".");
        }

        true
    }

    /// Renames the file
    fn rename_file(&self, path: impl AsRef<Path>, new_name: &str) -> PathBuf {
        let path = path.as_ref();

        let mut result = path.to_owned();

        result.set_file_name(new_name);

        if let Some(ext) = path.extension() {
            result.set_extension(ext);
        }
        result
    }
}
