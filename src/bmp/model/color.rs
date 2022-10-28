
mod color_conversion;
mod color_impl;
mod color_model;

pub use color_conversion::*;
pub use color_impl::*;
pub use color_model::*;

pub trait Color : Sized {
    ///
    /// Convert the Color to a u32,
    /// in either big or little endian.
    /// 
    fn to_u32(&self, big_endian: bool) -> u32;

    ///
    /// Create a Color from the given u32,
    /// in either big or little endian.
    /// 
    fn from_u32(value: u32, big_endian: bool) -> Self;

    ///
    /// Calculate the distance between self, and each color in the given set using the given algorithm, and return
    /// the closest color in the set, or none, if the given set is empty.
    /// 
    fn get_closest_in_set<'a, 'b>(&'a self, to_compare: &'b [Self], algorithm: fn(&Self, &Self) -> f32) -> Option<&'b Self>
    {
        let closest_tuple = to_compare.iter()
            .enumerate()
            .map(|(ndx, color)| (ndx, algorithm(self, color)))
            .reduce(|(ndxa, distancea), (ndxb, distanceb)| {
                if distancea <= distanceb {
                    (ndxa, distancea)
                }
                else {
                    (ndxb, distanceb)
                }
            });
        
        if let Some((closest_ndx, _)) = closest_tuple {
            Some(&to_compare[closest_ndx])
        }
        else {
            None
        }
    }

}