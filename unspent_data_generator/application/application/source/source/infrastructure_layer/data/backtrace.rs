use std::fmt::{
    Display,
    Error as FmtError,
    Formatter,
};

#[derive(Debug)]
pub struct Backtrace {
    backtrace_part_registry: Vec<BacktracePart>,
}

impl Backtrace {
    pub fn new(backtrace_part: BacktracePart) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part],
        };
    }

    pub fn add<'a>(&'a mut self, backtrace_part: BacktracePart) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }

    pub fn get_backtrace_part_registry<'a>(&'a self) -> &'a [BacktracePart] {
        return self.backtrace_part_registry.as_slice();
    }
}

impl Display for Backtrace {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
}

impl BacktracePart {
    pub fn new(line_number: u32, file_path: &'static str) -> Self {
        return Self {
            line_number,
            file_path,
        };
    }

    pub fn get_line_number<'a>(&'a self) -> u32 {
        return self.line_number;
    }

    pub fn get_file_path<'a>(&'a self) -> &'static str {
        return self.file_path;
    }
}
