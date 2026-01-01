use crate::arguments::Arguments;

pub trait Executable {
    fn execute(&self, arguments: &Arguments);
}
