use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Canvas, Paragraph, Widget, Clear},
    Frame, Terminal,
};

use crate::space::objects::CelestialObject;
use crate::space::system::{StarSystem, Simulatable};

#[derive(PartialEq, Copy, Clone)]
pub enum SimulationSpeed {
    Paused,
    Slow,
    Normal,
    Fast,
    VeryFast,
    Extreme,
    Cosmic,
}

pub struct StarSystemApp {
    pub simulation_speed: SimulationSpeed,
    pub running: bool,
    pub time_step: f64,
    pub time_elapsed: f64,
    pub scale: f64,
    pub focus_body_index: usize,
    pub show_info: bool,
    pub show_help: bool,
    pub show_trails: bool,
    pub trails: Vec<Vec<(f64, f64)>>,
}

impl StarSystemApp {
    pub fn new() -> Self {
        StarSystemApp {
            simulation_speed: SimulationSpeed::Normal,
            running: true,
            time_step: 3600.0, // 1 hour in seconds
            time_elapsed: 0.0,
            scale: 1e-10, // Scale factor for display (AU to pixels)
            focus_body_index: 0, // Default focus on the Sun
            show_info: true,
            show_help: false,
            show_trails: true,
            trails: vec![Vec::new(); 9], // One trail for each planet
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>, system: &mut StarSystem) -> io::Result<()> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(33); // ~30 FPS

        // Initialize trails with current positions
        for (i, body) in system.bodies.iter().enumerate() {
            if i < self.trails.len() {
                self.trails[i].push((body.position.x, body.position.y));
                // Limit trail length to avoid performance issues
                if self.trails[i].len() > 1000 {
                    self.trails[i].remove(0);
                }
            }
        }

        while self.running {
            terminal.draw(|f| self.ui(f, system))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key.code, system);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick(system);
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyCode, system: &mut StarSystem) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            KeyCode::Char(' ') => {
                if self.simulation_speed == SimulationSpeed::Paused {
                    self.simulation_speed = SimulationSpeed::Normal;
                } else {
                    self.simulation_speed = SimulationSpeed::Paused;
                }
            }
            KeyCode::Char('1') => {
                self.simulation_speed = SimulationSpeed::Slow;
            }
            KeyCode::Char('2') => {
                self.simulation_speed = SimulationSpeed::Normal;
            }
            KeyCode::Char('3') => {
                self.simulation_speed = SimulationSpeed::Fast;
            }
            KeyCode::Char('4') => {
                self.simulation_speed = SimulationSpeed::VeryFast;
            }
            KeyCode::Char('5') => {
                self.simulation_speed = SimulationSpeed::Extreme;
            }
            KeyCode::Char('6') => {
                self.simulation_speed = SimulationSpeed::Cosmic;
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.scale *= 1.5;
            }
            KeyCode::Char('-') => {
                self.scale /= 1.5;
            }
            KeyCode::Right => {
                self.focus_body_index = (self.focus_body_index + 1) % system.bodies.len();
            }
            KeyCode::Left => {
                if self.focus_body_index > 0 {
                    self.focus_body_index -= 1;
                } else {
                    self.focus_body_index = system.bodies.len() - 1;
                }
            }
            KeyCode::Char('i') => {
                self.show_info = !self.show_info;
            }
            KeyCode::Char('h') => {
                self.show_help = !self.show_help;
            }
            KeyCode::Char('t') => {
                self.show_trails = !self.show_trails;
            }
            KeyCode::Char('r') => {
                // Reset the system to initial state
                *system = StarSystem::solar();
                self.time_elapsed = 0.0;
                self.trails = vec![Vec::new(); 9];
            }
            KeyCode::Char('c') => {
                // Clear trails
                self.trails = vec![Vec::new(); 9];
                for (i, body) in system.bodies.iter().enumerate() {
                    if i < self.trails.len() {
                        self.trails[i].push((body.position.x, body.position.y));
                    }
                }
            }
            _ => {}
        }
    }

    fn on_tick(&mut self, system: &mut StarSystem) {
        // Skip simulation if paused
        if self.simulation_speed == SimulationSpeed::Paused {
            return;
        }

        // Determine number of simulation steps based on speed
        let steps = match self.simulation_speed {
            SimulationSpeed::Paused => 0,
            SimulationSpeed::Slow => 1,
            SimulationSpeed::Normal => 5,
            SimulationSpeed::Fast => 20,
            SimulationSpeed::VeryFast => 100,
            SimulationSpeed::Extreme => 500,
            SimulationSpeed::Cosmic => 2000,
        };

        if steps > 0 {
            system.simulate(self.time_step, steps);
            self.time_elapsed += self.time_step * steps as f64;
            
            // Store current positions for trails
            for (i, body) in system.bodies.iter().enumerate() {
                if i < self.trails.len() {
                    self.trails[i].push((body.position.x, body.position.y));
                    // Limit trail length to avoid performance issues
                    if self.trails[i].len() > 1000 {
                        self.trails[i].remove(0);
                    }
                }
            }
        }
    }
    
    // Format time elapsed into appropriate units (days, months, years)
    fn format_time_elapsed(&self) -> String {
        let seconds_per_day = 86400.0;
        let days_per_month = 30.44; // Average month length
        let days_per_year = 365.25; // Including leap years
        
        let days = self.time_elapsed / seconds_per_day;
        
        if days < 100.0 {
            // For short periods, show days
            format!("{:.2} days", days)
        } else if days < 1000.0 {
            // For medium periods, show months and days
            let months = (days / days_per_month).floor();
            let remaining_days = days % days_per_month;
            format!("{:.0} months, {:.1} days", months, remaining_days)
        } else {
            // For long periods, show years, months, and days
            let years = (days / days_per_year).floor();
            let remaining_days = days % days_per_year;
            let months = (remaining_days / days_per_month).floor();
            let last_days = remaining_days % days_per_month;
            
            if years > 100.0 {
                // For very long periods, only show years
                format!("{:.1} years", days / days_per_year)
            } else {
                // Otherwise show years, months, and days
                format!("{:.0} years, {:.0} months, {:.1} days", years, months, last_days)
            }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, system: &StarSystem) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(5),    // Canvas for simulation
                Constraint::Length(if self.show_info { 8 } else { 1 }), // Info panel
            ])
            .split(f.size());

        // Title
        let title = Paragraph::new("Univers - Star System Simulation")
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Simulation Canvas
        let focus_body = &system.bodies[self.focus_body_index];
        let focus_x = focus_body.position.x;
        let focus_y = focus_body.position.y;

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL))
            .x_bounds([
                focus_x - (chunks[1].width as f64 / 2.0) / self.scale,
                focus_x + (chunks[1].width as f64 / 2.0) / self.scale,
            ])
            .y_bounds([
                focus_y - (chunks[1].height as f64 / 2.0) / self.scale,
                focus_y + (chunks[1].height as f64 / 2.0) / self.scale,
            ])
            .paint(|ctx| {
                // Draw orbital trails if enabled
                if self.show_trails {
                    for (i, trail) in self.trails.iter().enumerate() {
                        if i == 0 { continue; } // Skip Sun's trail
                        
                        let color = match i {
                            1 => Color::Gray,       // Mercury
                            2 => Color::LightYellow, // Venus
                            3 => Color::Blue,       // Earth
                            4 => Color::Red,        // Mars
                            5 => Color::LightRed,   // Jupiter
                            6 => Color::LightMagenta, // Saturn
                            7 => Color::Cyan,       // Uranus
                            8 => Color::Blue,       // Neptune
                            _ => Color::White,
                        };
                        
                        // Draw trail (connecting the points)
                        for points in trail.windows(2) {
                            if let [p1, p2] = points {
                                ctx.line(p1.0, p1.1, p2.0, p2.1, color);
                            }
                        }
                    }
                }
                
                // Draw celestial bodies
                for (i, body) in system.bodies.iter().enumerate() {
                    let color = match i {
                        0 => Color::Yellow,      // Sun
                        1 => Color::Gray,        // Mercury
                        2 => Color::LightYellow, // Venus
                        3 => Color::Blue,        // Earth
                        4 => Color::Red,         // Mars
                        5 => Color::LightRed,    // Jupiter
                        6 => Color::LightMagenta, // Saturn
                        7 => Color::Cyan,        // Uranus
                        8 => Color::Blue,        // Neptune
                        _ => Color::White,
                    };

                    // Draw the body
                    if i == 0 {
                        // Sun is a special case
                        ctx.print(body.position.x, body.position.y, "☀", Style::default().fg(color));
                    } else {
                        // Planet size based on its position in the system (just for visualization)
                        let planet_symbol = match i {
                            1 | 2 => "•", // Mercury, Venus - small
                            3 | 4 => "○", // Earth, Mars - medium
                            5 | 6 => "◎", // Jupiter, Saturn - large
                            _ => "◉",     // Others - medium-large
                        };
                        ctx.print(body.position.x, body.position.y, planet_symbol, Style::default().fg(color));
                    }

                    // Highlight focused body
                    if i == self.focus_body_index {
                        ctx.print(
                            body.position.x, 
                            body.position.y + 1.0, 
                            format!("↑ {}", body.name),
                            Style::default().fg(Color::White)
                        );
                    }
                }
            });
        f.render_widget(canvas, chunks[1]);

        // Info panel
        if self.show_info {
            let focus_body = &system.bodies[self.focus_body_index];
            
            let info = vec![
                Line::from(vec![
                    Span::styled("Time Elapsed: ", Style::default().fg(Color::Gray)),
                    Span::styled(self.format_time_elapsed(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Focus: ", Style::default().fg(Color::Gray)),
                    Span::styled(&focus_body.name, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Position: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("({:.2e}, {:.2e}) m", focus_body.position.x, focus_body.position.y),
                        Style::default().fg(Color::White)
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Velocity: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("({:.2e}, {:.2e}) m/s", focus_body.velocity.x, focus_body.velocity.y),
                        Style::default().fg(Color::White)
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Speed: ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        match self.simulation_speed {
                            SimulationSpeed::Paused => "PAUSED",
                            SimulationSpeed::Slow => "SLOW",
                            SimulationSpeed::Normal => "NORMAL",
                            SimulationSpeed::Fast => "FAST",
                            SimulationSpeed::VeryFast => "VERY FAST",
                            SimulationSpeed::Extreme => "EXTREME",
                            SimulationSpeed::Cosmic => "COSMIC",
                        },
                        Style::default().fg(match self.simulation_speed {
                            SimulationSpeed::Paused => Color::Red,
                            SimulationSpeed::Slow => Color::Yellow,
                            SimulationSpeed::Normal => Color::Green,
                            SimulationSpeed::Fast => Color::Cyan,
                            SimulationSpeed::VeryFast => Color::Blue,
                            SimulationSpeed::Extreme => Color::Magenta,
                            SimulationSpeed::Cosmic => Color::LightMagenta,
                        })
                    ),
                ]),
            ];

            let info_panel = Paragraph::new(info)
                .block(Block::default().borders(Borders::ALL).title("Information"))
                .style(Style::default().fg(Color::White));
            f.render_widget(info_panel, chunks[2]);
        }

        // Help overlay
        if self.show_help {
            let area = centered_rect(60, 60, f.size());
            f.render_widget(Clear, area);
            
            let help_text = vec![
                Line::from("Controls:"),
                Line::from(""),
                Line::from(vec![
                    Span::styled("q, Esc", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Quit"),
                ]),
                Line::from(vec![
                    Span::styled("Space", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Pause/Resume simulation"),
                ]),
                Line::from(vec![
                    Span::styled("1-6", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Set simulation speed (Slow to Cosmic)"),
                ]),
                Line::from(vec![
                    Span::styled("+, -", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Zoom in/out"),
                ]),
                Line::from(vec![
                    Span::styled("Left/Right", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Change focus to previous/next body"),
                ]),
                Line::from(vec![
                    Span::styled("i", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Toggle information panel"),
                ]),
                Line::from(vec![
                    Span::styled("t", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Toggle orbital trails"),
                ]),
                Line::from(vec![
                    Span::styled("c", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Clear orbital trails"),
                ]),
                Line::from(vec![
                    Span::styled("h", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Toggle this help screen"),
                ]),
                Line::from(vec![
                    Span::styled("r", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Reset simulation"),
                ]),
            ];

            let help = Paragraph::new(help_text)
                .block(Block::default().borders(Borders::ALL).title("Help"))
                .style(Style::default().fg(Color::White));
            f.render_widget(help, area);
        }
    }
}

// Helper function for centered rect layout
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
} 