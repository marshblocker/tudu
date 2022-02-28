use std::result;

use crate::entry::{Entry};
use crate::constants::{MAX_UNFINISHED_TODO, MAX_ID_COUNTER_VAL};
use crate::err::AppErr;

pub struct App {
    todo_list: Vec<Entry>,
    todo_count: u8,
    finished_todo_list: Vec<Entry>,
    id_counter: u32,
}

impl App {
    pub fn new() -> App {
        App { 
            todo_list: Vec::new(), 
            todo_count: 0,
            finished_todo_list: Vec::new(),
            id_counter: 0
        }
    }

    pub fn add_new_entry(&mut self) -> result::Result<(), String> {
        if self.todo_count == MAX_UNFINISHED_TODO {
            return Err(AppErr::MaxUnfinishedTodoReachedErr.to_str())
        }

        if self.id_counter == MAX_ID_COUNTER_VAL {
            self._renew_all_id();
        }

        let new = Entry::new(self.id_counter);

        for (i, entry) in self.todo_list.iter().enumerate() {
            if new.get_priority() >= entry.get_priority() {
                self.todo_list.insert(i, new);
                self.todo_count += 1;
                self.id_counter += 1;
                return Ok(())
            }
        };

        self.todo_list.push(new);
        self.todo_count += 1;
        self.id_counter += 1;

        Ok(())
    }

    pub fn entry_accomplished(&mut self, id: u32) {
        for (i, entry) in self.todo_list.iter().enumerate() {
            if entry.get_id() == id {
                let accomplished_entry = (*entry).clone();
                self.todo_list.remove(i);
                self.finished_todo_list.push(accomplished_entry);
                self.todo_count -= 1;
                return;
            }
        }

        panic!("The accomplished entry is not found in the todo_list.");
    }

    pub fn delete_entry(&mut self, id: u32) {
        for (i, entry) in self.todo_list.iter().enumerate() {
            if entry.get_id() == id { 
                self.todo_list.remove(i); 
                self.todo_count -= 1;
                break;
            }
        }
    }

    pub fn sort_todo_list(&mut self) {
        self.todo_list.sort_by(|a, b| 
            a.get_priority().partial_cmp(b.get_priority()).unwrap()
        );
    }

    fn _renew_all_id(&mut self) {
        for i in 0..self.todo_count {
            self.todo_list[i as usize].update_id(i as u32);
        }
    }

    fn _debug_print_entries(&self) {
        for entry in &self.todo_list {
            println!("{:?}", entry);
        }
    }
}

impl Default for App {
    fn default() -> App {
        App::new()
    }
}

pub fn run_app() -> result::Result<(), String> {
    let mut app = App::new();
    if let Err(e) = app.add_new_entry() { return Err(e); }
    if let Err(e) = app.add_new_entry() { return Err(e); }
    app._debug_print_entries();

    Ok(())
}