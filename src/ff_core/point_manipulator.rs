use crate::ff_core::traits::traits_collection::PointManipulator;
use crate::ff_core::family::Family;
pub struct FamilyPointManipulator;
impl PointManipulator for FamilyPointManipulator {
    fn add_points(&self, family: &mut Family, points: u32) {
        family.set_points(points);
    }
}
