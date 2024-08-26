use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Style},

    Terminal,
};
use std::io;

use crate::system::SolarSystem;

pub struct TerminalInterface {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl TerminalInterface {
    // Constructor to create a new terminal interface
    pub fn new() -> Result<Self, io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(TerminalInterface { terminal })
    }

    // Method to draw the solar system simulation
    pub fn draw(&mut self, solar_system: &SolarSystem) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(f.area());

            let title = format!(
                "Solar System Simulation ({} bodies)",
                solar_system.bodies.len()
            );
            f.render_widget(
                ratatui::widgets::Paragraph::new(title.as_str())
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center),
                chunks[0],
            );

            let mut text = String::new();
            for body in &solar_system.bodies {
                text.push_str(&format!(
                    "{}: ({:.2}, {:.2})\n",
                    body.name, body.position.x, body.position.y
                ));
            }
            f.render_widget(
                ratatui::widgets::Paragraph::new(text.as_str())
                    .style(Style::default().fg(Color::White)),
                chunks[1],
            );

            let zoom = 1.0; // zoom level variable
            let _simspeed = 1.0; //simulation speed variable

            for body in &solar_system.bodies {
                let x = (body.position.x * zoom) as u16;
                let y = (body.position.y * zoom) as u16;
                f.set_cursor_position(Position::from((x, y)));
                // f.print_text(body.name);
            }
        })?;

        Ok(())
    }
}
