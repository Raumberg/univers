import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

def gravitational_force(m1, m2, r):
    return -G * m1 * m2 / r**2

def update_position(body1, body2, dt):
    r = np.sqrt((body1['x'] - body2['x'])**2 + (body1['y'] - body2['y'])**2)
    phi = np.arctan2(body2['y'] - body1['y'], body2['x'] - body1['x'])
    F = gravitational_force(body1['m'], body2['m'], r)
    Fx = F * np.cos(phi)
    Fy = F * np.sin(phi)
    ax = Fx / body1['m']
    ay = Fy / body1['m']
    body1['vx'] += ax * dt
    body1['vy'] += ay * dt
    body1['x'] += body1['vx'] * dt
    body1['y'] += body1['vy'] * dt

def calculate_gravitational_potential(body1, body2):
    r = np.sqrt((body1['x'] - body2['x'])**2 + (body1['y'] - body2['y'])**2)
    return -G * body1['m'] * body2['m'] / r

def simulate_solar_system(bodies, dt, t_total):
    t = 0
    positions = []
    while t < t_total:
        positions.append([body['x'] for body in bodies] + [body['y'] for body in bodies])
        for i, body1 in enumerate(bodies):
            for j, body2 in enumerate(bodies):
                if i != j:
                    update_position(body1, body2, dt)
        for body in bodies:
            print(f"Body {body['name']}: x={body['x']:.2e}, y={body['y']:.2e}, vx={body['vx']:.2e}, vy={body['vy']:.2e}")
        for i, body1 in enumerate(bodies):
            for j, body2 in enumerate(bodies):
                if i != j:
                    print(f"Gravitational potential between {body1['name']} and {body2['name']}: {calculate_gravitational_potential(body1, body2):.2e}")
        print()
        t += dt
    return np.array(positions)

G = 6.674e-11  # gravitational constant
dt = 1e5  # time step
t_total = 3.15e7  # total time

# Initialize the system
np.random.seed(0)  # for reproducibility
n_bodies = 4
bodies = []
for i in range(n_bodies):
    angle = np.random.uniform(0, 2*np.pi)
    radius = np.random.uniform(1e10, 1e11)
    x = radius * np.cos(angle)
    y = radius * np.sin(angle)
    vx = -np.sin(angle) * 2e4  # set initial velocity to be tangential to the orbit
    vy = np.cos(angle) * 2e4  # set initial velocity to be tangential to the orbit
    m = np.random.uniform(1e24, 1e25)
    bodies.append({'x': x, 'y': y, 'vx': vx, 'vy': vy, 'm': m, 'name': f'Body {i+1}'})

positions = simulate_solar_system(bodies, dt, t_total)

fig, ax = plt.subplots()

lines = [ax.plot([], [], 'o-')[0] for _ in range(n_bodies)]

ax.set_xlim(-2e11, 2e11)
ax.set_ylim(-2e11, 2e11)

def update(i):
    for j, line in enumerate(lines):
        line.set_data(positions[:i, j*2], positions[:i, j*2+1])
    return lines

ani = animation.FuncAnimation(fig, update, frames=len(positions), blit=True)

plt.show()