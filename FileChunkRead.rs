use std::fs::File;
use std::io::{self, Read};

fn main() {
  let file_path = "data.txt"; // Replace with your file path

  // Open the file
  let mut file = File::open(file_path).expect("Error opening file");

  // Allocate a buffer
  let mut buffer = [0; 1024];

  // Read the file in chunks
  loop {
    let bytes_read = file.read(&mut buffer).expect("Error reading");
    if bytes_read == 0 {
      break;
    }
    // Process the read bytes
  }
}
