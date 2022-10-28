const LAB_REF_1: f32 = 1_f32;
const LAB_REF_2: f32 = 1_f32;
const LAB_REF_3: f32 = 1_f32;

use super::{Color, color_model::*};

struct LabConvert<T>
    where T : Color 
{
    color: T,
    refs: (f32, f32, f32)
}

impl From<&RGBColor> for XYZColor {
    fn from(item: &RGBColor) -> Self {
        fn adj(channel: f32) -> f32 {
            let scaled = channel / 255_f32;
            if scaled > 0.04045 {
                ((scaled + 0.055) / 1.055).powf(2.4)
            }
            else {
                scaled / 12.92
            }
        }

        let var_r: f32 = adj(item.red as f32);
        let var_g: f32 = adj(item.green as f32);
        let var_b: f32 = adj(item.blue as f32);

        Self {
            x: var_r * 0.4124 + var_g * 0.3576 + var_b * 0.1805,
            y: var_r * 0.2126 + var_g * 0.7152 + var_b * 0.0722,
            z: var_r * 0.0193 + var_g * 0.1192 + var_b * 0.9505,
            alpha: item.alpha
        }
    }
}

impl From<&LabConvert<XYZColor>> for LABColor {
    fn from(item: &LabConvert<XYZColor>) -> Self {
        fn adj(channel: f32) -> f32 {
            if channel > 0.008856 {
                channel.powf(1_f32 / 3_f32)
            }
            else {
                (7.787 * channel) + (16_f32 / 116_f32)
            }
        }

        let var_x = adj(item.color.x / item.refs.0);
        let var_y = adj(item.color.y / item.refs.1);
        let var_z = adj(item.color.z / item.refs.2);

        Self {
            l: (116_f32 * var_y) - 16_f32,
            a: 500_f32 * (var_x - var_y),
            b: 200_f32 * (var_y - var_z),
            alpha: item.color.alpha
        }
    }
}

impl From<&LabConvert<RGBColor>> for LABColor {
    fn from(item: &LabConvert<RGBColor>) -> Self {
        let temp_xyz = XYZColor::from(&item.color);
        Self::from(&LabConvert::<XYZColor> {
            color: temp_xyz,
            refs: item.refs
        })
    }
}

impl RGBColor {
    ///
    /// Calculate the euclidean distance between self to other (rgb)
    ///
    pub fn get_euclidean_distance_rgb(&self, other: &RGBColor) -> f32 {
        let rgba_self = (self.red as f32, self.green as f32, self.blue as f32, self.alpha as f32);
        let rgba_other = (other.red as f32, other.green as f32, other.blue as f32, self.alpha as f32);

        f32::sqrt(
            (rgba_self.0 - rgba_other.0).powi(2)
            + (rgba_self.1 - rgba_other.1).powi(2)
            + (rgba_self.2 - rgba_other.2).powi(2)
            //+ (rgba_self.3 - rgba_other.3).powi(2) 
        )
    }

    ///
    /// Calculate the manhattan distance between self to other (rgb)
    ///
    pub fn get_manhattan_distance_rgb(&self, other: &RGBColor) -> f32 {
        let rgba_self = (self.red as f32, self.green as f32, self.blue as f32, self.alpha as f32);
        let rgba_other = (other.red as f32, other.green as f32, other.blue as f32, self.alpha as f32);

        (rgba_self.0 - rgba_other.0).abs()
        + (rgba_self.1 - rgba_other.1).abs()
        + (rgba_self.2 - rgba_other.2).abs()
        //+ (rgba_self.3 - rgba_other.3).abs()
    }
    
    ///
    /// Calculate the euclidean distance between self to other (xyz)
    ///
    pub fn get_euclidean_distance_xyz(&self, other: &RGBColor) -> f32 {
        let xyz_self = XYZColor::from(self);
        let xyz_other = XYZColor::from(other);

        f32::sqrt(
            (xyz_self.x - xyz_other.x).powi(2)
            + (xyz_self.y - xyz_other.y).powi(2)
            + (xyz_self.z - xyz_other.z).powi(2)
        )
    }

    ///
    /// Calculate the manhattan distance between self to other (xyz)
    ///
    pub fn get_manhattan_distance_xyz(&self, other: &RGBColor) -> f32 {
        let xyz_self = XYZColor::from(self);
        let xyz_other = XYZColor::from(other);

        (xyz_self.x - xyz_other.x).abs()
        + (xyz_self.y - xyz_other.y).abs()
        + (xyz_self.z - xyz_other.z).abs()
    }
  
    ///
    /// Calculate the euclidean distance between self to other (L*a*b*)
    ///
    pub fn get_euclidean_distance_lab(&self, other: &RGBColor) -> f32 {
        let reference_divisors = (LAB_REF_1, LAB_REF_2, LAB_REF_3);

        let lab_self = LABColor::from(&LabConvert::<RGBColor> {
            color: self.clone(),
            refs: reference_divisors
        });
        let lab_other = LABColor::from(&LabConvert::<RGBColor> {
            color: other.clone(),
            refs: reference_divisors
        });

        f32::sqrt(
            (lab_self.l - lab_other.l).powi(2)
            + (lab_self.a - lab_other.a).powi(2)
            + (lab_self.b - lab_other.b).powi(2)
        )
    }

    ///
    /// Calculate the manhattan distance between self to other (L*a*b*)
    ///
    pub fn get_manhattan_distance_lab(&self, other: &RGBColor) -> f32 {
        let reference_divisors = (LAB_REF_1, LAB_REF_2, LAB_REF_3);

        let lab_self = LABColor::from(&LabConvert::<RGBColor> {
            color: self.clone(),
            refs: reference_divisors
        });
        let lab_other = LABColor::from(&LabConvert::<RGBColor> {
            color: other.clone(),
            refs: reference_divisors
        });

        (lab_self.l - lab_other.l).abs()
        + (lab_self.a - lab_other.a).abs()
        + (lab_self.b - lab_other.b).abs()
    }
}