use super::super::*;

impl Clone for RGBColor {
    fn clone(&self) -> Self {
        RGBColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }
}

impl Clone for XYZColor {
    fn clone(&self) -> Self {
        XYZColor {
            x: self.x,
            y: self.y,
            z: self.z,
            alpha: self.alpha
        }
    }
}

impl Clone for LABColor {
    fn clone(&self) -> Self {
        LABColor {
            l: self.l,
            a: self.a,
            b: self.b,
            alpha: self.alpha
        }
    }
}

impl Clone for HSVColor {
    fn clone(&self) -> Self {
        HSVColor {
            hue: self.hue,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha
        }
    }
}

impl Color for RGBColor {   
    fn to_u32(&self, big_endian: bool) -> u32 {
        if big_endian {
            (self.alpha as u32) + ((self.blue as u32) << 8)  + ((self.green as u32) << 16) + ((self.red as u32) << 24)
        }
        else {
            (self.red as u32) + ((self.green as u32) << 8) + ((self.blue as u32) << 16) + ((self.alpha as u32) << 24)
        }
    }

    fn from_u32(value: u32, big_endian: bool) -> Self {
        if big_endian {
            RGBColor {
                alpha: (value & 0xFF) as u8,
                blue: ((value >> 8) & 0xFF) as u8,
                green: ((value >> 16) & 0xFF) as u8,
                red: ((value >> 24) & 0xFF) as u8,
            }
        }
        else {
            RGBColor {
                alpha: ((value >> 24) & 0xFF) as u8,
                blue: ((value >> 16) & 0xFF) as u8,
                green: ((value >> 8) & 0xFF) as u8,
                red: (value & 0xFF) as u8
            }
        }
    }
}

impl Color for XYZColor {
    fn to_u32(&self, big_endian: bool) -> u32 {
        panic!("Not implemented!");
    }

    fn from_u32(value: u32, big_endian: bool) -> Self {
        panic!("Not implemented!");
    }
}

impl Color for LABColor {
    fn to_u32(&self, big_endian: bool) -> u32 {
        panic!("Not implemented!");
    }

    fn from_u32(value: u32, big_endian: bool) -> Self {
        panic!("Not implemented!");
    }
}

impl Color for HSVColor {
    fn to_u32(&self, big_endian: bool) -> u32 {
        panic!("Not implemented!");
    }

    fn from_u32(value: u32, big_endian: bool) -> Self {
        panic!("Not implemented!");
    }
}

impl RGBColor {
    ///
    /// Create a RGBColor from the palette color at the given index
    /// of the palette table
    /// 
    pub fn from_table(palette: &BitMapPixelData, index: usize) -> Self {
        if palette.pixels.len() <= index {
            panic!("Tried to access index {} of palette, which only has {} entries!", index, palette.pixels.len());
        }

        palette.pixels[index].clone()
    }
}