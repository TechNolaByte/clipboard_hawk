use clipboard_rs::{
	Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext,
};
use std::{thread, time::Duration};

struct Manager {
	ctx: ClipboardContext,
}

impl Manager {
	pub fn new() -> Self {
		let ctx = ClipboardContext::new().unwrap();
		Manager { ctx }
	}
}

impl ClipboardHandler for Manager {
	fn on_clipboard_change(&mut self) {
        match self.ctx.get_text() {
            Ok(x) => println!("{}", x),
            Err(e) => println!("!ERROR! {:?}", e),
        }
	}
}

fn main() {
	let manager = Manager::new();
	let mut watcher: ClipboardWatcherContext<Manager> = ClipboardWatcherContext::new().unwrap();
    watcher.add_handler(manager);
	watcher.start_watch();
}