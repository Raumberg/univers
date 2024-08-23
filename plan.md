Project Overview

Our 2D solar system simulator will consist of the following components:

    Core Logic: Written in Rust, this will handle the simulation of the solar system, including the positions, velocities, and interactions of the planets, sun, and other celestial bodies.
    Terminal Interface: Using Ratatui, we'll create a terminal-based interface to display the solar system, allowing users to interact with it.
    Scripting: We'll use Python or Lua to add scripting capabilities, enabling users to create custom scenarios, modify the simulation, or even create games within the solar system.

Core Logic (Rust)

To start, we'll focus on the core logic of the solar system simulation. We'll need to:

    Define the celestial bodies (planets, sun, moons, etc.) and their initial positions, velocities, and masses.
    Implement the physics engine to simulate the motion of the celestial bodies, taking into account gravity, orbital trajectories, and other relevant forces.
    Develop a time-stepping mechanism to advance the simulation in discrete time steps.

Terminal Interface (Ratatui)

Once we have the core logic in place, we'll create a terminal-based interface using Ratatui. This will involve:

    Designing a layout to display the solar system, including the positions and trajectories of the celestial bodies.
    Implementing user input handling to allow users to interact with the simulation (e.g., zooming, panning, selecting objects).
    Integrating the core logic with the terminal interface to update the display in response to user input and simulation advancements.

Scripting (Python or Lua)

To add scripting capabilities, we'll:

    Choose a scripting language (Python or Lua) and integrate it with the Rust core logic.
    Develop a scripting API to expose the solar system simulation's functionality to scripts.
    Create a way for users to write and execute scripts within the terminal interface.

# Improvenents!
Improvement 2: Implement orbital trajectories

    Task 2.1: Refactor the calculate_gravitational_force function to use a more accurate algorithm for calculating gravitational forces between celestial bodies.
    Task 2.2: Update the update_body function to use the new gravitational force calculation and update the positions and velocities of celestial bodies accordingly.
    Task 2.3: Implement a more accurate numerical integration method, such as the Verlet integration method, to simulate the motion of celestial bodies over time.

Improvement 3: Enhance the terminal interface

    Task 3.1: Add zooming functionality to the terminal interface, allowing users to zoom in and out of the solar system simulation.
    Task 3.2: Implement panning functionality, allowing users to move the view around the solar system simulation.
    Task 3.3: Add a selection mechanism, allowing users to select individual celestial bodies and view their properties (e.g., mass, velocity, position).

Improvement 4: Add scripting capabilities

    Task 4.1: Choose a scripting language (e.g., Python, Lua) and integrate it with the Rust code using a foreign function interface (FFI).
    Task 4.2: Expose the solar system simulation's functionality to the scripting language, allowing scripts to interact with the simulation.
    Task 4.3: Create a simple scripting API, allowing users to write scripts that can modify the simulation, create custom scenarios, or even create games within the solar system.
