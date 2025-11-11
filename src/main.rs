use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{fs, io};

fn main() -> io::Result<()> {
    let cwd = Path::new("./src/bin");

    let (py_files, rs_files) = collate_source_files(cwd)?;

    let stems: HashSet<_> = py_files.keys().chain(rs_files.keys()).collect();

    for &stem in &stems {
        process_stem(cwd, stem, &py_files, &rs_files)?;
    }

    Ok(())
}

type Bucket = HashMap<String, PathBuf>;

fn collate_source_files(dir: &Path) -> io::Result<(Bucket, Bucket)> {
    let mut py_files = Bucket::new();
    let mut rs_files = Bucket::new();

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        println!("{:#?}", &entry);
        let path = entry?.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str())
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            let stem = stem.to_string();

            match ext {
                "py" => py_files.insert(stem, path),
                "rs" => rs_files.insert(stem, path),
                _ => continue,
            };
        }
    }

    Ok((py_files, rs_files))
}

fn process_stem(cwd: &Path, stem: &str, py_files: &Bucket, rs_files: &Bucket) -> io::Result<()> {
    let target_dir = cwd.join(stem);
    fs::create_dir_all(&target_dir)?;

    let mut buffer = Vec::with_capacity(2);

    if let Some(filename) = py_files.get(stem) {
        fs::rename(filename, target_dir.join("main.py"))?;
        buffer.push("main.py");
    }

    if let Some(filename) = rs_files.get(stem) {
        fs::rename(filename, target_dir.join("main.rs"))?;
        buffer.push("main.rs");
    }

    println!("Created {stem}/ with {}", buffer.join(" and "));

    Ok(())
}

#[allow(unused)]
fn glob_by_ext(dir: &Path, ext: &str) -> std::io::Result<impl Iterator<Item = PathBuf>> {
    Ok(fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(move |p| p.extension().and_then(|s| s.to_str()) == Some(ext)))
}

#[allow(unused)]
fn stem(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}
