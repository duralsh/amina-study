use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct TxRecorder {
    pub path: String,
}

impl TxRecorder {
    // Constructs a new TxRecorder.
    pub fn new(path: &str) -> TxRecorder {
        TxRecorder {
            path: path.to_string(),
        }
    }

    // Appends content to the file specified by the path.
    pub fn append_to_file(&self, content: &str) -> io::Result<()> {
        // Open the file in append mode. Create it if it does not exist.
        let mut file = OpenOptions::new()
            .append(true) // Set the file to open in append mode.
            .create(true) // Create the file if it does not exist.
            .open(&self.path)?;

        // Write the content to the file.
        writeln!(file, "{}", content)?;

        Ok(())
    }
}