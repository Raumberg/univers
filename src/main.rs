use std::{io, time::{Duration, Instant}, collections::VecDeque};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, canvas::{Canvas, Line as CanvasLine}, Paragraph},
    Frame, Terminal,
};

mod engine;
mod gen;
mod space;

use crate::space::system::{StarSystem, Simulatable};

const MAX_TRAIL_LENGTH: usize = 100;

#[derive(PartialEq, Copy, Clone)]
enum SimulationSpeed {
    Paused,
    Slow,
    Normal,
    Fast,
    VeryFast,
    Extreme,
    Cosmic,
}

struct App {
    solar_system: StarSystem,
    simulation_running: bool,
    simulation_speed: SimulationSpeed,
    time_step: f64,
    time_elapsed: f64,
    scale: f64,
    focus_body_index: usize,
    show_trails: bool,
    show_velocity: bool,
    trails: Vec<VecDeque<(f64, f64)>>,
}

impl App {
    fn new() -> Self {
        // Initialize with one trail for each planet
        let num_bodies = StarSystem::solar().bodies.len();
        let mut trails = Vec::with_capacity(num_bodies);
        for _ in 0..num_bodies {
            trails.push(VecDeque::with_capacity(MAX_TRAIL_LENGTH));
        }

        App {
            solar_system: StarSystem::solar(),
            simulation_running: true,
            simulation_speed: SimulationSpeed::Normal,
            time_step: 3600.0, // 1 hour in seconds
            time_elapsed: 0.0,
            scale: 1e-10, // Scale factor for display
            focus_body_index: 0, // Focus on Sun by default
            show_trails: true,
            show_velocity: true,
            trails,
        }
    }

    fn update_trails(&mut self) {
        // Store current positions in the trails
        for (i, body) in self.solar_system.bodies.iter().enumerate() {
            if i < self.trails.len() {
                self.trails[i].push_back((body.position.x, body.position.y));
                if self.trails[i].len() > MAX_TRAIL_LENGTH {
                    self.trails[i].pop_front();
                }
            }
        }
    }

    fn on_tick(&mut self) {
        if !self.simulation_running || self.simulation_speed == SimulationSpeed::Paused {
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
            self.solar_system.simulate(self.time_step, steps);
            self.time_elapsed += self.time_step * steps as f64;
            self.update_trails();
        }
    }

    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {
                self.simulation_running = false;
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
                self.focus_body_index = (self.focus_body_index + 1) % self.solar_system.bodies.len();
            }
            KeyCode::Left => {
                if self.focus_body_index > 0 {
                    self.focus_body_index -= 1;
                } else {
                    self.focus_body_index = self.solar_system.bodies.len() - 1;
                }
            }
            KeyCode::Char('t') => {
                self.show_trails = !self.show_trails;
            }
            KeyCode::Char('v') => {
                self.show_velocity = !self.show_velocity;
            }
            KeyCode::Char('r') => {
                self.solar_system = StarSystem::solar();
                self.time_elapsed = 0.0;
                // Clear trails
                for trail in &mut self.trails {
                    trail.clear();
                }
            }
            KeyCode::Char('c') => {
                // Clear trails but keep current position
                for trail in &mut self.trails {
                    trail.clear();
                }
                self.update_trails();
            }
            _ => {}
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
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                app.on_key(key.code);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Canvas for simulation
            Constraint::Length(3), // Info panel
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Univers - Star System Simulation")
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Simulation Canvas
    let focus_body = &app.solar_system.bodies[app.focus_body_index];
    let focus_x = focus_body.position.x;
    let focus_y = focus_body.position.y;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([
            focus_x - (chunks[1].width as f64 / 2.0) / app.scale,
            focus_x + (chunks[1].width as f64 / 2.0) / app.scale,
        ])
        .y_bounds([
            focus_y - (chunks[1].height as f64 / 2.0) / app.scale,
            focus_y + (chunks[1].height as f64 / 2.0) / app.scale,
        ])
        .paint(|ctx| {
            // Draw orbital trails if enabled
            if app.show_trails {
                for (i, trail) in app.trails.iter().enumerate() {
                    if i == 0 && trail.len() > 1 { continue; } // Skip Sun's trail

                    let trail_color = get_body_color(i);
                    
                    // Draw trail (connecting the points)
                    let trail_vec: Vec<&(f64, f64)> = trail.iter().collect();
                    for window in trail_vec.windows(2) {
                        if let [&p1, &p2] = window {
                            ctx.draw(&CanvasLine {
                                x1: p1.0,
                                y1: p1.1,
                                x2: p2.0,
                                y2: p2.1,
                                color: trail_color,
                            });
                        }
                    }
                }
            }
            
            // Draw celestial bodies
            for (i, body) in app.solar_system.bodies.iter().enumerate() {
                let color = get_body_color(i);

                // Draw the body
                if i == 0 {
                    // Sun is a special symbol
                    ctx.print(body.position.x, body.position.y, "☀");
                } else {
                    // Planet size based on its position in the system (just for visualization)
                    let planet_symbol = match i {
                        1 | 2 => "•", // Mercury, Venus - small
                        3 | 4 => "○", // Earth, Mars - medium
                        5 | 6 => "◎", // Jupiter, Saturn - large
                        _ => "◉",     // Others - medium-large
                    };
                    ctx.print(body.position.x, body.position.y, planet_symbol);
                }
                
                // Draw velocity vector if enabled
                if app.show_velocity && i > 0 {
                    let vel_scale = 1e6; // Scale the velocity vector for visibility
                    let end_x = body.position.x + body.velocity.x * vel_scale;
                    let end_y = body.position.y + body.velocity.y * vel_scale;
                    ctx.draw(&CanvasLine {
                        x1: body.position.x,
                        y1: body.position.y,
                        x2: end_x,
                        y2: end_y,
                        color,
                    });
                }

                // Highlight focused body with an arrow below
                if i == app.focus_body_index {
                    ctx.print(
                        body.position.x, 
                        body.position.y + 1.0, 
                        format!("↑ {}", body.name)
                    );
                }
            }
        });
    f.render_widget(canvas, chunks[1]);

    // Info panel at the bottom
    let focus_body = &app.solar_system.bodies[app.focus_body_index];
    let velocity_magnitude = (focus_body.velocity.x.powi(2) + focus_body.velocity.y.powi(2)).sqrt();
    
    let status_text = match app.simulation_speed {
        SimulationSpeed::Paused => "PAUSED",
        SimulationSpeed::Slow => "SLOW",
        SimulationSpeed::Normal => "NORMAL",
        SimulationSpeed::Fast => "FAST",
        SimulationSpeed::VeryFast => "VERY FAST",
        SimulationSpeed::Extreme => "EXTREME",
        SimulationSpeed::Cosmic => "COSMIC",
    };
    
    let status_color = match app.simulation_speed {
        SimulationSpeed::Paused => Color::Red,
        SimulationSpeed::Slow => Color::Yellow,
        SimulationSpeed::Normal => Color::Green,
        SimulationSpeed::Fast => Color::Cyan,
        SimulationSpeed::VeryFast => Color::Blue, 
        SimulationSpeed::Extreme => Color::Magenta,
        SimulationSpeed::Cosmic => Color::LightMagenta,
    };
    
    let info = Line::from(vec![
        Span::styled(format!("Time: {} | ", app.format_time_elapsed()), Style::default().fg(Color::Gray)),
        Span::styled(format!("Focus: {} | ", focus_body.name), Style::default().fg(Color::White)),
        Span::styled(format!("Vel: {:.2} km/s | ", velocity_magnitude / 1000.0), Style::default().fg(Color::White)),
        Span::styled(
            format!("Speed: {} | ", status_text),
            Style::default().fg(status_color)
        ),
        Span::styled(
            format!("Trails: {} | Vectors: {} | ", 
                if app.show_trails { "ON" } else { "OFF" },
                if app.show_velocity { "ON" } else { "OFF" }
            ),
            Style::default().fg(Color::Yellow)
        ),
        Span::styled("[1-6]speed [q]uit [space]pause [t]rails [v]ectors [+/-]zoom [←/→]focus [r]eset [c]lear", Style::default().fg(Color::Cyan)),
    ]);

    let info_panel = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(info_panel, chunks[2]);
}

fn get_body_color(index: usize) -> Color {
    match index {
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
    }
}