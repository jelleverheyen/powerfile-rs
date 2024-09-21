use crate::search::TemplateMetadata;
use crate::storage::TemplateError::ParseError;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct Template {
    pub metadata: TemplateMetadata,
    pub content: String,
}

fn parse_template(template_file: File) -> Result<Template, TemplateError> {
    let mut reader = BufReader::new(template_file);

    let raw_metadata = get_template_metadata(&mut reader)
        .map_err(TemplateError::from)?
        .and_then(|raw| YamlLoader::load_from_str(&raw).ok());

    let metadata = raw_metadata.map(|docs| {
        let doc = &docs[0];

        let parse_vec = |key: &str| -> Option<Vec<String>> {
            doc[key]
                .as_str()
                .map(|s| s.split_whitespace().map(String::from).collect())
        };

        TemplateMetadata::new(parse_vec("prefix"), parse_vec("suffix"), parse_vec("tags"))
    });

    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(TemplateError::from)?;

    Ok(Template {
        content,
        metadata: metadata.ok_or(ParseError("Failed to parse template metadata".to_string()))?,
    })
}

fn get_template_metadata<R: BufRead>(reader: &mut R) -> io::Result<Option<String>> {
    let mut in_block = false;
    let mut block_text = String::new();

    for line in reader.by_ref().lines() {
        let line = line?;

        // Look for the start of the block
        if line.trim() == "---" {
            if in_block {
                return Ok(Some(block_text));
            } else {
                // Start reading block content
                in_block = true;
                continue;
            }
        }

        // Collect the lines
        if in_block {
            block_text.push_str(&line);
            block_text.push('n');
        }
    }

    // If we finish reading the file without finding the second `---`
    Ok(None)
}
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
