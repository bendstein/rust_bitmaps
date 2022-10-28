#[allow(dead_code)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

#[allow(dead_code)]
pub struct XYZColor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: u8
}

#[allow(dead_code)]
pub struct LABColor {
    pub l: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: u8
}

#[allow(dead_code)]
pub struct HSVColor {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub alpha: u8
}
