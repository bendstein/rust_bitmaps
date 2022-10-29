///
/// Command line argument key for file path.
/// 
pub const FILE_PATH_KEY: &str = "path";

///
/// Command line argument key for the color representing
/// transparency.
/// 
pub const TRANSPARENCY_COLOR_KEY: &str = "transparency";

///
/// Command line argument key for the background color
/// 
pub const BACKGROUND_COLOR_KEY: &str = "background";

///
/// Command line argument key for whether to force
/// the output to not use truecolor
/// 
pub const FORCE_NO_TRUECOLOR_KEY: &str = "no_truecolor";

///
/// The string to use to represent the pixel in the console
/// 
pub const PIXEL_STRINGS_KEY: &str = "pixel_strings";

///
/// Default strings with which to represent the pixel in the console
/// 
pub const PIXEL_STRINGS_DEFAULT: &str = "██,█▓,▓▓,▓▒,▒▒,▒░,░░,░ ";

///
/// Command line argument key for the width of a
/// pixel in multiples of the pixel string
/// 
pub const PIXEL_STRING_WIDTH_KEY: &str = "pixel_width";

///
/// Default character width for each pixel
/// 
pub const PIXEL_STRING_WIDTH_DEFAULT: u32 = 1;

///
/// Command line argument key for the algorithm with which to
/// calculate the nearest console color when truecolor is disabled
/// 
pub const CONSOLE_COLOR_ALGORITHM_KEY: &str = "algorithm";

///
/// Use euclidean distance between rgb colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_RGB_EUCLIDEAN: &str = "euclidean";

///
/// Use manhattan distance between rgb colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_RGB_MANHATTAN: &str = "manhattan";

///
/// Use euclidean distance between xyz colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_XYZ_EUCLIDEAN: &str = "xyz_euclidean";

///
/// Use manhattan distance between xyz colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_XYZ_MANHATTAN: &str = "xyz_manhattan";

///
/// Use euclidean distance between l*a*b* colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN: &str = "lab_euclidean";

///
/// Use manhattan distance between L*a*b* colors to find distance
/// 
pub const CONSOLE_COLOR_ALGORITHM_LAB_MANHATTAN: &str = "lab_manhattan";

///
/// The default algorithm with which to calculate the nearest console
/// color when truecolor is disabled
/// 
pub const CONSOLE_COLOR_ALGORITHM_DEFAULT: &str = CONSOLE_COLOR_ALGORITHM_LAB_EUCLIDEAN;

///
/// Command line argument key to print help docs.
/// 
pub const HELP_KEY: &str = "help";

///
/// Environment variable for whether console supports
/// truecolor output
/// 
pub const TRUECOLOR_ENABLED_ENV_KEY: &str = "COLORTERM";

///
/// Valid value for {TRUECOLOR_ENABLED_ENV_KEY} indicating truecolor is enabled
/// 
pub const TRUECOLOR_ENABLED_VALUE_TRUECOLOR: &str = "truecolor";

///
/// Valid value for {TRUECOLOR_ENABLED_ENV_KEY} indicating truecolor is enabled
/// 
pub const TRUECOLOR_ENABLED_VALUE_24BIT: &str = "24bit";

///
/// Prefix for command line arguments.
/// 
pub const ARGUMENT_PREFIX: &str = "/";

///
/// Delimiter to split command line arguments
/// as key to value.
/// 
pub const ARGUMENT_DELIMITER: &str = ":";

///
/// Delimiter to split different opacity levels in
/// the pixels string
/// 
pub const OPACITY_LEVEL_DELIMITER: &str = ",";