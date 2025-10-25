#![allow(dead_code)]

#[derive(Debug)]
struct MyFile {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<MyFile>,
}

impl Folder {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            contents: Vec::new(), // or vec![]
        }
    }

    fn create_file(&mut self, name: &str) {
        let file = MyFile {
            name: String::from(name),
        };
        self.contents.push(file);
    }

    fn delete_file(&mut self, index: usize) -> Result<MyFile, String> {
        if index < self.contents.len() {
            Ok(self.contents.remove(index))
        } else {
            Err(String::from("File index out of bounds"))
        }
    }

    fn get_file(&self, index: usize) -> Option<&MyFile> {
        self.contents.get(index)
    }
}

pub fn chapter13_project_solution() {
    let mut my_folder = Folder::new("My Documents");
    my_folder.create_file("Resume.docx");
    my_folder.create_file("CoverLetter.docx");
    let _ = my_folder.delete_file(1);
    println!("{:#?}", my_folder);
    let file = my_folder.get_file(0);
    match file {
        Some(f) => println!("File found: {:?}", f),
        None => println!("File not found"),
    }
}
