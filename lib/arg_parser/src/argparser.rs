use std::{env, collections::HashSet};

///
///An enum representing a command-line
///argument
///
pub enum Arg {
    ///
    ///An argument whose presence
    ///implies a value of true, and whose
    ///absence implies a value of false.
    ///
    Flag(String),
    ///
    ///An argument made from a key
    ///and a value
    ///
    Pair(String, String)
}

impl Arg {
    pub fn to_key_value_pair(&self) -> (String, String) {
        match self {
            Arg::Flag(k) => (k.to_string(), true.to_string()),
            Arg::Pair(k, v) => (k.to_string(), v.to_string())
        }
    }
}

///
///Settings to modify the execution of parse_args
///
pub struct ParseArgsSettings {
    ///
    ///The prefix applied to arguments
    ///
    prefix: String,
    ///
    ///The value separating keys and values
    ///of arguments
    ///
    delimiter: String
}

impl Default for ParseArgsSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseArgsSettings {
    ///
    ///Create a new instance of ParseArgsSettings with default prefix
    ///and delimiter
    ///
    pub fn new() -> Self {
        Self {
            prefix: String::from(""),
            delimiter: String::from("=")
        }
    }

    ///
    ///Create a new instance of ParseArgsSettings with given prefix
    ///and delimiter
    ///
    pub fn init(prefix: String, delimiter: String) -> Self {
        Self {
            prefix,
            delimiter
        }
    }

    ///
    ///Set prefix on given instance of ParseArgsSettings and return
    ///self
    ///
    pub fn with_prefix(&mut self, prefix: String) -> &Self {
        self.prefix = prefix;
        self
    }

    ///
    ///Set delimiter on given instance of ParseArgsSettings and return
    ///self
    ///
    pub fn with_delimiter(&mut self, delimiter: String) -> &Self {
        self.delimiter = delimiter;
        self
    }

}

impl Clone for ParseArgsSettings {
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.to_string(),
            delimiter: self.delimiter.to_string()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.prefix = source.prefix.to_string();
        self.delimiter = source.delimiter.to_string();
    }
}

///
///Create a vector of Arg from a collection
///of command line arguments.
///
pub fn parse_args_with_opts(args: env::Args, settings: ParseArgsSettings) -> Result<Vec<Arg>, Vec<String>> {
    let mut contains_errors: bool = false;

    //Try to parse each argument into an Arg
    let parsed_or_errs: Vec<Result<Arg, &str>> = args.map(|a| {
        //Split each argument on delimiter (default '=') from settings
        let split: Vec<&str> = a.split(settings.delimiter.as_str()).collect();

        //If argument is empty, return None
        if split.is_empty() {
            Ok(None)
        }
        else {
            /*
             * The argument should be of the form {key[=value]},
             * so the first element of split will be the key. Ignore
             * start/end whitespace
             */
            let mut key: &str = split[0].trim();

            //If prefix from settings isn't empty, check if key begins with prefix from settings
            if!settings.prefix.is_empty() {
                //If it does, remove it, as it is not part of the key
                if key.starts_with(settings.prefix.as_str()) {
                    key = key[settings.prefix.len()..].trim();
                }
                //If it doesn't, return None
                else {
                    return Ok(None);
                }
            }

            //Make sure that the key has content.
            if key.is_empty() {
                Err("Argument key cannot be empty!")
            }
            else {
                /*
                * If there are no other elements, the argument is of the form {key},
                * meaning it is a flag.
                */
                if split.len() == 1 {
                    Ok(Some(Arg::Flag(String::from(key))))
                }
                /*
                * Otherwise, the argument is of the form {key=value}. Aggregate all
                * other elements for the case where there are multiple instances of
                * '=' in the argument, in which case, the split between key and value
                * will be on the first instance.
                */
                else {
                    Ok(Some(Arg::Pair(String::from(key), String::from(split[1..].join(settings.delimiter.as_str()).trim()))))
                }
            }   
        }
    })
    //Remove empty arguments and unwrap the remaining
    .filter(|a| match a {
        Err(_) => {
            //An error was detected, flip contains_errors to true
            contains_errors = true;
            true
        },
        Ok(opt) => opt.is_some()
    })
    .map(|a| {
        match a {
            Err(e) => Err(e),
            Ok(opt) => Ok(opt.unwrap())
        }
    })
    .collect(); 

    //If any of the elements of parsed_or_errs are Errs, return an Err containing all Errs in parsed
    if contains_errors {
        let mut errors: Vec<String> = Vec::new();

        for arg in parsed_or_errs {
            if arg.is_err() {
                errors.push(String::from(arg.err().unwrap()));
            }
        }

        return Err(errors);
    }

    //If this point is reached, all elements in parsed are Ok
    let mut parsed: Vec<Arg> = Vec::new();

    for arg in parsed_or_errs {
        parsed.push(arg.unwrap());
    }

    //Get a vector of indices with duplicate argument keys
    let find_dupes = |args: &Vec<Arg>| {
        //Collection of keys already recorded
        let mut seen: HashSet<&String> = HashSet::new();

        //Collection of indices which have a duplicate key
        let mut dupes: Vec<usize> = Vec::new();

        //Check each argument sequentially for duplicates
        for (i, arg) in args.iter().enumerate() {
            //Get the key from the argument
            let key = match arg {
                Arg::Flag(key) => key,
                Arg::Pair(key, _) => key
            };

            //If the key has been previously seen, mark the index as a duplicate
            if seen.contains(key) {
                dupes.push(i);
            }
            //Otherwise, record the key so any future appearances will be marked as duplicates
            else {
                seen.insert(key);
            }
        }

        dupes
    };

    let dupes = find_dupes(&parsed);

    //If there are any duplicates, return an error
    if !dupes.is_empty() {

        let dupe_args: Vec<&str> = parsed.iter()
        .enumerate()
        .filter(|(i, _)| dupes.contains(i))
        .map(|(_, arg)| match arg {
            Arg::Flag(key) => key.as_str(),
            Arg::Pair(key, _) => key.as_str()
        }).collect();

        return Err(vec![format!("Cannot have duplicate argument keys! ({})", dupe_args.join(", "))]);
    }

    Ok(parsed)
}

///
/// Calls 
/// ```
/// parse_args_with_opts
/// ```
/// with default settings.
/// 
pub fn parse_args(args: env::Args) -> Result<Vec<Arg>, Vec<String>> {
    parse_args_with_opts(args, ParseArgsSettings::new())
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //For a flag, just print the key
            Self::Flag(key) => {
                write!(f, "{key}")
            },
            //For a key-value pair, print both the key and the value
            Self::Pair(key, value) => {
                write!(f, "{key}: {value}")
            }
        }
    }
}