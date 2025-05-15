use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, canvas::{Canvas, Line as CanvasLine}, Paragraph},
};
use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let sim = app.simulation.lock().unwrap();
    let bodies = sim.system.bodies.clone();
    let time_elapsed = sim.time_elapsed;
    let speed = sim.speed;
    let running = sim.running;
    drop(sim);
    let main_chunks = if app.show_detailed_info {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(f.area())
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(f.area())
    };
    let title = Paragraph::new("Unive.rs - Star System Simulation")
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, main_chunks[0]);
    if app.show_detailed_info {
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(main_chunks[1]);
        render_simulation_canvas(f, app, &bodies, body_chunks[0]);
        render_detailed_info(f, app, &bodies, body_chunks[1], time_elapsed);
    } else {
        render_simulation_canvas(f, app, &bodies, main_chunks[1]);
    }
    render_control_panel(f, app, &bodies, main_chunks[2], time_elapsed, speed, running);
}

pub fn render_simulation_canvas(f: &mut Frame, app: &App, bodies: &Vec<crate::space::objects::CelestialObject>, area: ratatui::layout::Rect) {
    let focus_body = &bodies[app.focus_body_index];
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
            if app.show_trails {
                for (i, trail) in app.trails.iter().enumerate() {
                    if i == 0 && trail.len() > 1 { continue; }
                    let trail_color = get_body_color(i);
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
            for (i, body) in bodies.iter().enumerate() {
                let color = match body.kind {
                    crate::space::objects::CelestialType::Particle => Color::White,
                    _ => get_body_color(i),
                };
                if body.kind == crate::space::objects::CelestialType::Particle {
                    ctx.print(body.position.x, body.position.y, ".");
                } else {
                    let emoji = match body.name.as_str() {
                        "Sun" => "☀️",
                        "Mercury" => "🪨",
                        "Venus" => "🟤",
                        "Earth" => "🌍",
                        "Mars" => "🪨",
                        "Jupiter" => "🪐",
                        "Saturn" => "🪐",
                        "Uranus" => "🌀",
                        "Neptune" => "🌀",
                        _ => "◉",
                    };
                    ctx.print(body.position.x, body.position.y, emoji);
                }
                if app.show_velocity && i > 0 && body.kind != crate::space::objects::CelestialType::Particle {
                    let vel_scale = 1e6;
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
                if i == app.focus_body_index && body.kind != crate::space::objects::CelestialType::Particle {
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

pub fn format_value(value: f64, unit: &str) -> String {
    if value.abs() < 1.0 && value != 0.0 {
        if value.abs() < 0.000001 {
            format!("{:.2} nano{}", value * 1_000_000_000.0, unit)
        } else if value.abs() < 0.001 {
            format!("{:.2} micro{}", value * 1_000_000.0, unit)
        } else {
            format!("{:.2} milli{}", value * 1_000.0, unit)
        }
    } else if value.abs() >= 1_000_000_000_000.0 {
        format!("{:.2} trillion {}", value / 1_000_000_000_000.0, unit)
    } else if value.abs() >= 1_000_000_000.0 {
        format!("{:.2} billion {}", value / 1_000_000_000.0, unit)
    } else if value.abs() >= 1_000_000.0 {
        format!("{:.2} million {}", value / 1_000_000.0, unit)
    } else if value.abs() >= 1_000.0 {
        format!("{:.2} thousand {}", value / 1_000.0, unit)
    } else {
        format!("{:.2} {}", value, unit)
    }
}

pub fn render_detailed_info(f: &mut Frame, app: &App, bodies: &Vec<crate::space::objects::CelestialObject>, area: ratatui::layout::Rect, _time_elapsed: f64) {
    let focus_body = &bodies[app.focus_body_index];
    let velocity_magnitude = (focus_body.velocity.x.powi(2) + focus_body.velocity.y.powi(2)).sqrt();
    let distance_from_sun = if app.focus_body_index > 0 {
        let sun = &bodies[0];
        let dx = focus_body.position.x - sun.position.x;
        let dy = focus_body.position.y - sun.position.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    } else {
        0.0
    };
    let acceleration = if app.focus_body_index > 0 {
        (focus_body.acceleration.x.powi(2) + focus_body.acceleration.y.powi(2)).sqrt()
    } else {
        0.0
    };
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
    if app.focus_body_index > 0 {
        lines.push(Line::from(Span::styled(
            format!("Distance from Sun: {}", format_value(distance_from_sun, "m")),
            Style::default().fg(Color::White)
        )));
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
    if app.focus_body_index > 0 {
        let radius = (focus_body.mass / 5.5e3).powf(1.0/3.0);
        let surface_gravity = 6.67430e-11 * focus_body.mass / radius.powi(2);
        lines.push(Line::from(Span::styled(
            format!("Surface gravity: {:.2} m/s²", surface_gravity),
            Style::default().fg(Color::White)
        )));
        let earth_gravity = 9.81;
        lines.push(Line::from(Span::styled(
            format!("Gravity vs Earth: {:.2}g", surface_gravity / earth_gravity),
            Style::default().fg(Color::White)
        )));
    }
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

pub fn render_control_panel(f: &mut Frame, app: &App, bodies: &Vec<crate::space::objects::CelestialObject>, area: ratatui::layout::Rect, time_elapsed: f64, speed: usize, running: bool) {
    let focus_body = &bodies[app.focus_body_index];
    let velocity_magnitude = (focus_body.velocity.x.powi(2) + focus_body.velocity.y.powi(2)).sqrt();
    let (status_text, status_color) = if !running {
        ("PAUSED", Color::Red)
    } else {
        match speed {
            1 => ("SLOW", Color::Yellow),
            5 => ("NORMAL", Color::Green),
            20 => ("FAST", Color::Cyan),
            100 => ("VERY FAST", Color::Blue),
            500 => ("EXTREME", Color::Magenta),
            1000 => ("COSMIC", Color::LightMagenta),
            _ => ("CUSTOM", Color::White),
        }
    };
    let algo_str = match app.simulation.lock().unwrap().algorithm {
        crate::space::system::SimulationAlgorithm::Direct => "Direct (N²)",
        crate::space::system::SimulationAlgorithm::BarnesHut => "Barnes-Hut",
    };
    let g_val = app.simulation.lock().unwrap().system.g;
    let info = Line::from(vec![
        Span::styled(format!("Time: {} | ", app.format_time_elapsed()), Style::default().fg(Color::Gray)),
        Span::styled(format!("Focus: {} | ", focus_body.name), Style::default().fg(Color::White)),
        Span::styled(format!("Vel: {:.2} km/s | ", velocity_magnitude / 1000.0), Style::default().fg(Color::White)),
        Span::styled(
            format!("Speed: {} | ", status_text),
            Style::default().fg(status_color)
        ),
        Span::styled(
            format!("Algo: {} | ", algo_str),
            Style::default().fg(Color::Magenta)
        ),
        Span::styled(
            format!("G: {:.2e} | ", g_val),
            Style::default().fg(Color::Green)
        ),
        Span::styled(
            format!("Trails: {} | Vectors: {} | Info: {} | ", 
                if app.show_trails { "ON" } else { "OFF" },
                if app.show_velocity { "ON" } else { "OFF" },
                if app.show_detailed_info { "ON" } else { "OFF" }
            ),
            Style::default().fg(Color::Yellow)
        ),
        Span::styled("[1-6]speed [b]algo [q]uit [i]nfo [space]pause [t]rails [v]ectors [+/-]zoom [←/→]focus [[]/]]G", Style::default().fg(Color::Cyan)),
    ]);
    let info_panel = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(info_panel, area);
}

pub fn get_body_color(index: usize) -> Color {
    match index {
        0 => Color::Yellow,
        1 => Color::Gray,
        2 => Color::LightYellow,
        3 => Color::Blue,
        4 => Color::Red,
        5 => Color::LightRed,
        6 => Color::LightMagenta,
        7 => Color::Cyan,
        8 => Color::Blue,
        _ => Color::White,
    }
} 