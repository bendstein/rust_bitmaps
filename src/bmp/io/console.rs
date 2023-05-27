use colored::{Colorize, ColoredString};
use std::{collections::HashMap};
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
    /// Strings to use to represent different opacity levels
    /// for a pixel
    /// 
    opacity_levels: Vec<String>,
    ///
    /// Background color for the console
    /// 
    background: Option<u32>,
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
    pub fn new(transparency: Option<u32>, use_truecolor: bool, pixel_width: u32, opacity_levels: Vec<String>, background: Option<u32>, algorithm: fn(&RGBColor, &RGBColor) -> f32) -> Self {
        BitMapRawDrawToConsoleSettings {
            transparency,
            use_truecolor,
            pixel_width,
            opacity_levels,
            background,
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

    pub fn with_opacity_levels(&mut self, opacity_levels: Vec<String>) -> &Self {
        self.opacity_levels = opacity_levels;
        self
    }

    pub fn with_background(&mut self, background: Option<u32>) -> &Self {
        self.background = background;
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

    pub fn clone_with_algorithm(&self, algorithm: fn(&RGBColor, &RGBColor) -> f32) -> Self {
        let mut cloned = self.clone();
        cloned.with_algorithm(algorithm);
        cloned
    }

    pub fn clone_with_opacity_levels(&self, opacity_levels: Vec<String>) -> Self {
        let mut cloned = self.clone();
        cloned.with_opacity_levels(opacity_levels);
        cloned
    }

    pub fn clone_with_background(&self, background: Option<u32>) -> Self {
        let mut cloned = self.clone();
        cloned.with_background(background);
        cloned
    }

    ///
    /// Get the width of the longest string
    /// in opacity_levels
    /// 
    pub fn pixel_string_width(&self) -> usize {
        if self.opacity_levels.is_empty() {
            0_usize
        }
        else {
            self.opacity_levels.iter()
            .map(|o| o.graphemes(true).count())
            .reduce(usize::max)
            .unwrap_or(0)
        }
    }
}

impl Clone for BitMapRawDrawToConsoleSettings {
    fn clone(&self) -> Self {
        let cloned_opacity_levels: Vec<String> = self.opacity_levels.iter()
            .map(String::from)
            .collect();

        Self::new(self.transparency, self.use_truecolor, self.pixel_width, cloned_opacity_levels, self.background, self.algorithm)
    }
}

impl BitMapRaw {
    const TRANSPARENT_STRING: &str = " ";
    const TRANSPARENT_STRING_W_BACKGROUND: &str = ".";

    pub fn draw_to_console(&self, settings: &BitMapRawDrawToConsoleSettings) {
        let _ = colored::control::set_virtual_terminal(true);

        //Write some top padding
        println!();

        let some_adjusted_transparency_bit: Option<u32>;
        let adjusted_transparency: &Option<u32>;

        let some_adjusted_background_bit: Option<u32>;
        let adjusted_background: &Option<u32>;

        //
        // If not drawing in truecolor, adjust to the closest representation of the transparency/background
        // color.
        //
        if !settings.use_truecolor {
            if let Some(transparency_bit) = settings.transparency {
                let transparency_bmp_color = RGBColor::from_u32(transparency_bit | 0xFF, true);
                let temp_settings = settings.clone_with_transparency(None);
                let (_, adjusted_transparency_bit, _) = Self::get_color_type(&transparency_bmp_color, &temp_settings);

                some_adjusted_transparency_bit = Some(adjusted_transparency_bit);
                adjusted_transparency = &some_adjusted_transparency_bit;
            }
            else {
                adjusted_transparency = &settings.transparency;
            }

            if let Some(background_bit) = settings.background {
                let background_bmp_color = RGBColor::from_u32(background_bit | 0xFF, true);
                let temp_settings = settings.clone_with_transparency(None);
                let (_, adjusted_background_bit, _) = Self::get_color_type(&background_bmp_color, &temp_settings);
                
                some_adjusted_background_bit = Some(adjusted_background_bit);
                adjusted_background = &some_adjusted_background_bit;
            }
            else {
                adjusted_background = &settings.background;
            }
        }
        else {
            adjusted_transparency = &settings.transparency;
            adjusted_background = &settings.background;
        }

        if adjusted_transparency.is_some() {
            let temp_color = RGBColor::from_u32(adjusted_transparency.unwrap(), true);
            let (color_type, _, _) = Self::get_color_type(&temp_color, settings);
            
            //Get pixels string to use from opacity
            let pixel_string_ndx: Option<usize> = Self::get_pixel_from_opacity(&temp_color, settings);

            let width = u32::min(usize::MAX as u32, (settings.pixel_string_width() as u32) * settings.pixel_width) as usize;

            let transparent_string = Self::repeat_string(Self::TRANSPARENT_STRING, width);

            let pixel_string = match pixel_string_ndx {
                None => transparent_string,
                Some(n) => Self::repeat_string(settings.opacity_levels[n].as_str(), width)
            };

            let mut coloring = ColoredString::from(pixel_string.as_str());

            if let Some(fg) = color_type {
                coloring = coloring.color(fg);
            }

            println!("Transparent Color: {}.", coloring);
        }

        if adjusted_background.is_some() {
            let temp_color = RGBColor::from_u32(adjusted_background.unwrap(), true);
            let (background_color_type, _, _) = Self::get_color_type(&temp_color, settings);
            
            //Get pixels string to use from opacity
            let pixel_string_ndx: Option<usize> = Self::get_pixel_from_opacity(&temp_color, settings);

            let width = u32::min(usize::MAX as u32, (settings.pixel_string_width() as u32) * settings.pixel_width) as usize;

            let transparent_string = Self::repeat_string(Self::TRANSPARENT_STRING, width);

            let pixel_string = match pixel_string_ndx {
                None => transparent_string,
                Some(n) => Self::repeat_string(settings.opacity_levels[n].as_str(), width)
            };

            let mut coloring = ColoredString::from(pixel_string.as_str());

            if let Some(bg) = background_color_type {
                coloring = coloring.color(bg);
            }

            println!("Background Color: {}.", coloring);
        }

        let adjusted_settings = settings.clone_with_transparency(*adjusted_transparency);
        let adjusted_settings = adjusted_settings.clone_with_background(*adjusted_background);

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
                let pixel = &self.pixel_data.pixels[index as usize].clone();

                //Get background color
                let background_color = &adjusted_background
                    .map(|n| {
                        let mut temp = RGBColor::from_u32(n, true);
                        temp.alpha = 0xFF;
                        temp
                    });
                

                //Get pixels string to use from opacity
                let pixel_string_ndx: Option<usize> = Self::get_pixel_from_opacity(pixel, &adjusted_settings);

                let width = u32::min(usize::MAX as u32, (settings.pixel_string_width() as u32) * settings.pixel_width) as usize;

                let transparent_string = Self::repeat_string(Self::TRANSPARENT_STRING, width);
                let transparent_string_w_background = Self::repeat_string(Self::TRANSPARENT_STRING_W_BACKGROUND, width);

                let (pixel_string, color) = match pixel_string_ndx {
                    None if background_color.is_some() => (transparent_string_w_background, background_color.as_ref().unwrap()),
                    None => (transparent_string, pixel),
                    Some(n) =>(Self::repeat_string(settings.opacity_levels[n].as_str(), width), pixel)
                };

                //Apply ANSI coloring to the string so it is printed with color
                let (to_print, _) = Self::color_string(pixel_string.as_str(), color, &adjusted_settings);

                //Print the next pixel to the console
                print!("{to_print}");
            }
        }

    }

    fn get_pixel_from_opacity(color: &RGBColor, settings: &BitMapRawDrawToConsoleSettings) -> Option<usize> {
        if settings.opacity_levels.is_empty(){
            return None;
        }
        
        if color.alpha == 0 {
            return None;
        }

        let alpha_ratio = (color.alpha as f32) / 255_f32;

        let len = settings.opacity_levels.len() as f32;

        for index in 1..=settings.opacity_levels.len() {
            let lower_bound = (len - (index as f32)) / len;
            let upper_bound = (len - (index as f32) + 1_f32) / len;

            if alpha_ratio > lower_bound && alpha_ratio <= upper_bound {
                let return_index = f32::max(0_f32, (index as f32) - 1_f32) as usize;

                return Some(return_index);
            }
        }

        Some(settings.opacity_levels.len())    
    }

    fn color_string(value: &str, color: &RGBColor, settings: &BitMapRawDrawToConsoleSettings) -> (ColoredString, u32) {

        //Get the widest string in settings.opacity_levels
        let width = u32::min(usize::MAX as u32, (settings.pixel_string_width() as u32) * settings.pixel_width) as usize;

        //Repeat the given value {settings.pixel_width} times, storing a temp reference before converting to str
        let value_string = Self::repeat_string(value, width);
        let value_string = value_string.as_str();

        //Repeat a space a number of times equal to the length of the value string, storing a temp reference before converting to str
        let transparent_string = Self::repeat_string(Self::TRANSPARENT_STRING, width);
        let transparent_string = transparent_string.as_str();

        let (color_type, adj_color, is_transparent) = Self::get_color_type(color, settings);

        let is_transparent = is_transparent || color.alpha == 0;

        let background_color_type: Option<colored::Color>;

        if let Some(background_color_num) = settings.background {
            let background_color = RGBColor::from_u32(background_color_num, true);
            (background_color_type, _, _) = Self::get_color_type(&background_color, settings);
        }
        else {
            background_color_type = None;
        }

        let to_color = match is_transparent {
            false => value_string,
            true => transparent_string
        };

        let mut coloring = ColoredString::from(to_color);

        if let Some(fg) = color_type {
            coloring = coloring.color(fg);
        }

        if let Some(bg) = background_color_type {
            coloring = coloring.on_color(bg);
        }

        (coloring, adj_color)
    }

    ///
    /// Get a pointer to the function to set the fore/background color of the pixel
    /// 
    fn get_color_type(color: &RGBColor, settings: &BitMapRawDrawToConsoleSettings) -> (Option<colored::Color>, u32, bool) {
        let allowed_colors = HashMap::from([
            (0x00000000, colored::Color::Black), //Black
            (0x00008000, colored::Color::Blue), //Dark blue
            (0x00800000, colored::Color::Green), //Dark green
            (0x00808000, colored::Color::Cyan), //Cark cyan
            (0x80000000, colored::Color::Red), //Dark red
            (0x80008000, colored::Color::Magenta), //Dark magenta
            (0x80800000, colored::Color::Yellow), //Dark yellow
            (0x80808000, colored::Color::White), //Dark grey
            (0x0000FF00, colored::Color::BrightBlue), //Blue
            (0x00FF0000, colored::Color::BrightGreen), //Green
            (0x00FFFF00, colored::Color::BrightCyan), //Cyan
            (0xFF000000, colored::Color::BrightRed), //Red
            (0xFF00FF00, colored::Color::BrightMagenta), //Magenta
            (0xFFFF0000, colored::Color::BrightYellow), //Yellow
            (0xC0C0C000, colored::Color::BrightBlack), //Grey
            (0xFFFFFF00, colored::Color::BrightWhite) //White
        ]);

        //Convert color to u32
        let color_u32 = color.to_u32(true);

        if settings.use_truecolor {
            let is_transparent: bool;

            //Compare to transparent color
            if let Some(transparency_bit) = settings.transparency {
                is_transparent = transparency_bit == color_u32;
            }
            else {
                is_transparent = color.alpha == 0;
            }

            (Some(colored::Color::TrueColor { r: color.red, g: color.green, b: color.blue }), color_u32, is_transparent)
        }
        else {
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
                        is_transparent = color.alpha == 0;
                    }

                    match allowed_colors.get(&c_num) {
                        Some(fun) => {
                            (Some(*fun), c_num, is_transparent)
                        }
                        //This shouldn't happen
                        None => (None, c_num, is_transparent)
                    }                   
                }
                //This shouldn't happen
                _ => (None, color_u32, color.alpha == 0)
            }
        }
    }

    ///
    /// Repeat the input string the given number of times
    /// 
    fn repeat_string(input: &str, times: usize) -> String {
        if times == 0_usize {
            return String::from("");
        }

        let temp: Vec<&str> = (0..times).map(|_| input).collect();
        let combined = temp.concat();

        let width = combined.graphemes(true).count();
        
        if width > times {
            let graphemes = combined.graphemes(true).collect::<Vec<&str>>();
            let truncated_graphemes = &graphemes[0..times];
            let char_len = truncated_graphemes.iter()
                .map(|g| g.len())
                .reduce(|a, b| a + b)
                .unwrap_or(0);
                
            String::from(&combined[0..char_len])
        }
        else {
            combined
        }
    }
}
