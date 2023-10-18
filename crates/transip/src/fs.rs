use crate::Result;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub trait FileSystem {
    fn reader(&self) -> Result<File>;
    fn writer(&self) -> Result<File>;
}

impl<P> FileSystem for P
where
    P: AsRef<Path>,
{
    fn reader(&self) -> Result<File> {
        OpenOptions::new().read(true).open(self).map_err(Into::into)
    }

    fn writer(&self) -> Result<File> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(self)
            .map_err(Into::into)
    }
}
