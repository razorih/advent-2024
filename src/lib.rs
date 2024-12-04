use std::{io, path::{Path, PathBuf}};

pub mod grid;

pub fn read_input() -> io::Result<String> {
    let filename = get_filename_from_args()?;
    let resolved = resolve_path(&filename)?;

    std::fs::read_to_string(resolved)
}

fn get_filename_from_args() -> Result<String, io::Error> {
    std::env::args().nth(1)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::Other,
            r#"expected input file path or "-" as first argument"#
        ))
}

fn resolve_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let mut base = std::env::current_dir()?;
        base.push("inputs/");
        base.push(path);
        base
    }.canonicalize()
}
