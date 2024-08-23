Newton's Law of Universal Gravitation

The gravitational force between two objects is given by:

F = G * (m1 * m2) / r^2

where:

    F is the gravitational force between the two objects
    G is the gravitational constant (6.67430e-11 N*m^2/kg^2)
    m1 and m2 are the masses of the two objects
    r is the distance between the centers of the two objects

We can use this formula to calculate the gravitational force between each pair of celestial bodies in our solar system. Then, we can update their positions and velocities using the equations of motion.

Equations of Motion

For each celestial body, we need to update its position and velocity using the following equations:

    Acceleration: a = F / m
    Velocity: v(t + Δt) = v(t) + a * Δt
    Position: x(t + Δt) = x(t) + v(t + Δt) * Δt

where:

    a is the acceleration of the celestial body
    F is the net force acting on the celestial body (i.e., the sum of all gravitational forces from other bodies)
    m is the mass of the celestial body
    v is the velocity of the celestial body
    x is the position of the celestial body
    Δt is the time step (i.e., the interval between updates)

By iteratively applying these equations, we can simulate the motion of celestial bodies in our solar system.


### Verlet/Euler method
    pass

### Simplifying Einstein's General Relativity

Simplifying General Relativity (GR) for a 2D terminal universe simulation is a challenging task, but I can provide a rough outline of how you could approach it. Please note that this will be a highly simplified and approximate model, and it's not intended to be a rigorous or accurate representation of GR.

Simplifications and Assumptions:

    2D spacetime: We'll reduce the 4-dimensional spacetime to a 2-dimensional space, ignoring the time dimension.
    Flat spacetime: We'll assume the spacetime is flat, which means we can ignore the curvature of spacetime.
    Weak gravitational field: We'll assume the gravitational field is weak, which allows us to use a linear approximation.
    Point masses: We'll represent celestial bodies as point masses, ignoring their spatial extent.

Simplified GR Equations:

We'll use the linearized Einstein field equations to describe the gravitational field. In 2D, these equations can be written as:

∇²φ = 4πGρ

where:

    φ is the gravitational potential
    G is the gravitational constant
    ρ is the mass density
    ∇² is the 2D Laplace operator

Gravitational Potential:

We'll use the Newtonian gravitational potential as an approximation:

φ(r) = -G * m / r

where:

    m is the mass of the celestial body
    r is the distance from the body

Gravitational Force:

We'll use the Newtonian gravitational force as an approximation:

F(r) = -G * m1 * m2 / r²

where:

    m1 and m2 are the masses of the two celestial bodies
    r is the distance between them

Simulation:

Now, let's outline a simple simulation using these simplified GR equations:

    Initialize the system: Set up the initial positions, velocities, and masses of the celestial bodies in your 2D universe.
    Calculate the gravitational potential: For each body, calculate the gravitational potential φ at each point in the 2D space using the Newtonian potential formula.
    Calculate the gravitational force: For each pair of bodies, calculate the gravitational force F between them using the Newtonian force formula.
    Update the positions and velocities: Update the positions and velocities of each body using the calculated forces and the equations of motion.
