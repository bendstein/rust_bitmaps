use std::io::{self, Read, BufReader};
use std::fs::File;
use colored::{Colorize, ColoredString};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

///
/// A bitmap.
/// Bitmap format:
/// http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm
/// 
#[allow(dead_code)]
pub struct BitMapRaw {
    header: BitMapHeader,
    info_header: BitMapInfoHeader,
    color_table: BitMapColorTable,
    pixel_data: BitMapPixelData
}

///
/// Bitmap header data, regarding
/// the size of the bitmap and location
/// of the pixel data
/// 
#[allow(dead_code)]
pub struct BitMapHeader {
    ///
    /// Bitmap signature. Should always be BM.
    /// 
    signature: u16,
    ///
    /// The actual size of the file, including both headers, the color table,
    /// and the pixel data.
    /// 
    file_size: u32,
    ///
    /// ??
    /// 
    reserved: u32,
    ///
    /// The index at which the pixel data begins. Everything prior to this is
    /// header/color table data.
    /// 
    data_offset: u32
}

///
/// Bitmap info header data, regarding
/// layout/contents of the bitmap.
/// 
#[allow(dead_code)]
pub struct BitMapInfoHeader {
    ///
    /// Size of this info header.
    /// 
    size: u32,
    ///
    /// Horizontal width of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// vertically.
    /// 
    width: i32,
    ///
    /// Vertical height of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// horizontally.
    /// 
    height: i32,
    ///
    /// Number of planes (?).
    /// 
    planes: u16,
    ///
    /// Pixel bit depth, i.e. the number of
    /// bits required to represent a color.
    /// 
    /// 1, 4, 8: Bits contain index to a color in the color table.
    /// 16, 24, 32: Bits contain color data.
    /// 
    bits_per_pixel: u16,
    ///
    /// The type of compression used.
    ///     0 = BI_RGB   no compression
    ///     1 = BI_RLE8 8bit RLE encoding
    ///     2 = BI_RLE4 4bit RLE encoding
    /// 
    compression: u32,
    ///
    /// Compressed size of image.
    /// This can be 0 if compression == 0
    /// 
    image_size: u32,
    ///
    /// Horizontal resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// vertically.
    /// 
    x_pixels_per_meter: i32,
    ///
    /// Vertical resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// horizontally.
    /// 
    y_pixels_per_meter: i32,
    ///
    /// Number of colors used in the bitmap
    /// 
    colors_used: u32,
    ///
    /// Number of important colors (?)
    /// 0 = all
    /// 
    important_colors: u32
}

///
/// Bitmap color definitions.
/// Ordered Red-Green-Blue-Reserved,
/// each 1 byte in size.
/// Present only if bit depth is less than 8.
/// Colors ordered by importance.
/// 
#[allow(dead_code)]
pub struct BitMapColorTable {
    colors: Vec<BitMapColor>
}

///
/// The actual image data in the bitmap.
/// 
#[allow(dead_code)]
pub struct BitMapPixelData {
    pixels: Vec<BitMapColor>
}

///
/// RGB color for a pixel in the bitmap data
/// 
#[allow(dead_code)]
pub struct BitMapColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

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
    algorithm: fn(&BitMapColor, &BitMapColor) -> f32
}

impl BitMapRawDrawToConsoleSettings {
    ///
    /// Create a new instance of BitMapRawDrawToConsoleSettings with the
    /// given settings
    /// 
    pub fn new(transparency: Option<u32>, use_truecolor: bool, pixel_width: u32, pixel_string: &str, algorithm: fn(&BitMapColor, &BitMapColor) -> f32) -> Self {
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

    pub fn with_algorithm(&mut self, algorithm: fn(&BitMapColor, &BitMapColor) -> f32) -> &Self {
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

    pub fn clone_with_algorithm(&self, algorithm: fn(&BitMapColor, &BitMapColor) -> f32) -> Self {
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

impl Clone for BitMapColor {
    fn clone(&self) -> Self {
        BitMapColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha
        }
    }
}

impl BitMapColor {
    ///
    /// Convert the BitMapColor to a u32,
    /// in either big or little endian.
    /// 
    pub fn to_u32(&self, big_endian: bool) -> u32 {
        if big_endian {
            (self.alpha as u32) + ((self.blue as u32) << 8)  + ((self.green as u32) << 16) + ((self.red as u32) << 24)
        }
        else {
            (self.red as u32) + ((self.green as u32) << 8) + ((self.blue as u32) << 16) + ((self.alpha as u32) << 24)
        }
    }

    ///
    /// Create a BitMapColor from the given u32,
    /// in either big or little endian.
    /// 
    pub fn from_u32(value: u32, big_endian: bool) -> Self {
        if big_endian {
            BitMapColor {
                alpha: (value & 0xFF) as u8,
                blue: ((value >> 8) & 0xFF) as u8,
                green: ((value >> 16) & 0xFF) as u8,
                red: ((value >> 24) & 0xFF) as u8,
            }
        }
        else {
            BitMapColor {
                alpha: ((value >> 24) & 0xFF) as u8,
                blue: ((value >> 16) & 0xFF) as u8,
                green: ((value >> 8) & 0xFF) as u8,
                red: (value & 0xFF) as u8
            }
        }
    }

    ///
    /// Create a BitMapColor from the palette color at the given index
    /// of the palette table
    /// 
    pub fn from_table(palette: &BitMapColorTable, index: usize) -> Self {
        if palette.colors.len() <= index {
            panic!("Tried to access index {} of palette, which only has {} entries!", index, palette.colors.len());
        }

        palette.colors[index].clone()
    }

    ///
    /// Calculate the euclidean distance between self to other (rgb)
    ///
    pub fn get_euclidean_distance_rgb(&self, other: &BitMapColor) -> f32 {
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
    pub fn get_manhattan_distance_rgb(&self, other: &BitMapColor) -> f32 {
        let rgba_self = (self.red as f32, self.green as f32, self.blue as f32, self.alpha as f32);
        let rgba_other = (other.red as f32, other.green as f32, other.blue as f32, self.alpha as f32);

        (rgba_self.0 - rgba_other.0).abs()
        + (rgba_self.1 - rgba_other.1).abs()
        + (rgba_self.2 - rgba_other.2).abs()
        //+ (rgba_self.3 - rgba_other.3).abs()
    }

    ///
    /// Convert RGB color to XYZ
    /// See: http://www.easyrgb.com/en/math.php
    /// 
    fn get_xyz(&self) -> (f32, f32, f32) {
        fn adj(channel: f32) -> f32 {
            let scaled = channel / 255_f32;
            if scaled > 0.04045 {
                ((scaled + 0.055) / 1.055).powf(2.4)
            }
            else {
                scaled / 12.92
            }
        }

        let var_r: f32 = adj(self.red as f32);
        let var_g: f32 = adj(self.green as f32);
        let var_b: f32 = adj(self.blue as f32);

        (
            var_r * 0.4124 + var_g * 0.3576 + var_b * 0.1805,
            var_r * 0.2126 + var_g * 0.7152 + var_b * 0.0722,
            var_r * 0.0193 + var_g * 0.1192 + var_b * 0.9505
        )
    }

    ///
    /// Calculate the euclidean distance between self to other (xyz)
    ///
    pub fn get_euclidean_distance_xyz(&self, other: &BitMapColor) -> f32 {
        let xyz_self = self.get_xyz();
        let xyz_other = other.get_xyz();

        f32::sqrt(
            (xyz_self.0 - xyz_other.0).powi(2)
            + (xyz_self.1 - xyz_other.1).powi(2)
            + (xyz_self.2 - xyz_other.2).powi(2)
        )
    }

    ///
    /// Calculate the manhattan distance between self to other (xyz)
    ///
    pub fn get_manhattan_distance_xyz(&self, other: &BitMapColor) -> f32 {
        let xyz_self = self.get_xyz();
        let xyz_other = other.get_xyz();

        (xyz_self.0 - xyz_other.0).abs()
        + (xyz_self.1 - xyz_other.1).abs()
        + (xyz_self.2 - xyz_other.2).abs()
    }

    ///
    /// Convert RGB color to CIEL*A*B*
    /// See: http://www.easyrgb.com/en/math.php
    /// 
    fn get_lab(&self, reference_divisors: (f32, f32, f32)) -> (f32, f32, f32) {
        fn adj(channel: f32) -> f32 {
            if channel > 0.008856 {
                channel.powf(1_f32 / 3_f32)
            }
            else {
                (7.787 * channel) + (16_f32 / 116_f32)
            }
        }
      
        //Convert RGB color to XYZ
        let (x, y, z) = self.get_xyz();

        let var_x = adj(x / reference_divisors.0);
        let var_y = adj(y / reference_divisors.1);
        let var_z = adj(z / reference_divisors.2);

        (
            (116_f32 * var_y) - 16_f32,
            500_f32 * (var_x - var_y),
            200_f32 * (var_y - var_z)
        )
    }

    const LAB_REF_1: f32 = 1_f32;
    const LAB_REF_2: f32 = 1_f32;
    const LAB_REF_3: f32 = 1_f32;

    ///
    /// Calculate the euclidean distance between self to other (L*a*b*)
    ///
    pub fn get_euclidean_distance_lab(&self, other: &BitMapColor) -> f32 {
        let reference_divisors = (Self::LAB_REF_1, Self::LAB_REF_2, Self::LAB_REF_3);
        let lab_self = self.get_lab(reference_divisors);
        let lab_other = other.get_lab(reference_divisors);

        f32::sqrt(
            (lab_self.0 - lab_other.0).powi(2)
            + (lab_self.1 - lab_other.1).powi(2)
            + (lab_self.2 - lab_other.2).powi(2)
        )
    }

    ///
    /// Calculate the manhattan distance between self to other (L*a*b*)
    ///
    pub fn get_manhattan_distance_lab(&self, other: &BitMapColor) -> f32 {
        let reference_divisors = (Self::LAB_REF_1, Self::LAB_REF_2, Self::LAB_REF_3);
        let lab_self = self.get_lab(reference_divisors);
        let lab_other = other.get_lab(reference_divisors);

        (lab_self.0 - lab_other.0).abs()
        + (lab_self.1 - lab_other.1).abs()
        + (lab_self.2 - lab_other.2).abs()
    }

    ///
    /// Calculate the distance between self, and each color in the given set using the given algorithm, and return
    /// the closest color in the set, or none, if the given set is empty.
    /// 
    pub fn get_closest_in_set<'a, 'b>(&'a self, to_compare: &'b [BitMapColor], algorithm: fn(&BitMapColor, &BitMapColor) -> f32) -> Option<&'b BitMapColor> {
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

impl BitMapRaw {
    pub fn read_from_file(path: &str) -> Result<Self, io::Error> {
        //Open the file
        let fs = File::open(path)?;

        // //Get file metadata
        // let file_metadata = File::metadata(&fs)?;

        //Read file to buffer
        let mut br = BufReader::new(fs);
        let mut buffer = Vec::new();

        br.read_to_end(&mut buffer)?;

        let mut offset: usize = 0x0;
        fn get_next_n_bytes<'a, 'b>(buffer: &'a [u8], offset: &'b mut usize, n: usize) -> &'a [u8] {
            let o_offset = *offset;
            *offset += n;
            &buffer[o_offset..*offset]
        }

        let header = BitMapHeader {
            signature: Self::reduce_bit_slice_u16(get_next_n_bytes(&buffer, &mut offset, 2)),
            file_size: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            reserved: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            data_offset: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4))
        };

        let info_header = BitMapInfoHeader {
            size: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            width: Self::reduce_bit_slice_i32(get_next_n_bytes(&buffer, &mut offset, 4)),
            height: Self::reduce_bit_slice_i32(get_next_n_bytes(&buffer, &mut offset, 4)),
            planes: Self::reduce_bit_slice_u16(get_next_n_bytes(&buffer, &mut offset, 2)),
            bits_per_pixel: Self::reduce_bit_slice_u16(get_next_n_bytes(&buffer, &mut offset, 2)),
            compression: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            image_size: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            x_pixels_per_meter: Self::reduce_bit_slice_i32(get_next_n_bytes(&buffer, &mut offset, 4)),
            y_pixels_per_meter: Self::reduce_bit_slice_i32(get_next_n_bytes(&buffer, &mut offset, 4)),
            colors_used: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4)),
            important_colors: Self::reduce_bit_slice_u32(get_next_n_bytes(&buffer, &mut offset, 4))
        };

        //Vector of pixels in the palette
        let mut color_table_vec: Vec<BitMapColor> = Vec::new();

        //Vector of pixels in the bitmap
        let mut pixel_vec: Vec<BitMapColor> = Vec::new();

        //If there is any data between the current offset and data offset, insert it into the pallette
        let color_table_length = ((header.data_offset as i32) - (offset as i32)) as usize;
        if color_table_length > 0 {
            let color_table_raw = get_next_n_bytes(&buffer, &mut offset, color_table_length);

            //Each color in the pallette is 4 bytes, the first 3 representing the Blue, Green and Red intensities respectively, with the last unused
            color_table_raw.chunks(4)
            .map(|chunk| BitMapColor {
                blue: chunk[0],
                green: chunk[1],
                red: chunk[2],
                alpha: chunk[3]
            })
            .for_each(|entry| color_table_vec.push(entry));
        }
        
        let color_table: BitMapColorTable = BitMapColorTable {
            colors: color_table_vec
        };

        ///
        /// Round the value up to the nearest multiple of 4
        /// See: https://stackoverflow.com/a/9194117
        /// 
        fn round_to_next_multiple_of_4(value: i32) -> usize {
            ((value + 4 - 1) & -4) as usize
        }
        
        //bpp = 1, 4 or 8: value of each pixel has a size <= 1 byte, and is an index of the color table
        if [1, 4, 8].contains(&info_header.bits_per_pixel) {
            //Get the width of the scanline based on bit depth and line width
            let pixels_per_bit = f32::ceil(8_f32 / (info_header.bits_per_pixel as f32)) as usize;
            let scaline_width_temp = f32::ceil(f32::abs(info_header.width as f32) / (pixels_per_bit as f32)) as i32;
            let scanline_width = round_to_next_multiple_of_4(scaline_width_temp);

            //Read in each scanline
            loop {
                let mut done: bool = false;
    
                let mut count = scanline_width;
    
                //
                // I don't think this should ever happen for a properly-formatted
                // bitmap, but if the scanline goes past the end of the file,
                // truncate it
                //
                if buffer.len() < offset + scanline_width {
                    count = ((buffer.len() as i32) - (offset as i32)) as usize;
                    done = true;
                }

                //Get the scanline data
                let scanline = get_next_n_bytes(&buffer, &mut offset, count as usize);
    
                //
                // Loop over each bit in the scanline, ignoring 0-padding at the end of the scanline.
                //
                scanline.iter()
                    .enumerate()
                    .for_each(|(ndx, chunk)| {
                        if ndx < (scaline_width_temp as usize) {
                            //For each pixel in the bit
                            for i in 1..=pixels_per_bit {
                                //If past the width of the line, the rest of the bits are padding
                                if (pixels_per_bit * ndx) + i > (info_header.width as usize) {
                                    break;
                                }

                                //Extract the (i - 1)th pixel from the byte
                                let index = (*chunk >> (8 - ((info_header.bits_per_pixel as i32) * (i as i32)))) & ((2_u16.pow(info_header.bits_per_pixel as u32) - 1) as u8);
                                
                                //Extract the color from the color table and add it to the pixel data
                                let color = BitMapColor::from_table(&color_table, index as usize);

                                pixel_vec.push(color);
                            }
                        }
                    });
    
                if done {
                    break;
                }
            };
        }       
        //bpp = 16: value of each pixel is 2 bytes, with each 5 bits representing Blue, Green and Red intensities respectively, and the last bit being unused.
        else if info_header.bits_per_pixel == 16 {
            panic!("Not implemented for 16 bit images!");
        }
        //bpp = 24: value of each pixel is 3 bytes, representing Blue, Green and Red intensities respectively
        //bpp = 32: value of each pixel is 4 bytes, representing Alpha, Blue, Green and Red intensities respectively
        else if [24, 32].contains(&info_header.bits_per_pixel) {
            //Get scanline width based on line width
            let bytesperpixel = f32::ceil((info_header.bits_per_pixel as f32) / 8_f32) as usize;
            let scaline_width_temp = i32::abs(info_header.width * (bytesperpixel as i32));
            let scanline_width = round_to_next_multiple_of_4(scaline_width_temp);
    
            //Read in each scanline
            loop {
                let mut done: bool = false;
    
                let mut count = scanline_width;
    
                //
                // I don't think this should ever happen for a properly-formatted
                // bitmap, but if the scanline goes past the end of the file,
                // truncate it
                //
                if buffer.len() < offset + scanline_width {
                    count = ((buffer.len() as i32) - (offset as i32)) as usize;
                    done = true;
                }
    
                //Get the scanline data
                let scanline = get_next_n_bytes(&buffer, &mut offset, count as usize);
    
                //
                // Loop over each chunk of 3 bytes in the scanline, ignoring 0-padding at the end of the scanline.
                //
                scanline
                    .chunks(bytesperpixel)
                    .for_each(|chunk| {
                        //Ignore 0-padding
                        if chunk.len() == bytesperpixel {
                            //Extract alpha, blue, green, and red from their respective bytes
                            let color = BitMapColor {
                                alpha: match bytesperpixel {
                                    32 => chunk[0],
                                    _ => 0
                                },
                                blue: match bytesperpixel {
                                    32 => chunk[1],
                                    _ => chunk[0]
                                },
                                green: match bytesperpixel {
                                    32 => chunk[2],
                                    _ => chunk[1]
                                },
                                red: match bytesperpixel {
                                    32 => chunk[3],
                                    _ => chunk[2]
                                },
                            };

                            pixel_vec.push(color);
                        }
                    });
    
                if done {
                    break;
                }
            };
        }
        //bpp cannot be anything but 1, 4, 8, 16, 24, or 32
        else {
            panic!("{} is not a valid value for 'bits_per_pixel'.", info_header.bits_per_pixel);
        }

        let pixel_data = BitMapPixelData {
            pixels: pixel_vec
        };

        Ok(Self {
            header,
            info_header,
            color_table,
            pixel_data
        })
    }

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
                let transparency_bmp_color = BitMapColor::from_u32(transparency_bit, true);
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
            let temp_color = BitMapColor::from_u32(adjusted_transparency.unwrap(), true);
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

    fn color_string(value: &str, color: &BitMapColor, settings: &BitMapRawDrawToConsoleSettings) -> (ColoredString, u32) {

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

            let defaults: Vec<BitMapColor> = allowed_colors.keys()
                .map(|k| BitMapColor::from_u32(*k, true))
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

    fn reduce_bit_slice_u16(slice: &[u8]) -> u16 {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| u16::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn reduce_bit_slice_u32(slice: &[u8]) -> u32 {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| u32::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn reduce_bit_slice_i32(slice: &[u8]) -> i32 {
        slice.iter()
            .enumerate()
            .map(|(index, byte)| i32::from(*byte) << (8 * index))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}