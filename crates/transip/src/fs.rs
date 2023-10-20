use crate::{error::ResultExt, Result};
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
        OpenOptions::new().read(true).open(self).err_into()
    }

    fn writer(&self) -> Result<File> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(self)
            .err_into()
    }
}
