//! `domain::elements` never imports `domain::ports`: the model is pure data, and
//! the ports are served over it, never the other way round.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn rust_files_under(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            rust_files_under(&path, files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

#[test]
fn elements_do_not_import_ports() {
    let elements = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/elements");
    let mut files = vec![elements.with_extension("rs")];
    rust_files_under(&elements, &mut files).unwrap();

    let importers: Vec<&PathBuf> = files
        .iter()
        .filter(|file| {
            let source = fs::read_to_string(file).unwrap();
            source.contains("::ports") || source.contains("ports::")
        })
        .collect();
    assert!(
        importers.is_empty(),
        "elements must not import ports: {importers:?}"
    );
}
