use std::path::PathBuf;
use walkdir::WalkDir;

struct Indexer {
    base_path: String,
}

impl Indexer {
    fn new(base_path: String) -> Indexer {
        Self { base_path }
    }

    pub fn load(&self) {
        let _template_paths = Self::find_templates(self.base_path.as_str());
    }

    fn find_templates(dir: &str) -> Vec<PathBuf> {
        let mut file_paths = Vec::new();

        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            // If the entry is a file, push it to the file_paths vector
            if path.is_file() {
                file_paths.push(path.to_path_buf());
            }
        }

        file_paths
    }
}
