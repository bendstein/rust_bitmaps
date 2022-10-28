use colored::{Colorize, ColoredString};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use super::super::*;

///
/// Settings for BitMapRaw::draw_to_console
/// 
#[allow(dead_code)]
pub struct BitMapRawDrawToConsoleSettings {
    ///
    /// Color representing a transparent pixel.
    /// 
    transparency: Option<u32>,
    ///
    /// Whether to use truecolor when drawing
    /// to console.
    /// 
    use_truecolor: bool,
    ///
    /// The number of repetitions of pixel_string representing one pixel
    /// 
    pixel_width: u32,
    ///
    /// The string with which to represent a pixel
    /// 
    pixel_string: String,
    ///
    /// The algorithm to use to find the nearest console color
    /// 
    algorithm: fn(&RGBColor, &RGBColor) -> f32
}

impl BitMapRawDrawToConsoleSettings {
    ///
    /// Create a new instance of BitMapRawDrawToConsoleSettings with the
    /// given settings
    /// 
    pub fn new(transparency: Option<u32>, use_truecolor: bool, pixel_width: u32, pixel_string: &str, algorithm: fn(&RGBColor, &RGBColor) -> f32) -> Self {
        BitMapRawDrawToConsoleSettings {
            transparency,
            use_truecolor,
            pixel_width,
            pixel_string: String::from(pixel_string),
            algorithm
        }
    }

    pub fn with_transparency(&mut self, transparency: Option<u32>) -> &Self {
        self.transparency = transparency;
        self
    }

    pub fn with_use_truecolor(&mut self, use_truecolor: bool) -> &Self {
        self.use_truecolor = use_truecolor;
        self
    }

    pub fn with_pixel_width(&mut self, pixel_width: u32) -> &Self {
        self.pixel_width = pixel_width;
        self
    }

    pub fn with_pixel_string(&mut self, pixel_string: &str) -> &Self {
        self.pixel_string = String::from(pixel_string);
        self
    }

    pub fn with_algorithm(&mut self, algorithm: fn(&RGBColor, &RGBColor) -> f32) -> &Self {
        self.algorithm = algorithm;
        self
    }

    pub fn clone_with_transparency(&self, transparency: Option<u32>) -> Self {
        let mut cloned = self.clone();
        cloned.with_transparency(transparency);
        cloned
    }

    pub fn clone_with_use_truecolor(&self, use_truecolor: bool) -> Self {
        let mut cloned = self.clone();
        cloned.with_use_truecolor(use_truecolor);
        cloned
    }

    pub fn clone_with_pixel_width(&self, pixel_width: u32) -> Self {
        let mut cloned = self.clone();
        cloned.with_pixel_width(pixel_width);
        cloned
    }

    pub fn clone_with_pixel_string(&self, pixel_string: &str) -> Self {
        let mut cloned = self.clone();
        cloned.with_pixel_string(pixel_string);
        cloned
    }

    pub fn clone_with_algorithm(&self, algorithm: fn(&RGBColor, &RGBColor) -> f32) -> Self {
        let mut cloned = self.clone();
        cloned.with_algorithm(algorithm);
        cloned
    }
}

impl Clone for BitMapRawDrawToConsoleSettings {
    fn clone(&self) -> Self {
        Self::new(self.transparency, self.use_truecolor, self.pixel_width, self.pixel_string.as_str(), self.algorithm)
    }
}

impl BitMapRaw {
    pub fn draw_to_console(&self, settings: &BitMapRawDrawToConsoleSettings) {
        //Write some top padding
        println!();

        let some_adjusted_transparency_bit: Option<u32>;
        let adjusted_transparency: &Option<u32>;

        //
        // If not drawing in truecolor, adjust to the closest representation of the transparency
        // color.
        //
        if !settings.use_truecolor {
            if let Some(transparency_bit) = settings.transparency {
                let transparency_bmp_color = RGBColor::from_u32(transparency_bit, true);
                let (_, adjusted_transparency_bit) = Self::color_string("", &transparency_bmp_color, settings);
                some_adjusted_transparency_bit = Some(adjusted_transparency_bit);
                adjusted_transparency = &some_adjusted_transparency_bit;
            }
            else {
                adjusted_transparency = &settings.transparency;
            }
        }
        else {
            adjusted_transparency = &settings.transparency;
        }

        if adjusted_transparency.is_some() {
            let temp_color = RGBColor::from_u32(adjusted_transparency.unwrap(), true);
            let temp_settings = settings.clone_with_transparency(None);
            let transparent_actual = Self::color_string(&settings.pixel_string, &temp_color, &temp_settings);

            println!("Shade being replaced with transparent: {}.", transparent_actual.0);
        }

        let adjusted_settings = settings.clone_with_transparency(*adjusted_transparency);

        let m = i32::abs(self.info_header.height);
        let n = i32::abs(self.info_header.width);

        //Outer loop is rows
        for j_temp in 0..m {
            //If height is negative, loop over rows in the opposite direction
            let j = match self.info_header.height {
                x if x < 0 => (m - 1) - j_temp,
                _ => j_temp
            };

            //Move to the next line
            println!();

            //Inner loop is columns
            for i_temp in 0..n {
                //If width is negative, loop over columns in the opposite direction
                let i = match self.info_header.width {
                    x if x < 0 => (n - 1) - i_temp,
                    _ => i_temp
                };

                //Calculate index of next color
                let index = (n * (m - j - 1)) + i;
                
                //Get color from index
                let pixel = &self.pixel_data.pixels[index as usize];

                //Apply ANSI coloring to the string so it is printed with color
                let (to_print, _) = Self::color_string(&adjusted_settings.pixel_string, pixel, &adjusted_settings);

                //Print the next pixel to the console
                print!("{to_print}");
            }
        }

    }

    fn color_string(value: &str, color: &RGBColor, settings: &BitMapRawDrawToConsoleSettings) -> (ColoredString, u32) {
        ///
        /// Repeat the input string the given number of times
        /// 
        fn repeat_string(input: &str, times: u32) -> String {
            let temp: Vec<&str> = (0..times).map(|_| input).collect();
            temp.concat()
        }

        //Repeat the given value {settings.pixel_width} times, storing a temp reference before converting to str
        let value_string = repeat_string(value, settings.pixel_width);
        let value_string = value_string.as_str();

        let value_string_len = value_string.graphemes(true).count();

        //Repeat a space a number of times equal to the length of the value string, storing a temp reference before converting to str
        let transparent_string = repeat_string(" ", value_string_len as u32);
        let transparent_string = transparent_string.as_str();

        if settings.use_truecolor {
            //Convert color to u32
            let color_u32 = color.to_u32(true);

            let is_transparent: bool;

            //Compare to transparent color
            if let Some(transparency_bit) = settings.transparency {
                is_transparent = transparency_bit == color_u32;
            }
            else {
                is_transparent = false;
            }

            //If transparent, print spaces with no color
            if is_transparent {
                (transparent_string.clear(), color_u32)
            }
            //Otherwise, print the given value with the given color
            else {
                (value_string.truecolor(color.red, color.green, color.blue), color_u32)
            }
        }
        //If not using truecolor, convert to a predefined console color
        else {

            //Map each console color to a function converting the text to said color
            type ColorText = fn(&str) -> ColoredString;

            let black: ColorText = |v: &str| v.black();
            let blue: ColorText = |v: &str| v.blue();
            let green: ColorText = |v: &str| v.green();
            let cyan: ColorText = |v: &str| v.cyan();
            let red: ColorText = |v: &str| v.red();
            let magenta: ColorText = |v: &str| v.magenta();
            let yellow: ColorText = |v: &str| v.yellow();
            let white: ColorText = |v: &str| v.white();
            let bright_blue: ColorText = |v: &str| v.bright_blue();
            let bright_green: ColorText = |v: &str| v.bright_green();
            let bright_cyan: ColorText = |v: &str| v.bright_cyan();
            let bright_red: ColorText = |v: &str| v.bright_red();
            let bright_magenta: ColorText = |v: &str| v.bright_magenta();
            let bright_yellow: ColorText = |v: &str| v.bright_yellow();
            let bright_black: ColorText = |v: &str| v.bright_black();
            let bright_white: ColorText = |v: &str| v.bright_white();

            let allowed_colors = HashMap::from([
                (0x00000000, black), //Black
                (0x00008000, blue), //Dark blue
                (0x00800000, green), //Dark green
                (0x00808000, cyan), //Cark cyan
                (0x80000000, red), //Dark red
                (0x80008000, magenta), //Dark magenta
                (0x80800000, yellow), //Dark yellow
                (0x80808000, white), //Dark grey
                (0x0000FF00, bright_blue), //Blue
                (0x00FF0000, bright_green), //Green
                (0x00FFFF00, bright_cyan), //Cyan
                (0xFF000000, bright_red), //Red
                (0xFF00FF00, bright_magenta), //Magenta
                (0xFFFF0000, bright_yellow), //Yellow
                (0xC0C0C000, bright_black), //Grey
                (0xFFFFFF00, bright_white) //White
            ]);

            let defaults: Vec<RGBColor> = allowed_colors.keys()
                .map(|k| RGBColor::from_u32(*k, true))
                .collect();

            //Find the closest console color to this pixel
            let nearest_color = color.get_closest_in_set(&defaults[..], settings.algorithm);

            match nearest_color {
                Some(c) => {
                    //Convert the console color to u32
                    let c_num = c.to_u32(true);

                    let is_transparent: bool;

                    //Compare the console color to the transparency color
                    if let Some(transparency_bit) = settings.transparency {
                        is_transparent = transparency_bit == c_num;
                    }
                    else {
                        is_transparent = false;
                    }

                    //If color is transparent, print spaces with no color
                    if is_transparent {
                        (transparent_string.clear(), c_num)
                    }
                    else {
                        match allowed_colors.get(&c_num) {
                            //Print the value text with the given console color
                            Some(fun) => (fun(value_string), c_num),
                            //This shouldn't happen, but if it does, print the text with default color
                            None => (value_string.normal(), c_num)
                        }
                    }

                    
                }
                //This shouldn't happen, but if it does, print the text with default color
                _ => (value_string.normal(), color.to_u32(true))
            }
        }
    }
}
