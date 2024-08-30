use crate::space::types::*;

/// The train `Sim` is applied to an objects that needs to be simulated
pub trait Sim {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Position; // Point2<T>
    fn get_velocity(&self) -> Velocity; // Vector2<T>
    fn get_mass(&self) -> f64;
    fn get_ratius(&self) -> f64;
    fn simulate(&mut self)
}

/// The trait `Orbitable` is applied to the objects that can be orbited
/// by other objects.
pub trait Orbitable {
    /// `Satellite` stores the type of object's satellites
    type Satellite: Orbit;
    
    /// Returns all the satellites that orbit the object
    /// 
    /// # Arguments
    /// * `self` - A reference to the object
    /// 
    /// # Returns
    /// * `Vec<Self::Satellite>` - A vector containing all the 
    /// satellites that orbit the object
    fn get_satellites(&self) -> Vec<Self::Satellite>;

    fn update_orbits(&mut self);
}

pub trait Orbit {
    type Host: Orbitable;
    fn get_orbit_radius(&self) -> f32;
    fn get_orbit_period(&self) -> f32;
    /// Returns the position in orbit in radians [0; 2pi], counting from the rightmost point
    fn get_orbit_position(&self) -> f32;
    fn get_angular_speed(&self) -> f32;
    fn update_orbit_position(&mut self);
}

pub trait Displayable {
    fn get_name(&self) -> String;
    fn get_properties(&self) -> Vec<Vec<String>> { Vec::new() }
    fn get_menu_color(&self) -> ratatui::style::Color { ratatui::style::Color::White }
}