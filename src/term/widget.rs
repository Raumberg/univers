use crate::term::prelude::*;
use ratatui::prelude::Rect;
use ratatui::widgets::Widget;

use crate::gen::sim::Simulation;

pub struct Void {
    pub simulation: Simulation
}

impl Void {
    fn new() -> Self {
        Void { simulation }
    }
}

impl<B: ratatui::backend::Backend> Widget for Void {
    fn render(&mut self, f: &mut ratatui::Frame<B>, rect: Rect) {
        let block = Block::default()
            .title("Void")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        f.render_widget(block, rect);

        let mut bodies = self.simulation.bodies.clone();

        for body in bodies.iter() {
            // Draw a dot for each body
            let x = (body.position.x + 1000.0) as u16; // scale and offset x coordinate
            let y = (body.position.y + 1000.0) as u16; // scale and offset y coordinate
            f.set_cursor(x, y);
            f.print("·"); // print a dot
        }
    }
}