/// Goals:
/// * Create struct for todos. todos must contain the ff: DONE
///     - todo title
///     - checkbox
///     - priority level
/// * Limit number of unfinished todos to 15. DONE
/// * Limit length of entry name to 50. DONE
/// * Limit priority value from 0 to 2. DONE
/// * todo_list must be initially sorted by priority level. DONE
/// * Create keyboard commands.
/// * Create TUI.

use tudu::app;
    
fn main() {
    if let Err(e) = app::run_app() {
        eprintln!("{}", e);
    }
}