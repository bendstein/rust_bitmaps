use std::collections::HashMap;
use regex::Regex;

use crate::handle_bmp::{BitMapRaw, BitMapColor, BitMapRawDrawToConsoleSettings};

///
/// Command line argument key for file path.
/// 
const FILE_PATH_KEY: &str = "path";

///
/// Command line argument key for the byte representing
/// transparency.
/// 
const TRANSPARENCY_BYTE_KEY: &str = "transparency";

///
/// Command line argument key for whether to force
/// the output to not use truecolor
/// 
const FORCE_NO_TRUECOLOR_KEY: &str = "no_truecolor";

///
/// The string to use to represent the pixel in the console
/// 
const PIXEL_STRING_KEY: &str = "pixel_string";

///
/// Default string with which to represent the pixel in the console
/// 
const PIXEL_STRING_DEFAULT: &str = "█";

///
/// Command line argument key for the width of a
/// pixel in multiples of the pixel string
/// 
const PIXEL_STRING_WIDTH_KEY: &str = "pixel_width";

///
/// Default character width for each pixel
/// 
const PIXEL_STRING_WIDTH_DEFAULT: u32 = 1;

///
/// Command line argument key for the algorithm with which to
/// calculate the nearest console color when truecolor is disabled
/// 
const CONSOLE_COLOR_ALGORITHM_KEY: &str = "algorithm";

///
/// Use euclidean distance between rgb colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_RGB_EUCLIDEAN: &str = "euclidean";

///
/// Use manhattan distance between rgb colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_RGB_MANHATTAN: &str = "manhattan";

///
/// Use euclidean distance between xyz colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_XYZ_EUCLIDEAN: &str = "xyz_euclidean";

///
/// Use manhattan distance between xyz colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_XYZ_MANHATTAN: &str = "xyz_manhattan";

///
/// Use euclidean distance between l*a*b* colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN: &str = "lab_euclidean";

///
/// Use manhattan distance between L*a*b* colors to find distance
/// 
const CONSOLE_COLOR_ALGORITHM_LAB_MANHATTAN: &str = "lab_manhattan";

///
/// The default algorithm with which to calculate the nearest console
/// color when truecolor is disabled
/// 
const CONSOLE_COLOR_ALGORITHM_DEFAULT: &str = CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN;

///
/// Command line argument key to print help docs.
/// 
const HELP_KEY: &str = "help";

///
/// Environment variable for whether console supports
/// truecolor output
/// 
const TRUECOLOR_ENABLED_ENV_KEY: &str = "COLORTERM";

///
/// Valid value for {TRUECOLOR_ENABLED_ENV_KEY} indicating truecolor is enabled
/// 
const TRUECOLOR_ENABLED_VALUE_TRUECOLOR: &str = "truecolor";

///
/// Valid value for {TRUECOLOR_ENABLED_ENV_KEY} indicating truecolor is enabled
/// 
const TRUECOLOR_ENABLED_VALUE_24BIT: &str = "24bit";

///
/// Prefix for command line arguments.
/// 
const ARGUMENT_PREFIX: &str = "/";

///
/// Delimiter to split command line arguments
/// as key to value.
/// 
const ARGUMENT_DELIMITER: &str = ":";

pub mod handle_bmp;

fn main() {
    //Read in command line arguments
    let args = get_args_map();

    if !args.is_empty() {
        let blank_arg = String::from("");

        println!("Arguments:\r\n{}", match &args.keys()
            .map(|k| {
                let temp = format!(" - {}: {}", k, args.get(k).unwrap());
                temp
            })
            .reduce(|a, b| {
                let temp = format!("{a}\r\n{b}");
                temp
            }) 
            {
                Some(s) => s,
                None => &blank_arg
            }
        );
    }

    //If help flag is present, print help
    if args.contains_key(HELP_KEY) && String::from(args.get(HELP_KEY).unwrap()).eq(true.to_string().as_str()) {      
        print_help();
        return;
    }

    //Get file path from arguments
    if !args.contains_key(FILE_PATH_KEY) {
        eprintln!("Argument '{FILE_PATH_KEY}' is required!");
        println!("For help, run with {ARGUMENT_PREFIX}{HELP_KEY}");
        return;
    }
    
    let path: String = String::from(args.get(FILE_PATH_KEY).unwrap());

    //
    // Acceptable values for *TRUECOLOR_ENABLED_ENV_KEY* indicating
    // that the console supports truecolor output
    //
    let truecolor_enabled_env_values: Vec<&str> = vec![TRUECOLOR_ENABLED_VALUE_TRUECOLOR, TRUECOLOR_ENABLED_VALUE_24BIT];

    let use_truecolor: bool = match std::env::var(TRUECOLOR_ENABLED_ENV_KEY) {
        Ok(env_var) => (!args.contains_key(FORCE_NO_TRUECOLOR_KEY) || !String::from(args.get(FORCE_NO_TRUECOLOR_KEY).unwrap()).eq(true.to_string().as_str()))
            && truecolor_enabled_env_values.contains(&env_var.as_str()),
        Err(_) => false
    };

    let algorithm_name = match args.get(CONSOLE_COLOR_ALGORITHM_KEY) {
        None => CONSOLE_COLOR_ALGORITHM_DEFAULT,
        Some(value) => value.as_str()
    };

    let algorithm = match algorithm_name.to_lowercase().as_str() {
        CONSOLE_COLOR_ALGORITHM_RGB_MANHATTAN => BitMapColor::get_manhattan_distance_rgb,
        CONSOLE_COLOR_ALGORITHM_RGB_EUCLIDEAN => BitMapColor::get_euclidean_distance_rgb,
        CONSOLE_COLOR_ALGORITHM_LAB_MANHATTAN => BitMapColor::get_manhattan_distance_lab,
        CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN => BitMapColor::get_euclidean_distance_lab,
        CONSOLE_COLOR_ALGORITHM_XYZ_MANHATTAN => BitMapColor::get_manhattan_distance_xyz,
        CONSOLE_COLOR_ALGORITHM_XYZ_EUCLIDEAN => BitMapColor::get_euclidean_distance_xyz,
        _ => panic!("{algorithm_name} is not a valid distance algorithm.")
    };

    if !use_truecolor {
        println!("Truecolor is not enabled for this terminal. Will approximate distance to console colors using {algorithm_name} distance.");
    }

    let pixel_string: &str = match args.get(PIXEL_STRING_KEY) {
        None => PIXEL_STRING_DEFAULT,
        Some(value) => value.as_str()
    };

    const REGEX_VALUE_CAPTURE_NAME: &str = "VALUE";
    let u32_regex: Regex = Regex::new(r"^(?P<VALUE>\d+)$").unwrap();
    let hex_regex: Regex = Regex::new(r"^0[xX](?P<VALUE>[0-9a-fA-F]+)$").unwrap();
    let binary_regex: Regex = Regex::new(r"^0[bB](?P<VALUE>[0-1]+)$").unwrap();

    fn string_to_u32(text: &str, radix: u32, regex: &Regex) -> Option<u32> {
        if regex.is_match(text) {
            let matches: Vec<&str> = regex.captures_iter(text)
                .map(|cm| cm.name(REGEX_VALUE_CAPTURE_NAME))
                .filter(|cm| cm.is_some())
                .map(|cm| cm.unwrap().as_str())
                .collect();

            if !matches.is_empty() {
                return match u32::from_str_radix(matches[0], radix) {
                    Err(_) => None,
                    Ok(n) => Some(n)
                };
            }
            else {
                return None;
            }
        }
        None 
    }

    let pixel_width: u32 = match args.get(PIXEL_STRING_WIDTH_KEY) {
        None => PIXEL_STRING_WIDTH_DEFAULT,
        Some(value) => {
            match string_to_u32(value, 10, &u32_regex) {
                None => {
                    match string_to_u32(value, 16, &hex_regex) {
                        None => match string_to_u32(value, 2, &binary_regex) {
                            None => PIXEL_STRING_WIDTH_DEFAULT,
                            Some(x) => x
                        },
                        Some(x) => x
                    }
                },
                Some(x) => x
            }
        }
    };

    println!("Pixel representation: {pixel_string}.");
    println!("Characters per pixel: {pixel_width}.");

    let transparent_byte: Option<u32> = match args.get(TRANSPARENCY_BYTE_KEY) {
        None => None,
        Some(value) => {
            match string_to_u32(value, 10, &u32_regex) {
                None => {
                    match string_to_u32(value, 16, &hex_regex) {
                        None => string_to_u32(value, 2, &binary_regex),
                        Some(x) => Some(x)
                    }
                },
                Some(x) => Some(x)
            } 
        }
    };

    if let Some(n) = transparent_byte {
        println!("Transparency byte: {n}.");
    }
    else {
        println!("No transparency byte given.");
    }

    println!("Reading file:");

    let bitmap = match BitMapRaw::read_from_file(&path) {
        Err(msg) => {
            panic!("{msg}");
        },
        Ok(bmp) => bmp
    };
    
    println!("Successfully read file.");

    println!("Drawing to console:");

    bitmap.draw_to_console(&BitMapRawDrawToConsoleSettings::new(transparent_byte, use_truecolor, pixel_width, pixel_string, algorithm));
}

///
/// Get command line arguments
/// as a map from key to value.
/// 
fn get_args_map() -> HashMap<String, String> {
    let mut rv = HashMap::new(); 
    
    match parse_args::argparser::parse_args_with_opts(
        std::env::args(), 
        parse_args::argparser::ParseArgsSettings::init(
            String::from(ARGUMENT_PREFIX), 
            String::from(ARGUMENT_DELIMITER))
        ) {
            Err(msgs) => {
                panic!("Failed to parse arguments: {}", msgs.join(", "));
            },
            Ok(args) => args
        }
        .iter()
        .for_each(|arg| {
            let kvp = arg.to_key_value_pair();
            rv.insert(kvp.0, kvp.1);
        })
    ;

    rv
}

///
/// Print help docs
/// 
fn print_help() {
    fn flag_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}");
        temp
    }

    fn pair_example(key: &str) -> String {
        let temp = format!("{ARGUMENT_PREFIX}{key}{ARGUMENT_DELIMITER}{{VALUE}}");
        temp
    }

    let false_string = false.to_string();
    let false_string = false_string.as_str();

    let pixel_string_width_default_string = PIXEL_STRING_WIDTH_DEFAULT.to_string();
    let pixel_string_width_default_string = pixel_string_width_default_string.as_str();

    let flag_key_restriction = "If used as a key-value argument, rather than a flag argument, must be either true or false.";
    let u32_restriction = "Must be a non-negative, 32-bit integer.";

    let arg_info = vec![
        (
            HELP_KEY,
            "Display application help.".to_string(),
            flag_example(HELP_KEY),
            flag_key_restriction.to_string(),
            None
        ),
        (
            FILE_PATH_KEY,
            "The path to the bitmap.".to_string(),
            pair_example(FILE_PATH_KEY),
            "Must be a valid filepath (either relative or absolute) to a bitmap.".to_string(),
            None
        ),
        (
            TRANSPARENCY_BYTE_KEY,
            "A 32-bit, RGBA color representing transparency. Can be in decimal, binary (prefixed with 0b), or hex (prefixed with 0x).".to_string(),
            pair_example(TRANSPARENCY_BYTE_KEY),
            u32_restriction.to_string(),
            None
        ),
        (
            FORCE_NO_TRUECOLOR_KEY,
            "When set, will display bitmap using 4-bit terminal colors even if the terminal supports truecolor/24-bit color.".to_string(),
            flag_example(FORCE_NO_TRUECOLOR_KEY),
            flag_key_restriction.to_string(),
            Some(false_string)
        ),
        (
            PIXEL_STRING_KEY,
            "The string to use to represent a pixel when displaying the bitmap in the terminal.".to_string(),
            pair_example(PIXEL_STRING_KEY),
            "".to_string(),
            Some(PIXEL_STRING_DEFAULT)
        ),
        (
            PIXEL_STRING_WIDTH_KEY,
            format!("The number of times {{{ARGUMENT_PREFIX}{PIXEL_STRING_KEY}}} should be repeated to display a pixel."),
            pair_example(PIXEL_STRING_WIDTH_KEY),
            u32_restriction.to_string(),
            Some(pixel_string_width_default_string)
        ),
        (
            CONSOLE_COLOR_ALGORITHM_KEY,
            "The algorithm to use to calculate the distance between 2 colors when determining the best match between the actual color of a pixel and the 4-bit terminal color to use. Ignored if displaying bitmap in truecolor.".to_string(),
            pair_example(CONSOLE_COLOR_ALGORITHM_KEY),
            format!("[{CONSOLE_COLOR_ALGORITHM_RGB_EUCLIDEAN}, {CONSOLE_COLOR_ALGORITHM_RGB_MANHATTAN}, {CONSOLE_COLOR_ALGORITHM_XYZ_EUCLIDEAN}, {CONSOLE_COLOR_ALGORITHM_XYZ_MANHATTAN}, {CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN}, {CONSOLE_COLOR_ALGORITHM_LAB_MANHATTAN}]"),
            Some(CONSOLE_COLOR_ALGORITHM_DEFAULT)
        )
    ];

    println!("\r\nApplication arguments must be of one of the following forms:\r\n  -For a key/value: {}\r\n  -For a flag: {}\r\n\r\nThe following are acceptable arguments:", pair_example("{KEY}"), flag_example("{FLAG}"));

    arg_info.iter()
    .for_each(|info| {
        println!();
        println!("  -{}", info.0);
        println!("    -Description: {}", info.1);
        println!("    -Usage: {}", info.2);

        if !info.3.is_empty() {
            println!("    -Restrictions: {}", info.3);
        }

        if let Some(default_value) = info.4 {
            println!("    -Default Value: {}", default_value);
        }
    });

    println!();
}