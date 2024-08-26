import matplotlib.pyplot as plt
import matplotlib.animation as animation
import json

# Load the JSON data
with open('output.json', 'r') as f:
    data = json.load(f)

# Extract the positions, velocities, accelerations, and names from the JSON data
positions = [[[body['position']['x'], body['position']['y']] for body in time_step] for time_step in data]
velocities = [[[body['velocity']['x'], body['velocity']['y']] for body in time_step] for time_step in data]
accelerations = [[[body['acceleration']['x'], body['acceleration']['y']] for body in time_step] for time_step in data]
names = [body['name'] for body in data[0]]

print(positions)

# Create a figure and axis
fig, ax = plt.subplots()

# Set the axis limits to a large range to accommodate the positions
ax.set_xlim(-2e20, 2e20)
ax.set_ylim(-2e20, 2e20)

# Initialize the plot
scatters = [ax.scatter([], [], s=5, label=name) for name in names]
tracers = [ax.plot([], [], 'k-', alpha=0.5)[0] for _ in names]
arrows = []

# Add a legend
ax.legend()

# Define a color cycle for the planets
color_cycle = ['b', 'g', 'r', 'c', 'm', 'y', 'k', 'orange', 'purple', 'pink']

# Update function for the animation
def update(i):
    for arrow in arrows:
        arrow.remove()
    arrows.clear()
    for j, (p, v, a) in enumerate(zip(positions[i], velocities[i], accelerations[i])):
        scatters[j].set_offsets([p])
        arrow1 = ax.arrow(p[0], p[1], v[0]*0.1, v[1]*0.1, head_width=0.05, color=color_cycle[j % len(color_cycle)])
        arrow2 = ax.arrow(p[0], p[1], a[0]*0.01, a[1]*0.01, head_width=0.01, color=color_cycle[j % len(color_cycle)])
        arrows.extend([arrow1, arrow2])
        tracer_pos = [positions[k][j] for k in range(i+1)]
        tracers[j].set_data([x for x, y in tracer_pos], [y for x, y in tracer_pos])
    return *scatters, *tracers

# Create the animation
ani = animation.FuncAnimation(fig, update, frames=len(data), blit=True)

# Show the animation
plt.show()