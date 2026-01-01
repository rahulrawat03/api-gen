use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Arguments {
    pub options: HashMap<String, String>,
    pub flags: HashSet<String>,
}

impl Arguments {
    pub fn parse(args: &[String]) -> Self {
        let mut arguments = Self {
            options: HashMap::new(),
            flags: HashSet::new(),
        };

        let mut current_arg_name = None;

        for arg in args {
            if arg.starts_with("--") {
                if let Some(previous_arg_name) = current_arg_name {
                    arguments.flags.insert(previous_arg_name);
                }

                current_arg_name = Some(arg.replace("--", "").to_string());
            } else {
                if let Some(arg_name) = current_arg_name {
                    arguments.options.insert(arg_name, arg.to_string());
                }

                current_arg_name = None;
            }
        }

        if let Some(previous_arg_name) = current_arg_name {
            arguments.flags.insert(previous_arg_name);
        }

        arguments
    }
}
