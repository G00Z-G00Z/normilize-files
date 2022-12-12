use std::{ffi::OsString, path::Path};

pub enum PathCrawlerError {
    PathDoesNotExists,
}

pub struct PathCrawler {
    transform_function: fn(&str) -> String,
}

impl PathCrawler {
    pub fn build(transform_function: fn(&str) -> String) -> Result<PathCrawler, PathCrawlerError> {
        Ok(PathCrawler {
            transform_function: transform_function,
        })
    }

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

            println!("Filename : {:?}", file_result.file_name());
        }

        Ok(())
    }

    pub fn is_hidden(&self, file_os_ref: OsString) -> bool {
        let cosa = file_os_ref.into_string();

        if let Ok(s) = cosa {
            return s.starts_with(".");
        }

        true
    }
}
