use crate::constants::{MAX_UNFINISHED_TODO, MAX_ENTRY_NAME_LEN, MAX_ENTRY_PRIO_VAL};

pub enum AppErr {
    MaxUnfinishedTodoReachedErr,
    MaxEntryNameLenErr,
    InvalidEntryPriorityValErr,
    IOError,
}

impl AppErr {
    pub fn to_str(&self) -> String {
        let error = "Error: ".to_string();
        let message = match self {
            AppErr::MaxUnfinishedTodoReachedErr => format!("Reached max number of unfinished todos ({}), consider finishing some of it before adding more. :)", MAX_UNFINISHED_TODO),
            AppErr::MaxEntryNameLenErr => format!("The length of the given entry name has reached the max length of {}.", MAX_ENTRY_NAME_LEN),
            AppErr::InvalidEntryPriorityValErr => format!("Entry priority value is within the range [0, {}] only.", MAX_ENTRY_PRIO_VAL),
            AppErr::IOError => String::from("Failed to read input."),
        };
        let error = error + message.as_str();

        error
    }
}