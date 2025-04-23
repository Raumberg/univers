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
mod space;

use crate::space::system::{StarSystem, Simulatable};

const MAX_TRAIL_LENGTH: usize = 200;

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
    show_detailed_info: bool,
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
            time_step: 3600.0,      // 1 hour in seconds
            time_elapsed: 0.0,
            scale: 1e-10,           // Scale factor for display
            focus_body_index: 0,    // Focus on Sun by default
            show_trails: true,
            show_velocity: true,
            show_detailed_info: false,
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
            SimulationSpeed::Cosmic => 1000,
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
            KeyCode::Char('i') => {
                self.show_detailed_info = !self.show_detailed_info;
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
        
        match days {
            d if d < 100.0 => {
                // For short periods, show days
                format!("{:.2} days", d)
            },
            d if d < 1000.0 => {
                // For medium periods, show months and days
                let months = (d / days_per_month).floor();
                let remaining_days = d % days_per_month;
                format!("{:.0} months, {:.1} days", months, remaining_days)
            },
            d if d > 100.0 * days_per_year => {
                // For very long periods, only show years
                format!("{:.1} years", days / days_per_year)
            },
            _ => {
                // For long periods (but not extremely long), show years, months, and days
                let years = (days / days_per_year).floor();
                let remaining_days = days % days_per_year;
                let months = (remaining_days / days_per_month).floor();
                let last_days = remaining_days % days_per_month;
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
    // First, determine if we need to show the detailed info panel
    let main_chunks = if app.show_detailed_info {
        // If detailed info is shown, use a horizontal split for the main area
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(5),    // Body (simulation and info)
                Constraint::Length(3), // Bottom controls
            ])
            .split(f.area())
    } else {
        // Original layout with just vertical split
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(5),    // Canvas for simulation
                Constraint::Length(3), // Info panel
            ])
            .split(f.area())
    };

    // Title
    let title = Paragraph::new("Unive.rs - Star System Simulation")
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, main_chunks[0]);

    // Simulation area
    if app.show_detailed_info {
        // Split the main area horizontally for simulation and detailed info
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Simulation takes 70%
                Constraint::Percentage(30), // Info panel takes 30%
            ])
            .split(main_chunks[1]);
            
        render_simulation_canvas(f, app, body_chunks[0]);
        render_detailed_info(f, app, body_chunks[1]);
    } else {
        // Just render the simulation canvas taking the full area
        render_simulation_canvas(f, app, main_chunks[1]);
    }

    // Bottom control panel
    render_control_panel(f, app, main_chunks[2]);
}

// Simulation canvas rendering (extracted from original ui function)
fn render_simulation_canvas(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let focus_body = &app.solar_system.bodies[app.focus_body_index];
    let focus_x = focus_body.position.x;
    let focus_y = focus_body.position.y;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([
            focus_x - (area.width as f64 / 2.0) / app.scale,
            focus_x + (area.width as f64 / 2.0) / app.scale,
        ])
        .y_bounds([
            focus_y - (area.height as f64 / 2.0) / app.scale,
            focus_y + (area.height as f64 / 2.0) / app.scale,
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
    f.render_widget(canvas, area);
}

fn format_value(value: f64, unit: &str) -> String {
    if value.abs() < 1.0 && value != 0.0 {
        // Small values
        if value.abs() < 0.000001 {
            format!("{:.2} nano{}", value * 1_000_000_000.0, unit)
        } else if value.abs() < 0.001 {
            format!("{:.2} micro{}", value * 1_000_000.0, unit)
        } else {
            format!("{:.2} milli{}", value * 1_000.0, unit)
        }
    } else if value.abs() >= 1_000_000_000_000.0 {
        // Trillions
        format!("{:.2} trillion {}", value / 1_000_000_000_000.0, unit)
    } else if value.abs() >= 1_000_000_000.0 {
        // Billions
        format!("{:.2} billion {}", value / 1_000_000_000.0, unit)
    } else if value.abs() >= 1_000_000.0 {
        // Millions
        format!("{:.2} million {}", value / 1_000_000.0, unit)
    } else if value.abs() >= 1_000.0 {
        // Thousands
        format!("{:.2} thousand {}", value / 1_000.0, unit)
    } else {
        // Regular values
        format!("{:.2} {}", value, unit)
    }
}

// Now modify the detailed info panel to use this function
fn render_detailed_info(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let focus_body = &app.solar_system.bodies[app.focus_body_index];
    
    // Format details for the current body with scientific notation for large numbers
    let velocity_magnitude = (focus_body.velocity.x.powi(2) + focus_body.velocity.y.powi(2)).sqrt();
    let distance_from_sun = if app.focus_body_index > 0 {
        let sun = &app.solar_system.bodies[0];
        let dx = focus_body.position.x - sun.position.x;
        let dy = focus_body.position.y - sun.position.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    } else {
        0.0 // Sun itself
    };
    
    // Calculate instantaneous acceleration
    let acceleration = if app.focus_body_index > 0 {
        (focus_body.acceleration.x.powi(2) + focus_body.acceleration.y.powi(2)).sqrt()
    } else {
        0.0 // Sun doesn't accelerate much
    };
    
    // Generate text lines
    let mut lines = vec![
        Line::from(Span::styled(
            format!(" {} Details ", focus_body.name),
            Style::default().fg(Color::Green)
        )),
        Line::from("―――――――――――――――――――――"),
        Line::from(Span::styled(
            format!("Mass: {}", format_value(focus_body.mass, "kg")),
            Style::default().fg(Color::White)
        )),
        Line::from(Span::styled(
            format!("Position: ({}, {})", 
                format_value(focus_body.position.x, "m"),
                format_value(focus_body.position.y, "m")),
            Style::default().fg(Color::White)
        )),
        Line::from(Span::styled(
            format!("Velocity: {}", format_value(velocity_magnitude, "m/s")),
            Style::default().fg(Color::White)
        )),
        Line::from(Span::styled(
            format!("Direction: ({:.2}, {:.2})", 
                focus_body.velocity.x / velocity_magnitude.max(1.0),
                focus_body.velocity.y / velocity_magnitude.max(1.0)),
            Style::default().fg(Color::White)
        )),
        Line::from(Span::styled(
            format!("Acceleration: {}", format_value(acceleration, "m/s²")),
            Style::default().fg(Color::White)
        )),
    ];
    
    // Add distance from Sun for non-Sun bodies
    if app.focus_body_index > 0 {
        lines.push(Line::from(Span::styled(
            format!("Distance from Sun: {}", format_value(distance_from_sun, "m")),
            Style::default().fg(Color::White)
        )));
        
        // Add orbital period (approximate)
        if velocity_magnitude > 0.0 {
            let orbital_circumference = 2.0 * std::f64::consts::PI * distance_from_sun;
            let orbital_period_seconds = orbital_circumference / velocity_magnitude;
            let orbital_period_days = orbital_period_seconds / 86400.0;
            
            lines.push(Line::from(Span::styled(
                format!("Orbital period: {:.2} days", orbital_period_days),
                Style::default().fg(Color::White)
            )));
        }
    }
    
    // Add gravity at surface (for planets)
    if app.focus_body_index > 0 {
        // Approximate radius based on mass
        let radius = (focus_body.mass / 5.5e3).powf(1.0/3.0); // Very rough approximation
        let surface_gravity = 6.67430e-11 * focus_body.mass / radius.powi(2);
        
        lines.push(Line::from(Span::styled(
            format!("Surface gravity: {:.2} m/s²", surface_gravity),
            Style::default().fg(Color::White)
        )));
        
        // Add Earth relative values
        let earth_gravity = 9.81;
        lines.push(Line::from(Span::styled(
            format!("Gravity vs Earth: {:.2}g", surface_gravity / earth_gravity),
            Style::default().fg(Color::White)
        )));
    }
    
    // Add help text at bottom
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press 'i' to hide this panel",
        Style::default().fg(Color::Gray)
    )));
    
    let detailed_info = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Celestial Body Information"))
        .style(Style::default().fg(Color::White));
        
    f.render_widget(detailed_info, area);
}

// Bottom control panel 
fn render_control_panel(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
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
            format!("Trails: {} | Vectors: {} | Info: {} | ", 
                if app.show_trails { "ON" } else { "OFF" },
                if app.show_velocity { "ON" } else { "OFF" },
                if app.show_detailed_info { "ON" } else { "OFF" }
            ),
            Style::default().fg(Color::Yellow)
        ),
        Span::styled("[1-6]speed [q]uit [i]nfo [space]pause [t]rails [v]ectors [+/-]zoom [←/→]focus", Style::default().fg(Color::Cyan)),
    ]);

    let info_panel = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(info_panel, area);
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