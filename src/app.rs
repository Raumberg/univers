use std::collections::VecDeque;
use crossterm::event::KeyCode;
use crate::simulation::SimulationState;

const MAX_TRAIL_LENGTH: usize = 200;

pub struct App {
    pub simulation: std::sync::Arc<std::sync::Mutex<SimulationState>>,
    pub scale: f64,
    pub focus_body_index: usize,
    pub show_trails: bool,
    pub show_velocity: bool,
    pub show_detailed_info: bool,
    pub trails: Vec<VecDeque<(f64, f64)>>,
}

impl App {
    pub fn new(simulation: std::sync::Arc<std::sync::Mutex<SimulationState>>) -> Self {
        let num_bodies = simulation.lock().unwrap().system.bodies.len();
        let mut trails = Vec::with_capacity(num_bodies);
        for _ in 0..num_bodies {
            trails.push(VecDeque::with_capacity(MAX_TRAIL_LENGTH));
        }
        App {
            simulation,
            scale: 1e-10,
            focus_body_index: 0,
            show_trails: true,
            show_velocity: true,
            show_detailed_info: false,
            trails,
        }
    }

    pub fn update_trails(&mut self) {
        for (i, body) in self.simulation.lock().unwrap().system.bodies.iter().enumerate() {
            if i < self.trails.len() {
                self.trails[i].push_back((body.position.x, body.position.y));
                if self.trails[i].len() > MAX_TRAIL_LENGTH {
                    self.trails[i].pop_front();
                }
            }
        }
    }

    pub fn on_tick(&mut self) {
        self.update_trails();
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {},
            KeyCode::Char(' ') => {
                let mut sim = self.simulation.lock().unwrap();
                if sim.running {
                    sim.pause();
                } else {
                    sim.resume();
                }
            }
            KeyCode::Char('i') => {
                self.show_detailed_info = !self.show_detailed_info;
            }
            KeyCode::Char('1') => {
                self.simulation.lock().unwrap().set_speed(1);
            }
            KeyCode::Char('2') => {
                self.simulation.lock().unwrap().set_speed(5);
            }
            KeyCode::Char('3') => {
                self.simulation.lock().unwrap().set_speed(20);
            }
            KeyCode::Char('4') => {
                self.simulation.lock().unwrap().set_speed(100);
            }
            KeyCode::Char('5') => {
                self.simulation.lock().unwrap().set_speed(500);
            }
            KeyCode::Char('6') => {
                self.simulation.lock().unwrap().set_speed(1000);
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.scale *= 1.5;
            }
            KeyCode::Char('-') => {
                self.scale /= 1.5;
            }
            KeyCode::Right => {
                self.focus_body_index = (self.focus_body_index + 1) % self.simulation.lock().unwrap().system.bodies.len();
            }
            KeyCode::Left => {
                if self.focus_body_index > 0 {
                    self.focus_body_index -= 1;
                } else {
                    self.focus_body_index = self.simulation.lock().unwrap().system.bodies.len() - 1;
                }
            }
            KeyCode::Char('t') => {
                self.show_trails = !self.show_trails;
            }
            KeyCode::Char('v') => {
                self.show_velocity = !self.show_velocity;
            }
            KeyCode::Char('r') => {
                self.simulation.lock().unwrap().reset();
                for trail in &mut self.trails {
                    trail.clear();
                }
            }
            KeyCode::Char('c') => {
                for trail in &mut self.trails {
                    trail.clear();
                }
                self.update_trails();
            }
            KeyCode::Char('b') => {
                let mut sim = self.simulation.lock().unwrap();
                use crate::space::system::SimulationAlgorithm;
                let new_algo = match sim.algorithm {
                    SimulationAlgorithm::Direct => SimulationAlgorithm::BarnesHut,
                    SimulationAlgorithm::BarnesHut => SimulationAlgorithm::Direct,
                };
                sim.set_algorithm(new_algo);
            }
            KeyCode::Char('[') => {
                let mut sim = self.simulation.lock().unwrap();
                sim.system.g *= 0.9;
            }
            KeyCode::Char(']') => {
                let mut sim = self.simulation.lock().unwrap();
                sim.system.g *= 1.1;
            }
            _ => {}
        }
    }

    pub fn format_time_elapsed(&self) -> String {
        let seconds_per_day = 86400.0;
        let days_per_month = 30.44;
        let days_per_year = 365.25;
        let days = self.simulation.lock().unwrap().time_elapsed / seconds_per_day;
        match days {
            d if d < 100.0 => format!("{:.2} days", d),
            d if d < 1000.0 => {
                let months = (d / days_per_month).floor();
                let remaining_days = d % days_per_month;
                format!("{:.0} months, {:.1} days", months, remaining_days)
            },
            d if d > 100.0 * days_per_year => format!("{:.1} years", days / days_per_year),
            _ => {
                let years = (days / days_per_year).floor();
                let remaining_days = days % days_per_year;
                let months = (remaining_days / days_per_month).floor();
                let last_days = remaining_days % days_per_month;
                format!("{:.0} years, {:.0} months, {:.1} days", years, months, last_days)
            }
        }
    }
} 