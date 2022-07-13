use crate::ff_core::family::*;

#[test]
fn family_get_name() {
    let mut family = Family::with_name("RandomName");
    let f_members = family.get_members();
    assert_eq!(f_members.len(), 5);
    assert_eq!(family.get_name().to_string(), "RandomName");
}
#[test]
fn family_members_add() {
    let mut family = Family::with_name("RandomName");
    family.add_member(FamilyMember::new("Ania"), 0);
    family.add_member(FamilyMember::new("Baba"), 1);
    family.add_member(FamilyMember::new("Blabl"), 2);
    family.add_member(FamilyMember::new("Random"), 3);
    family.add_member(FamilyMember::new("Jaroslaw"), 4);
    let f_members = family.get_members();
    assert_eq!(f_members.len(), 5);
}

#[test]
fn family_members_add_error() {
    let mut family = Family::with_name("RandomName");
    family.add_member(FamilyMember::new("Ania"), 0);
    family.add_member(FamilyMember::new("Baba"), 1);
    family.add_member(FamilyMember::new("Blabl"), 2);
    family.add_member(FamilyMember::new("Random"), 3);
    family.add_member(FamilyMember::new("Jaroslaw"), 4);
    family.add_member(FamilyMember::new("Jaroslaw 2"), 4);
    let f_members = family.get_members();
    assert_eq!(f_members.len(), 5);
    let last_member = f_members[4].get_name();
    assert_ne!(last_member.to_string(), "Jaroslaw".to_string());
    assert_eq!(last_member.to_string(), "Jaroslaw 2".to_string());
}
