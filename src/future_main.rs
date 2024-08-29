use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui_splash_screen::{SplashConfig, SplashScreen, SplashError};

mod term;
mod space;
use crate::term::terminal::TerminalInterface;
use crate::term::terminal::prelude::*;
use crate::space::system::StarSystem;

static SPLASH: SplashConfig = SplashConfig {
    image_data: include_bytes!("../assets/splash.png"),
    // sha256sum: Some("c692ae1f9bd4a03cb6fc74a71cb585a8d70c2eacda8ec95e26aa0d6a0670cffd"),
    render_steps: 15,
    use_colors: true,
};

fn main() -> Result<(), std::io::Error> {
    let mut terminal_interface = TerminalInterface::new()?;
    let mut solar_system = StarSystem::solar();

    terminal_interface.run(&solar_system)?;

    // Clean up
    disable_raw_mode()?;
    execute!(terminal_interface.terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}