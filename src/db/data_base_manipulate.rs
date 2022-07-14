use crate::questions::question::Question;

pub trait DBConnection {
    fn connect(&mut self);
}
pub trait DBRead {
    fn read(&mut self) -> Result<Question, &'static str>;
}

pub trait ConnectAndReadFromDB: DBConnection + DBRead {}
