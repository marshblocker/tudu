use std::{result, io};

use crate::err::AppErr;
use crate::constants::{MAX_ENTRY_NAME_LEN, MAX_ENTRY_PRIO_VAL};

#[derive(Clone, Debug)]
pub struct EntryName(String);

impl EntryName {
    pub fn new() -> result::Result<EntryName, String> {
        println!("Entry name (Max: {} characters):", MAX_ENTRY_NAME_LEN);
        let mut entry_name = String::new();
        io::stdin()
            .read_line(&mut entry_name)
            .expect("Can't read entry name input!");
        let entry_name = entry_name.trim().to_string();

        if entry_name.len() > MAX_ENTRY_NAME_LEN { 
            return Err(AppErr::MaxEntryNameLenErr.to_str()) 
        }

        Ok(EntryName(entry_name))
    }
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct EntryPriority(u8);

impl EntryPriority {
    pub fn new() -> result::Result<EntryPriority, String> {
        println!("Entry priority (0-lowest, {}-highest): ", MAX_ENTRY_PRIO_VAL);
        let mut priority = String::new();

        if io::stdin().read_line(&mut priority).is_err() {
            return Err(AppErr::IOError.to_str());
        }

        let priority = match priority.trim().parse::<u8>() {
            Ok(val) => {
                if val <= MAX_ENTRY_PRIO_VAL { EntryPriority(val) }
                else { return Err(AppErr::InvalidEntryPriorityValErr.to_str()); }
            },
            Err(_) => return Err(AppErr::InvalidEntryPriorityValErr.to_str())
        };
        
        Ok(priority)
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    name: EntryName,
    is_done: bool,
    priority: EntryPriority,
    id: u32
}

impl Entry {
    pub fn new( id: u32) -> Entry {
        let name = loop {
            match EntryName::new() {
                Ok(name) => break name,
                Err(e) => eprintln!("{}", e)
            }
        };
        let priority = loop {
            match EntryPriority::new() {
                Ok(priority) => break priority,
                Err(e) => eprintln!("{}", e)
            }
        };
        Entry { name, is_done: false, priority, id }
    }

    pub fn get_name(&self) -> &EntryName {
        &self.name
    }

    pub fn get_is_done(&self) -> bool {
        self.is_done
    }

    pub fn get_priority(&self) -> &EntryPriority {
        &self.priority
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn update_name(&mut self, new_name: EntryName) {
        self.name = new_name;
    }

    pub fn entry_accomplished(&mut self) {
        self.is_done = true;
    }

    pub fn update_priority(&mut self, new_prio: EntryPriority) {
        self.priority = new_prio;
    }

    pub fn update_id(&mut self, new_id: u32) {
        self.id = new_id;
    }
}