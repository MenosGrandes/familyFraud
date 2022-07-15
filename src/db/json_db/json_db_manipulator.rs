use crate::{
    db::data_base_manipulate::{
        ConnectAndReadFromDB, DBConnection, DBError, DBRead, DBReadError, DBStatus,
    },
    questions::question::Questions,
};
pub struct JsonDB {
    is_open: bool,
    db_file: String,
}
impl JsonDB {
    pub fn new(db_file: &str) -> Self {
        JsonDB {
            is_open: false,
            db_file: db_file.to_string(),
        }
    }
}

impl ConnectAndReadFromDB for JsonDB {}
impl DBRead for JsonDB {
    fn read(&mut self) -> Result<Questions, DBReadError> {
        if self.is_open == false {
            Err(DBReadError::DBClosed)
        } else {
            let json_insides = {
                let json_file = std::fs::read_to_string(&self.db_file).unwrap();
                serde_json::from_str::<Questions>(&json_file).unwrap()
            };
            Ok(json_insides)
        }
    }
}
impl DBConnection for JsonDB {
    fn connect(&mut self) -> Result<Option<DBStatus>, DBError> {
        self.is_open = true;
        Ok(Some(DBStatus::Opened))
    }
}
