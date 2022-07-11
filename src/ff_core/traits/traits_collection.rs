use crate::ff_core::family::Family;
pub trait PointManipulator {
    fn add_points(&self, family: &mut Family, points: u32);
}
