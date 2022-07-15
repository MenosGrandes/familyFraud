use crate::questions::question::Questions;

#[derive(Debug)]
pub enum DBStatus {
    Opened,
    Closed,
    Other,
}
#[derive(Debug)]
pub enum DBError {
    CannotFindFile,
    Timeout,
}
#[derive(Debug)]
pub enum DBReadError {
    Empty,
    Timeout,
    DBClosed
}

pub trait DBConnection {
    fn connect(&mut self) -> Result<Option<DBStatus>, DBError>;
}
pub trait DBRead {
    fn read(&mut self) -> Result<Questions, DBReadError>;
}

pub trait ConnectAndReadFromDB: DBConnection + DBRead {}
