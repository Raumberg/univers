use crate::term::prelude::*;

pub struct App {
    sender: mpsc::Sender<AppEvent>,
    is_running: bool,
    last_tick: Duration,
    aux_buffer: Rc<RefCell<Buffer>>,
    inspected_effect: u8,
    screen_area: Rect
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
            ui(f, &app, &mut effects)
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