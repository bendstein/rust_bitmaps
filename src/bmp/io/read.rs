use std::io::{self, Read, BufReader};
use std::fs::File;
use super::super::*;

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
        let mut color_table_vec: Vec<RGBColor> = Vec::new();

        //Vector of pixels in the bitmap
        let mut pixel_vec: Vec<RGBColor> = Vec::new();

        //If there is any data between the current offset and data offset, insert it into the pallette
        let color_table_length = ((header.data_offset as i32) - (offset as i32)) as usize;
        if color_table_length > 0 {
            let color_table_raw = get_next_n_bytes(&buffer, &mut offset, color_table_length);

            //Each color in the pallette is 4 bytes, the first 3 representing the Blue, Green and Red intensities respectively, with the last unused
            color_table_raw.chunks(4)
            .map(|chunk| RGBColor {
                blue: chunk[0],
                green: chunk[1],
                red: chunk[2],
                alpha: chunk[3]
            })
            .for_each(|entry| color_table_vec.push(entry));
        }
        
        let color_table: BitMapPixelData = BitMapPixelData {
            pixels: color_table_vec
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
                                let color = RGBColor::from_table(&color_table, index as usize);

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
                            let color = RGBColor {
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