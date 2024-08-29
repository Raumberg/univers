# Mathematical statement of the following codebase


## Newton's Law of Universal Gravitation

**The gravitational force between two objects is given by the following equation:**
```
F = G * (m1 * m2) / r^2
```
where:
```
F is the gravitational force between the two objects
G is the gravitational constant (6.67430e-11 N*m^2/kg^2)
m1 and m2 are the masses of the two objects
r is the distance between the centers of the two objects
This equation is a fundamental principle in astrophysics and is used to calculate the gravitational force between celestial bodies in our solar system. However, when dealing with a large number of bodies, the computational complexity of calculating the forces between all pairs of bodies becomes a significant challenge.
```
### Equations of Motion

**To simulate the motion of celestial bodies, we need to update their positions and velocities using the following equations:**
```
Acceleration: a = F / m
Velocity: v(t + Δt) = v(t) + a * Δt
Position: x(t + Δt) = x(t) + v(t + Δt) * Δt
```
where:
```
a is the acceleration of the celestial body
F is the net force acting on the celestial body (i.e., the sum of all gravitational forces from other bodies)
m is the mass of the celestial body
v is the velocity of the celestial body
x is the position of the celestial body
Δt is the time step (i.e., the interval between updates)
These equations are based on the fundamental principles of classical mechanics and are widely used in astrophysical simulations.
```
### Verlet/Euler Method

**The Verlet/Euler method is a numerical integration technique used to solve the equations of motion.**  
It is a simple and efficient method that is widely used in astrophysical simulations. The method involves the following steps:

- Initialize the positions and velocities of the celestial bodies.
- Calculate the accelerations of the celestial bodies using the gravitational forces.
- Update the velocities of the celestial bodies using the accelerations.
- Update the positions of the celestial bodies using the velocities.
- The Verlet/Euler method is a first-order method, which means that it has a linear accuracy in time. However, it is a simple and efficient method that is widely used in astrophysical simulations.

## Simplifying Einstein's General Relativity

**Simplifying General Relativity (GR) for a 2D terminal universe simulation is a challenging task. However, we can make several simplifications and assumptions to reduce the complexity of the problem. These simplifications and assumptions include:**

- 2D spacetime: We reduce the 4-dimensional spacetime to a 2-dimensional space, ignoring the time dimension.
- Flat spacetime: We assume the spacetime is flat, which means we can ignore the curvature of spacetime.
- Weak gravitational field: We assume the gravitational field is weak, which allows us to use a linear approximation.
- Point masses: We represent celestial bodies as point masses, ignoring their spatial extent.
- Using these simplifications and assumptions, we can derive the simplified GR equations. In 2D, these equations can be written as:
```
∇²φ = 4πGρ
```
where:
```
φ is the gravitational potential
G is the gravitational constant
ρ is the mass density
∇² is the 2D Laplace operator
We can use the Newtonian gravitational potential as an approximation:  

φ(r) = -G * m / r
```
where:
```
m is the mass of the celestial body
r is the distance from the body
We can also use the Newtonian gravitational force as an approximation:  

F(r) = -G * m1 * m2 / r²
```
where:
```
m1 and m2 are the masses of the two celestial bodies
r is the distance between them
```
### Barnes-Hut Algorithm

**The Barnes-Hut algorithm is a hierarchical algorithm used to calculate the forces between celestial bodies.**  
The algorithm reduces the computational complexity of calculating forces between all pairs of bodies from **O(n^2) to O(n log n).** The algorithm involves the following steps:

- Divide the space into a hierarchical grid of cells.
- Calculate the center of mass and total mass of each cell.
- Calculate the gravitational force between each pair of cells.
- Use the gravitational force to update the positions and velocities of the celestial bodies.  
The Barnes-Hut algorithm is a widely used algorithm in astrophysical simulations. It is an efficient and accurate method for calculating the forces between celestial bodies.

# Simulation

**To simulate the motion of celestial bodies, we can use the following steps:**

- **Initialize the system:** Set up the initial positions, velocities, and masses of the celestial bodies in 2D universe.  
- **Calculate the gravitational potential:** For each body, calculate the gravitational potential φ at each point in the 2D space using the Newtonian potential formula.
- **Calculate the gravitational force:** For each pair of bodies, calculate the gravitational force F between them using the Newtonian force formula.
- **Update the positions and velocities:** Update the positions and velocities of each body using the calculated forces and the equations of motion.  
*Repeat steps 2-4: Continue iterating the simulation until the desired time step is reached. ^_^*

## Accuracy and Approximations

The accuracy of the simulation depends on several factors, including the accuracy of the gravitational potential and force calculations, the time step Δt, and the numerical integration method used. To improve the accuracy of the simulation, we can use more sophisticated numerical methods, such as the Runge-Kutta method, or more accurate approximations of the gravitational potential and force.  

*theta* - Accuracy of Force Calculations  

The accuracy of the force calculations can be controlled by the parameter θ, which determines the accuracy of the Barnes-Hut algorithm. A smaller value of θ results in a more accurate calculation of the forces, but at the cost of increased computational time.
