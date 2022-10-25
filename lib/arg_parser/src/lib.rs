pub mod argparser;

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::env;

//     #[test]
//     fn parse_args() {
//         let args = match argparser::parse_args_with_opts(env::args(), argparser::ParseArgsSettings::init(String::from(":"), String::from("="))) {
//             Err(msgs) => {
//                 eprintln!("Failed to parse arguments: {}", msgs.join(", "));
//                 None
//             },
//             Ok(args) => Some(args)
//         };
//     }
// }