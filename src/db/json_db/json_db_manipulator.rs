use crate::{
    db::data_base_manipulate::{ConnectAndReadFromDB, DBConnection, DBRead},
    questions::question::Question,
};
pub struct JsonDB {
    is_open: bool,
}
impl JsonDB {
    fn new() -> Self {
        JsonDB { is_open: false }
    }
}

// impl ConnectAndReadFromDB for JsonDB {}
// impl DBRead for JsonDB {
//     fn read(&mut self) -> Result<Question, &'static str> {
//         if self.is_open == false {
//             Err("JsonDB is not opened")
//         } else {
//         }
//     }
// }
// impl DBConnection for JsonDB {
//     fn connect(&mut self) {
//         self.is_open = true;
//     }
// }
