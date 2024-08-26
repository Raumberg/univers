import json
import matplotlib.pyplot as plt
import matplotlib.animation as animation

# Load the JSON data
with open('simulation.json', 'r') as f:
    data = json.load(f)

# Extract the planet positions
planet_positions = {}
for frame in data:
    for planet in frame:
        name = planet['name']
        if name not in planet_positions:
            planet_positions[name] = {'x': [], 'y': []}
        planet_positions[name]['x'].append(planet['position']['x'])
        planet_positions[name]['y'].append(planet['position']['y'])

# Calculate the center of the solar system
center_x = 0
center_y = 0

# Create a figure and axis
fig, ax = plt.subplots(figsize=(8, 8))

# Initialize the axis limits
ax.set_xlim(-2e21, 2e21)
ax.set_ylim(-2e21, 2e21)

# Set the axis origin to the center of the solar system
ax.spines['left'].set_position('center')
ax.spines['right'].set_color('none')
ax.spines['bottom'].set_position('center')
ax.spines['top'].set_color('none')
ax.xaxis.set_ticks_position('bottom')
ax.yaxis.set_ticks_position('left')

# Create a scatter plot for each planet
planet_plots = []
for name in planet_positions:
    planet_plot, = ax.plot([], [], 'o', label=name, markersize=10)
    planet_plots.append(planet_plot)
print(f"Planet plots: {planet_plots}")

# Define the update function for the animation
def update(frame):
    for i, name in enumerate(planet_positions):
        planet_plots[i].set_data(planet_positions[name]['x'][frame] - center_x, planet_positions[name]['y'][frame] - center_y)
    return planet_plots

# Create the animation
ani = animation.FuncAnimation(fig, update, frames=len(data), blit=True, interval=50)

# Add a legend
ax.legend()

# Show the animation
plt.show()