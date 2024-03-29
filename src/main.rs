use ctrlc;

use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossterm::{Result, input, InputEvent, KeyEvent};

mod termdisplay;

#[derive(Default)]
struct PlayerInput {
	left: bool,
	right: bool,
	up: bool,
	down: bool
}

fn main() -> Result<()> {
	// Catch Ctrl-C and gracefully exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
	let handler = move || {
		r.store(false, Ordering::SeqCst);
	};
	ctrlc::set_handler(handler).expect("Error setting handler");

	let cleanup = termdisplay::init()?;

	let input = input();
	let mut stdin = input.read_async();
	let mut player_input;

	let mut pos = (0, 0);
	while running.load(Ordering::SeqCst) {
        player_input = PlayerInput { ..Default::default() };

		while let Some(key_event) = stdin.next() {
			if !process_input_event(key_event, &mut player_input) {
				running.store(false, Ordering::SeqCst);
			}
		}
		
		update_sim(&player_input, &mut pos);

		termdisplay::clear()?;
		termdisplay::draw_border()?;

		let (x, y) = pos;
		termdisplay::draw_player(x, y, "A")?;

    	thread::sleep(time::Duration::from_millis(100));
    }

    cleanup()?;

    Ok(())
}

fn update_sim(player_input: &PlayerInput, (x, y): &mut (u16, u16)) {
    if player_input.left {
        if *x > 1 { *x -= 2; }
    }
    if player_input.right {
        if *x < 99 { *x += 2; }
    }    
    if player_input.up {
        if *y > 0 { *y -= 1; }
    }
    if player_input.down {
        if *y < 100 { *y += 1; }
    }
}

fn process_input_event(key_event: InputEvent, player_input: &mut PlayerInput) -> bool {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => match c {
                	'w' => {
                		player_input.up = true;
                	},
                	'a' => {
                		player_input.left = true;
                	},
                	's' => {
                		player_input.down = true;
                	},
                	'd' => {
                		player_input.right = true;
                	},
                	'q' => {
                		return false;
                	},
                	_ => ()
                },
            	_ => ()
            }
        },
        _ => ()
    };

    true
}