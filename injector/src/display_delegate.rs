use std::fmt::{Display, Error, Formatter};

pub struct DisplayDelegate<T: Fn (&mut Formatter) -> Result<(), Error>> {
    delegate: T,
}

impl <T: Fn(&mut Formatter) -> Result<(), Error>> DisplayDelegate<T> {
    pub fn new(delegate: T) -> Self {
        DisplayDelegate { delegate }
    }
}

impl <T: Fn(&mut Formatter) -> Result<(), Error>> Display for DisplayDelegate<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        (self.delegate)(formatter)
    }
}
