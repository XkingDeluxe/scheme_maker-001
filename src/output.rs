use std::{fs::{File, OpenOptions}, io::{Read, Write, BufWriter, BufReader, BufRead, ErrorKind, Seek, SeekFrom}, cmp::max};
pub struct Output{
    file: File
}

impl Output{
    pub fn create(file_name: &str) -> Output{
        let csv_name = file_name.to_owned()+".csv";
        let file = OpenOptions::new().write(true).read(true).truncate(true).open(csv_name.clone());
        let create_file = match file {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => OpenOptions::new().write(true).read(true).create(true).open(csv_name).unwrap(),
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
        };
        Output{
            file: create_file
        }
    }

    pub fn write_line(&self, x:f64, y:f64){
        let mut writer = BufWriter::new(&self.file);
        let mut line = String::from(format!("{:.7}", x).as_str());
        line.push(';');
        line+=format!("{:.7}", y).as_str();
        line.push('\n');
        line = line.replace(".", ",");


        match writer.write_all(line.as_bytes()) {
            Ok(()) => (),
            Err(e) => panic!("{:?}", e)
        };
    }
}