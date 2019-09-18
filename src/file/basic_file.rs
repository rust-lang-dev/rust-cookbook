use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Lines, Write};

pub fn test() {
    let path = "./test.log";
    // Write to file and read
    println!("{}", "Write and read");
    write_file(path, "Hello World").expect("Failed to write to file");
    let data = read_file(path).expect("Failed to read from file");
    println!("{}", data);

    // Overwrite
    println!("{}", "Overwriting file");
    write_file(path, "'Hello World' has been replaced").expect("Failed to write to file");
    let data = read_file(path).expect("Failed to read from file");
    println!("{}", data);

    // Append and read
    println!("{}", "Appending file");
    append_file(path, "\nAppend something here\n").expect("Failed to append");
    let lines = read_file_iterator(path).expect("Falied to read iterator");
    for line in lines {
        println!("{}", line.expect("Failed to read line"));
    }
}

fn read_file(path: &str) -> io::Result<String> {
    // open() opens the file in read-only mode
    let file = File::open(path)?;
    // Wrap the file in a BufReader
    // to read in an efficient way
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines())
}

fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_file(path: &str, content: &str) -> Result<(), io::Error> {
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}
