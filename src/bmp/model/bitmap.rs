use super::*;

///
/// A bitmap.
/// Bitmap format:
/// http://www.ece.ualberta.ca/~elliott/ee552/studentAppNotes/2003_w/misc/bmp_file_format/bmp_file_format.htm
/// 
#[allow(dead_code)]
pub struct BitMapRaw {
    pub header: BitMapHeader,
    pub info_header: BitMapInfoHeader,
    pub color_table: BitMapPixelData,
    pub pixel_data: BitMapPixelData
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
    pub signature: u16,
    ///
    /// The actual size of the file, including both headers, the color table,
    /// and the pixel data.
    /// 
    pub file_size: u32,
    ///
    /// ??
    /// 
    pub reserved: u32,
    ///
    /// The index at which the pixel data begins. Everything prior to this is
    /// header/color table data.
    /// 
    pub data_offset: u32
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
    pub size: u32,
    ///
    /// Horizontal width of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// vertically.
    /// 
    pub width: i32,
    ///
    /// Vertical height of bitmap, in pixels.
    /// If negative, indicates the image is mirrored
    /// horizontally.
    /// 
    pub height: i32,
    ///
    /// Number of planes (?).
    /// 
    pub planes: u16,
    ///
    /// Pixel bit depth, i.e. the number of
    /// bits required to represent a color.
    /// 
    /// 1, 4, 8: Bits contain index to a color in the color table.
    /// 16, 24, 32: Bits contain color data.
    /// 
    pub bits_per_pixel: u16,
    ///
    /// The type of compression used.
    ///     0 = BI_RGB   no compression
    ///     1 = BI_RLE8 8bit RLE encoding
    ///     2 = BI_RLE4 4bit RLE encoding
    /// 
    pub compression: u32,
    ///
    /// Compressed size of image.
    /// This can be 0 if compression == 0
    /// 
    pub image_size: u32,
    ///
    /// Horizontal resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// vertically.
    /// 
    pub x_pixels_per_meter: i32,
    ///
    /// Vertical resolution in pixels per meter
    /// If negative, indicates the image is mirrored
    /// horizontally.
    /// 
    pub y_pixels_per_meter: i32,
    ///
    /// Number of colors used in the bitmap
    /// 
    pub colors_used: u32,
    ///
    /// Number of important colors (?)
    /// 0 = all
    /// 
    pub important_colors: u32
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
    pub colors: Vec<RGBColor>
}

///
/// The actual image data in the bitmap.
/// 
#[allow(dead_code)]
pub struct BitMapPixelData {
    pub pixels: Vec<RGBColor>
}