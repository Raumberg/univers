mod objects;
mod system;
mod physics;
mod terminal;

use crate::system::SolarSystem;
use crate::terminal::TerminalInterface;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;
use std::io;


fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let mut solar_system = SolarSystem::new();
    let mut terminal_interface = TerminalInterface::new()?;
    let mut paused = false;
    let mut speed = 1.0;

    loop {

        clear_screen();

        for body in &solar_system.bodies {
            print!("{} {}", body.position.0, body.position.1);
        }

        if !paused {
            solar_system.run(speed);
        }

        terminal_interface.draw(&solar_system)?;

        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char(' ') => paused = !paused,
                KeyCode::Char('+') => speed += 0.1,
                KeyCode::Char('-') => speed -= 0.1,
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    solar_system = SolarSystem::new();
                    paused = false;
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16)); // 60 FPS
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}