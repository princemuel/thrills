use std::{fs, io};

fn main() -> io::Result<()> {
    // i want to:
    // get all the names of rs and py files in the src/bin dir
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // i want to:
    // get all the names of rs and py files in the src/bin dir
    let entries: Vec<_> = fs::read_dir(".")?
        .filter_map(Result::ok)
        .filter_map(|e| {
            let path = e.path();
            match path.extension()?.to_str()? {
                "py" => Some((path.file_stem()?.to_string_lossy().into_owned(), true)),
                "rs" => Some((path.file_stem()?.to_string_lossy().into_owned(), false)),
                _ => None,
            }
        })
        .collect();

    println!("{:#?}", entries);

    //         let entries = fs::read_dir(".")?.filter_map(|res| {
    // res.ok().and_then(|e| e.path().)
    //         })       .collect::<Result<Vec<_>, io::Error>>()?;

    // create a dir for each name
    // move the files to their relevant dir
    // rename the files to main.rs and main.py

    Ok(())
}
