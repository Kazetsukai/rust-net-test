use ctrlc;

use std::{thread, time, process};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossterm::{cursor, terminal, AlternateScreen, ClearType, Result};

fn main() -> Result<()> {
	let screen = AlternateScreen::to_alternate(false)?;

	// Catch Ctrl-C and gracefully exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	let handler = move || {
		r.store(false, Ordering::SeqCst);
	};
	ctrlc::set_handler(handler).expect("Error setting handler");

	let cursor = cursor();
	let terminal = terminal();

	cursor.hide()?;

	let mut i = 0;
	while running.load(Ordering::SeqCst) {
		terminal.clear(ClearType::All)?;
		draw_border()?;
		draw_player(i, 0, "X")?;
		i += 1;
		if i > 100 {
			running.store(false, Ordering::SeqCst);
		}
    	thread::sleep(time::Duration::from_millis(50));
    }

    // Clean up terminal
	cursor.show()?;
	screen.to_main()?;

    Ok(())
}

fn draw_player(x: u16, y: u16, ch: &str) -> Result<()> {
	let cursor = cursor();
	let terminal = terminal();

	cursor.goto(x + 6, y + 6)?;
	terminal.write(ch)?;

	Ok(())
}

fn draw_border() -> Result<()> {
	let cursor = cursor();
	let terminal = terminal();

	cursor.goto(5, 5)?;
    terminal.write("#############################################################")?;

    for y in 6..30 {
	    cursor.goto(5, y)?;
	    terminal.write("#")?;
	    cursor.goto(65, y)?;
	    terminal.write("#")?;
    }

    cursor.goto(5, 30)?;
    terminal.write("#############################################################")?;

    Ok(())
}