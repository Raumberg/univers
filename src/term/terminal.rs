use crate::term::prelude::*;
use std::io;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::space::system::StarSystem;
use crate::term::star_system::StarSystemApp;

pub struct App {
    sender: mpsc::Sender<AppEvent>,
    is_running: bool,
    last_tick: Duration,
    aux_buffer: Rc<RefCell<Buffer>>,
    inspected_effect: u8,
    screen_area: Rect,
    simulation_speed: SimulationSpeed,
    time_step: f64,
    time_elapsed: f64,
    scale: f64,
    focus_body_index: usize,
    show_info: bool,
    show_help: bool,
}

#[derive(Default)]
pub struct Fx {
    pub post_process: Option<Effect>,
}

impl Fx {
    pub fn push(&mut self, effect: Effect) {
        self.post_process = Some(effect);
    }

    pub fn process_active_fx(
        &mut self,
        duration: Duration,
        buffer: &mut Buffer,
        area: Rect
    ) {
        self.post_process.iter_mut().for_each(|effect| { effect.process(duration, buffer, area); });
        if self.post_process.iter().all(Effect::done) {
            self.post_process = None;
        }
    }
}

impl App {
    pub fn new(
        sender: mpsc::Sender<AppEvent>,
        aux_buffer_area: Rect,
    ) -> Self {
        Self {
            sender,
            is_running: true,
            last_tick: Duration::ZERO,
            aux_buffer: Rc::new(RefCell::new(Buffer::empty(aux_buffer_area))),
            screen_area: Rect::default(),
            inspected_effect: 0,
            simulation_speed: SimulationSpeed::Normal,
            time_step: 3600.0, // 1 hour in seconds
            time_elapsed: 0.0,
            scale: 1e-10, // Scale factor for display (AU to pixels)
            focus_body_index: 0, // Default focus on the Sun
            show_info: true,
            show_help: false,
        }
    }

    pub fn inspected_effect(&self, areas: EffectTimelineRects) -> Effect {
        effect_in(self.inspected_effect_no, areas)
    }

    pub fn effect_timeline(&self, areas: EffectTimelineRects) -> EffectTimeline {
        let idx = self.inspected_effect_no;
        let area = self.aux_buffer.borrow().area;
        let fx = transition_fx(area, self.sender.clone(), effect_in(idx, areas));

        EffectTimeline::builder()
            .effect(&fx)
            .build()
    }

    pub fn inspected_transition_effect(&self) -> Effect {
        let area = self.aux_buffer.borrow().area;
        let layout = self.effect_timeline(baseline_rects()).layout(area);
        transition_fx(area,  self.sender.clone(), self.inspected_effect(layout))
    }

    pub fn refresh_aux_buffer(&self) {
        let effect = self.inspected_transition_effect();

        let mut buf = self.aux_buffer.borrow_mut();
        Clear.render(buf.area, &mut buf);

        Block::new()
            .style(Style::default().bg(Color::Black))
            .render(buf.area, &mut buf);

        EffectTimeline::builder()
            .effect(&effect)
            .build()
            .render(buf.area, &mut buf);
    }

    pub fn apply_event(&mut self, effects: &mut Effects, e: AppEvent) {
        match e {
            AppEvent::Tick => (),
            AppEvent::KeyPressed(key) => match key {
                KeyCode::Esc => self.is_running = false,
                KeyCode::Char(' ') => {
                    // sends RefreshAufBuffer after transitioning out
                    effects.push(self.inspected_transition_effect())
                }
                KeyCode::Enter => {
                    self.inspected_effect_no = (self.inspected_effect_no + 1) % 3;
                    // sends RefreshAufBuffer after transitioning out
                    effects.push(self.inspected_transition_effect())
                },
                _ => (),
            },
            AppEvent::RefreshAufBuffer => {
                self.refresh_aux_buffer();
            },
            AppEvent::Resize(r) => self.screen_area = r
        }
    }

    pub fn run<B: Backend>(&self, terminal: &mut Terminal<B>, system: &mut StarSystem) -> io::Result<()> {
        let mut last_tick = std::time::Instant::now();
        let tick_rate = Duration::from_millis(33); // ~30 FPS

        let mut app = self.clone();

        while app.is_running {
            terminal.draw(|f| app.ui(f, system))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        app.handle_key_event(key.code, system);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.on_tick(system);
                last_tick = std::time::Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyCode, system: &mut StarSystem) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.is_running = false;
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
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.scale *= 1.1;
            }
            KeyCode::Char('-') => {
                self.scale /= 1.1;
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
            KeyCode::Char('r') => {
                // Reset the system to initial state
                *system = StarSystem::solar();
                self.time_elapsed = 0.0;
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
        };

        if steps > 0 {
            system.simulate(self.time_step, steps);
            self.time_elapsed += self.time_step * steps as f64;
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
                // Draw celestial bodies
                for (i, body) in system.bodies.iter().enumerate() {
                    let color = if i == 0 {
                        Color::Yellow // Sun
                    } else if i == 3 {
                        Color::Blue // Earth
                    } else if i == 1 {
                        Color::Gray // Mercury
                    } else if i == 2 {
                        Color::LightYellow // Venus
                    } else if i == 4 {
                        Color::Red // Mars
                    } else if i == 5 {
                        Color::LightRed // Jupiter
                    } else if i == 6 {
                        Color::LightMagenta // Saturn
                    } else if i == 7 {
                        Color::Cyan // Uranus
                    } else if i == 8 {
                        Color::Blue // Neptune
                    } else {
                        Color::White
                    };

                    // Size based on mass (log scale)
                    let size = (body.mass.log10() / 2.0).max(0.2);
                    
                    // Draw orbit path (except for Sun)
                    if i > 0 {
                        ctx.draw(&symbols::Marker::Circle, body.position.x, body.position.y, color);
                    } else {
                        // Sun is bigger
                        ctx.print(body.position.x, body.position.y, "☀", Style::default().fg(color));
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
                    Span::styled(format!("{:.2} days", self.time_elapsed / 86400.0), Style::default().fg(Color::White)),
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
                        },
                        Style::default().fg(match self.simulation_speed {
                            SimulationSpeed::Paused => Color::Red,
                            SimulationSpeed::Slow => Color::Yellow,
                            SimulationSpeed::Normal => Color::Green,
                            SimulationSpeed::Fast => Color::Cyan,
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
                    Span::styled("1, 2, 3", Style::default().fg(Color::Yellow)),
                    Span::raw(" - Set simulation speed (Slow, Normal, Fast)"),
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

impl Clone for App {
    fn clone(&self) -> Self {
        App {
            sender: self.sender.clone(),
            is_running: self.is_running,
            last_tick: self.last_tick,
            aux_buffer: self.aux_buffer.clone(),
            inspected_effect: self.inspected_effect,
            screen_area: self.screen_area,
            simulation_speed: self.simulation_speed,
            time_step: self.time_step,
            time_elapsed: self.time_elapsed,
            scale: self.scale,
            focus_body_index: self.focus_body_index,
            show_info: self.show_info,
            show_help: self.show_help,
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

// Terminal Interface structure used for interacting with the terminal
pub struct TerminalInterface {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalInterface {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(TerminalInterface { terminal })
    }

    pub fn run(&mut self, system: &mut StarSystem) -> io::Result<()> {
        let mut app = StarSystemApp::new();
        let res = app.run(&mut self.terminal, system);
        
        // Clean up
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        
        res
    }
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;

    // event handler
    let event_handler = EventHandler::new(Duration::from_millis(33));
    let sender = event_handler.sender();

    // create app and run it
    let app = App::new(sender, Rect::new(0, 0, 100, 40));
    let res = run_app(&mut terminal, app, event_handler);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub type AuxBuffer = Rc<RefCell<Buffer>>;

fn run_app(
    terminal: &mut Terminal,
    mut app: App,
    event_handler: EventHandler,
) -> io::Result<()> {
    let mut last_frame_instant = std::time::Instant::now();

    let mut effects = Effects::default();
    effects.push(app.inspected_effect(baseline_rects()));
    app.refresh_aux_buffer();

    while app.is_running {
        event_handler.receive_events(|e| app.apply_event(&mut effects, e));

        app.last_tick = last_frame_instant.elapsed();
        last_frame_instant = std::time::Instant::now();
        terminal.draw(|f| {
            app.screen_area = f.area();
            app.ui(f, &mut StarSystem::solar())
        })?;
    }

    Ok(())
}

fn  ui(
    f: &mut Frame,
    app: &App,
    effects: &mut Effects
) {
    let rect = f.area();
    if rect.area() == 0 { return; }

    let buf: &mut Buffer = f.buffer_mut();
    Clear.render(rect, buf);

    let shortcut_key_style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD);
    let shortcut_label_style = Style::default()
        .fg(Color::DarkGray);

    app.aux_buffer.render_buffer(Offset::default(), buf);
    effects.process_active_fx(app.last_tick, buf, rect);

    let shortcuts = Line::from(vec![
        Span::from("ENTER ").style(shortcut_key_style),
        Span::from("next transition ").style(shortcut_label_style),
        Span::from(" SPACE ").style(shortcut_key_style),
        Span::from("replay transition ").style(shortcut_label_style),
        Span::from(" ESC ").style(shortcut_key_style),
        Span::from("quit").style(shortcut_label_style),
    ]);

    let centered = Rect {
        x: rect.x + (rect.width - shortcuts.width() as u16) / 2,
        y: rect.y + rect.height - 1,
        width: shortcuts.width() as u16,
        height: 1,
    };
    shortcuts.render(centered, buf);
}


fn setup_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );

        panic_hook(panic);
    }));

    Ok(terminal)
}

enum AppEvent {
    Tick,
    KeyPressed(KeyCode),
    Resize(Rect),
    RefreshAufBuffer,
}

pub struct EventHandler {
    sender: mpsc::Sender<AppEvent>,
    receiver: mpsc::Receiver<AppEvent>,
    _handler: thread::JoinHandle<()>
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();

        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for events") {
                        Self::apply_event(&sender);
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(AppEvent::Tick)
                            .expect("failed to send tick event");

                        last_tick = std::time::Instant::now();
                    }
                }
            })
        };

        Self { sender, receiver, _handler: handler }
    }

    pub(crate) fn sender(&self) -> mpsc::Sender<AppEvent> {
        self.sender.clone()
    }

    fn next(&self) -> std::result::Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }

    fn try_next(&self) -> Option<AppEvent> {
        match self.receiver.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None
        }
    }

    pub(crate) fn receive_events<F>(&self, mut f: F)
        where F: FnMut(AppEvent)
    {
        f(self.next().unwrap());
        while let Some(event) = self.try_next() { f(event) }
    }

    fn apply_event(sender: &mpsc::Sender<AppEvent>) {
        match event::read().expect("unable to read event") {
            Event::Key(e) if e.kind == KeyEventKind::Press =>
                sender.send(AppEvent::KeyPressed(e.code)),
            Event::Resize(w, h) =>
                sender.send(AppEvent::Resize(Rect::new(0, 0, w, h))),
            _ => Ok(())
        }.expect("failed to send event")
    }
}

fn baseline_rects() -> EffectTimelineRects {
    // giving an approximate layout so that all rects resolve to unique values,
    // enabling us to get the actual layout from the effect timeline. something
    // of a hack...
    EffectTimelineRects {
        tree: Rect::new(0, 0, 25, 32),
        chart: Rect::new(35, 0, 65, 32),
        cell_filter: Rect::new(25, 0, 6, 32),
        areas: Rect::new(31, 0, 4, 32),
        legend: Rect::new(35, 34, 29, 6),
        cell_filter_legend: Rect::new(35, 34, 9, 2),
        areas_legend: Rect::new(48, 34, 16, 2),
    }
}