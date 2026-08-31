use crate::{error::Error};

pub struct Version(usize);

impl Version {
    pub fn new(value: usize) -> Result<Self, Error> {
        if (1..=40).contains(&value) {
            Ok(Version(value))
        } else {
            Err(Error::InvalidVersion)
        }
    }

    pub fn get(&self) -> usize {
        return self.0
    }

    pub fn get_size(&self) -> usize {
        match self.0 {
            1 => 21,
            _ => panic!("Not implemented yet")
        }
    }

}
