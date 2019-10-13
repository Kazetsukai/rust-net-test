use crossterm::{AlternateScreen, RawScreen, ClearType, cursor, terminal, Result};

pub fn init() -> Result<impl FnOnce() -> Result<()>> {
	let cursor = cursor();
	let screen = AlternateScreen::to_alternate(false)?;
	let raw = RawScreen::into_raw_mode()?;
	cursor.hide()?;

	Ok(move || -> Result<()> {
		cursor.show()?;
		drop(raw);
		screen.to_main()?;
		Ok(())
	})
}

pub fn clear() -> Result<()> {
	let terminal = terminal();
	terminal.clear(ClearType::All)
}

pub fn draw_player(x: u16, y: u16, ch: &str) -> Result<()> {
	let cursor = cursor();
	let terminal = terminal();

	cursor.goto(x + 6, y + 6)?;
	terminal.write(ch)?;

	Ok(())
}

pub fn draw_border() -> Result<()> {
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