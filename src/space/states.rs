use crate::space::objects::StarSystem;

pub struct SystemState {
    system: StarSystem,
    core: Star,
    planets: Vec<Planet>,
}

impl Default for SystemState {
    fn default() -> Self {
        let mut system: StarSystem;
        let core: Star;
    }

    Self {
        system,
        core,
        planets: Vec::new(),
    }
}