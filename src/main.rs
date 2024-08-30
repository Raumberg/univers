use nalgebra::{Point2, Vector2};

mod engine;
mod gen;
mod space;
mod term;

use crate::space::objects::CelestialObject;
use crate::gen::sim::Simulation;
use crate::term::widget;
use crate::term::prelude::*;

fn main() -> Result<(), io::Error>{
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let bodies = vec![
        CelestialObject::new(
            "Sun".to_string(),
            1.989e30,
            Point2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Point2::new(0.0, 0.0), // initial prevposition
        ),
        CelestialObject::new(
            "Earth".to_string(),
            5.972e24,
            Point2::new(149.596e9, 0.0),
            Vector2::new(0.0, 29.78e3),
            Vector2::new(0.0, 0.0),
            Point2::new(149.596e9, 0.0), // initial prevposition
        ),
        CelestialObject::new(
            "Mars".to_string(),
            6.419e23,
            Point2::new(227.939e9, 0.0),
            Vector2::new(0.0, 24.07e3),
            Vector2::new(0.0, 0.0),
            Point2::new(227.939e9, 0.0), // initial prevposition
        ),
    ];

    let theta = 0.5;
    let time_step = 0.1;
    let iterations = 1000;

    let mut simulation = Simulation::new(bodies, theta, time_step);
    simulation.run(iterations, false);

    // for body in &simulation.bodies {
    //     println!("{}: position = {:?}, velocity = {:?}", body.name, body.position, body.velocity);
    // }
    
    let void = widget::Void::new(simulation);

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)]);
            let rect = layout.split(size)[0];
            f.render_widget(void, rect);
        })?;
    }
}