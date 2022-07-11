use crate::ff_core::family::{Family, FamilyMember};
use crate::ff_core::traits::traits_collection::PointManipulator;
use crate::ff_core::point_manipulator::FamilyPointManipulator;
pub mod ff_core;

fn main() {
    let point_manipulator = FamilyPointManipulator;
    let mut family_1 = Family::empty();
    family_1.add_member(FamilyMember::new("Grzegorz"), 0);
    family_1.add_member(FamilyMember::new("Wacek"), 1);
    family_1.add_member(FamilyMember::new("Placek"), 2);
    family_1.add_member(FamilyMember::new("Jacek"), 3);
    family_1.add_member(FamilyMember::new("Anna"), 4);

    point_manipulator.add_points(&mut family_1, 1100);
    let points = family_1.get_points();
    println!("{points}");
}
