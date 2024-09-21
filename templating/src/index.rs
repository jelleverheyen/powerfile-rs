use crate::index::TemplateError::ParseError;
use crate::search::{TemplateEngine, TemplateMetadata};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write};
use std::path::PathBuf;
use uuid::Uuid;
use walkdir::WalkDir;
use yaml_rust::YamlLoader;

pub struct TemplateOptions {
    pub template_source_dir: PathBuf,
    pub cached_templates_dir: PathBuf,
    pub index_path: PathBuf,
    pub block_size: usize,
}

pub struct TemplateIndex {
    templates: Vec<CachedTemplate>,
    options: TemplateOptions,
}

impl TemplateIndex {
    fn new(options: TemplateOptions) -> Self {
        TemplateIndex {
            options,
            templates: Vec::new(),
        }
    }

    pub fn build(options: TemplateOptions) -> Self {
        let paths = scan_template_dir(options.template_source_dir.to_path_buf());
        //fs::create_dir_all("/some/dir");

        let cache_results: Vec<_> = paths
            .into_iter()
            .map(|path| {
                let output_path = options
                    .cached_templates_dir
                    .join(Uuid::new_v4().to_string());
                cache_template(path, output_path)
            })
            .collect();

        let mut templates = Vec::new();
        let mut errs = Vec::new();
        for cache in cache_results {
            match cache {
                Ok(x) => {
                    templates.push(x);
                }
                Err(e) => errs.push(e),
            }
        }

        // TODO: Handle Errors

        TemplateIndex { templates, options }
    }

    pub fn write(&self) -> io::Result<()> {
        let mut file = File::open(&self.options.cached_templates_dir)?;

        let paths = self.templates.iter().map(|t| &t.path).collect::<Vec<_>>();
        for path in paths {
            let temp = path.to_string_lossy();
            let bytes = temp.as_bytes();
            if bytes.len() > self.options.block_size {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
            }

            let mut buffer = vec![0; self.options.block_size];
            buffer[..bytes.len()].copy_from_slice(bytes);
            file.write_all(&buffer)?;
        }

        Ok(())
    }

    pub fn get_templates_path(
        &self,
        indexes: &mut [usize],
    ) -> io::Result<Vec<(usize, Option<String>)>> {
        let mut file = File::open(&self.options.index_path)?;

        let mut paths = Vec::with_capacity(indexes.len());

        for &mut index in indexes {
            let mut buffer = vec![0; self.options.block_size];
            file.seek(io::SeekFrom::Start(
                (index * self.options.block_size) as u64,
            ))?;
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 {
                paths.push((index, None));
            } else {
                let template_path = String::from_utf8_lossy(&buffer[..bytes_read])
                    .trim_end_matches(char::from(0)) // Remove filler bytes
                    .trim()
                    .to_string();

                match template_path.is_empty() {
                    true => paths.push((index, None)),
                    false => paths.push((index, Some(template_path))),
                }
            }
        }

        Ok(paths)
    }

    pub fn to_engine(&self) -> TemplateEngine {
        let metadata = self
            .templates
            .iter()
            .map(|t| t.metadata.clone())
            .collect::<Vec<_>>();
        TemplateEngine::from_templates(metadata)
    }
}

fn get_raw_metadata<R: BufRead>(reader: &mut R) -> Result<String, TemplateError> {
    let mut in_block = false;
    let mut block_text = String::new();

    for line in reader.by_ref().lines() {
        let line = line?;

        // Look for the start of the block
        if line.trim() == "---" {
            if in_block {
                return Ok(block_text);
            } else {
                // Start reading block content
                in_block = true;
                continue;
            }
        }

        // Collect the lines
        if in_block {
            block_text.push_str(&line);
            block_text.push('\n');
        }
    }

    // If we finish reading the file without finding the second `---`
    Err(ParseError("No template metadata found".to_string()))
}

fn scan_template_dir(dir: PathBuf) -> Vec<PathBuf> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            file_paths.push(path.to_path_buf());
        }
    }

    file_paths
}

pub struct Template {
    pub metadata: TemplateMetadata,
    pub content: String,
}

pub struct CachedTemplate {
    path: PathBuf,
    metadata: TemplateMetadata,
}

fn cache_template(
    source_path: PathBuf,
    output_path: PathBuf,
) -> Result<CachedTemplate, TemplateError> {
    let source_file = File::open(source_path).map_err(TemplateError::from)?;
    let mut reader = BufReader::new(source_file);

    let yaml = get_raw_metadata(&mut reader)?;
    let metadata = parse_metadata(&yaml)?;

    let output_file = File::create(&output_path).map_err(TemplateError::from)?;
    let mut writer = BufWriter::new(output_file);
    io::copy(&mut reader, &mut writer)?;

    writer.flush().map_err(TemplateError::from)?;

    Ok(CachedTemplate {
        metadata,
        path: output_path,
    })
}

fn parse_metadata(raw_metadata: &str) -> Result<TemplateMetadata, TemplateError> {
    let raw = YamlLoader::load_from_str(raw_metadata).unwrap();
    //.map_err(|err| ParseError(err.to_string()))?;

    let doc = &raw[0];

    let parse_vec = |key: &str| -> Option<Vec<String>> {
        doc[key]
            .as_str()
            .map(|s| s.split_whitespace().map(String::from).collect())
    };

    Ok(TemplateMetadata::new(
        parse_vec("prefix"),
        parse_vec("suffix"),
        parse_vec("tags"),
    ))
}

// pub fn write_template_index(index_path: &str, template_paths: Vec<&str>) -> io::Result<()> {
//     let mut file = File::open(index_path)?;
//
//     for path in template_paths {
//         let bytes = path.as_bytes();
//         if bytes.len() > BLOCK_SIZE {
//             return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
//         }
//
//         let mut buffer = vec![0; BLOCK_SIZE];
//         buffer[..bytes.len()].copy_from_slice(bytes);
//         file.write_all(&buffer)?;
//     }
//
//     Ok(())
// }

#[derive(Debug)]
enum TemplateError {
    IoError(io::Error),
    ParseError(String),
}

impl From<io::Error> for TemplateError {
    fn from(err: io::Error) -> TemplateError {
        TemplateError::IoError(err)
    }
}
