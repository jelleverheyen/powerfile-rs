use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

const BLOCK_SIZE: usize = 128;

pub fn read_template_index(
    path: &str,
    indexes: &[usize],
) -> io::Result<Vec<(usize, Option<String>)>> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    let mut paths = Vec::with_capacity(indexes.len());

    for &index in indexes {
        let mut buffer = vec![0; BLOCK_SIZE];
        file.seek(io::SeekFrom::Start((index * BLOCK_SIZE) as u64))?;
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

pub fn write_template_index(index_path: &str, template_paths: Vec<&str>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(index_path)?;

    for path in template_paths {
        let bytes = path.as_bytes();
        if bytes.len() > BLOCK_SIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
        }

        let mut buffer = vec![0; BLOCK_SIZE];
        buffer[..bytes.len()].copy_from_slice(bytes);
        file.write_all(&buffer)?;
    }

    Ok(())
}
