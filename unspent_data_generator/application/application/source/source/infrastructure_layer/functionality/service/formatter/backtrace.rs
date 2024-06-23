use super::Formatter;
use crate::infrastructure_layer::data::backtrace::Backtrace;

impl Formatter<Backtrace> {
    pub fn format<'a>(backtrace: &'a Backtrace) -> String {
        let mut message = String::new();

        '_a: for (index, backtrace_part) in backtrace.get_backtrace_part_registry().iter().enumerate() {
            message = if index == 0 {
                format!(
                    "({}) {}:{}.\n",
                    index,
                    backtrace_part.get_file_path(),
                    backtrace_part.get_line_number()
                )
            } else {
                format!(
                    "{}({}) {}:{}.\n",
                    message.as_str(),
                    index,
                    backtrace_part.get_file_path(),
                    backtrace_part.get_line_number()
                )
            };
        }

        return message;
    }
}
