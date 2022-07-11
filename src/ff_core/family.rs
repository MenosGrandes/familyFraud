#[derive(Debug)]
pub struct FamilyMember {
    name: String,
}
impl FamilyMember {
    pub fn new(name: &str) -> FamilyMember {
        FamilyMember {
            name: name.to_string(),
        }
    }
    fn no_name() -> FamilyMember {
        FamilyMember {
            name: "".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct Family {
    members: [FamilyMember; 5],
    points: u32,
    name: String,
}
impl Family {
    pub fn empty() -> Family {
        Family {
            members: ([
                FamilyMember::no_name(),
                FamilyMember::no_name(),
                FamilyMember::no_name(),
                FamilyMember::no_name(),
                FamilyMember::no_name(),
            ]),
            points: (0),
            name: String::new(),
        }
    }
    pub fn add_member(&mut self, member: FamilyMember, index: usize) {
        self.members[index] = member;
    }
    pub fn set_points(&mut self, points: u32) {
        self.points += points;
    }
    pub fn get_points(self) -> u32 {
        self.points
    }
}
