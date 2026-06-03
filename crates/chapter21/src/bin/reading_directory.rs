/*
The fs::read_dir function returns a io::Result<ReadDir> enum.
The ReadDir struct implements the Iterator trait.
The iterator yields Result<DirEntry, Error> enums.
The DirEntry struct supports a "path" method.
The fs::metadata function returns a Metadata struct.
The Metadata struct includes a "is_file" method.
The fs::read_to_string function returns a io::Result<String>.
*/

use std::fs;
// use std::process;
use std::io;

pub fn main() -> io::Result<()> {
    println!("Chapter 21: Reading directory");

    // let directory = fs::read_dir("./").unwrap_or_else(|error| {
    //   eprint!("Could not read directory: {error}");
    //   process::exit(1);
    // });

    // for entry_result in directory {
    //   match entry_result {
    //     Ok(entry) => println!("{:#?}", entry.path()),
    //     Err(error) => {
    //       eprintln!("Could not read entry: {error}");
    //     }
    //   }
    // }

    for entry_result in fs::read_dir("./")? {
        // skip error entries
        // if let Ok(entry) = entry_result {
        //   println!("{:#?}", entry.path());
        // }
        let entry = entry_result?; // exits program on error of each entry
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;
        if metadata.is_file() {
            println!("{entry_path:?}\n----------");
            let contents = fs::read_to_string(&entry_path)?;
            println!("{contents}");
        }
    }

    Ok(())
}
