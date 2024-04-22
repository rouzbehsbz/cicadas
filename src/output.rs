use std::{fs::File, io::Write, path::PathBuf, sync::Arc};

use crate::{
    errors::{AppResult, ErrorType},
    storage::Storage,
};

trait Flushable {
    fn flush(&mut self, buffer: &[u8]) -> AppResult<()>;
}

pub struct Output<T>
where
    T: Flushable,
{
    buffer: String,
    writer: T,
}

impl<T> Output<T>
where
    T: Flushable,
{
    pub fn new(writer: T) -> Self {
        let buffer = String::new();

        Self { buffer, writer }
    }

    pub fn write(storage: Arc<Storage>) {}

    pub fn flush(&mut self) -> AppResult<()> {
        self.writer.flush(self.buffer.as_bytes())
    }
}

pub struct FileManager {
    file: File,
}

impl FileManager {
    pub fn new(path: PathBuf) -> AppResult<Self> {
        let file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Err(ErrorType::InvalidError),
        };

        Ok(Self { file })
    }
}

impl Flushable for FileManager {
    fn flush(&mut self, buffer: &[u8]) -> AppResult<()> {
        match self.file.write(buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorType::InvalidError),
        }
    }
}
