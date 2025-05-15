# Univers 🚀

**Unive.rs** — the ultimate terminal-based sandbox for simulating, breaking, and bending the laws of the universe.  
Built in Rust. Powered by [ratatui](https://github.com/ratatui-org/ratatui).

---

![preview](assets/pocket_solar_system.jpeg)

## ✨ Features

- **🌌 Interactive Solar System** — Realistic orbits, all major planets, and the Sun, rendered with emoji for maximum cosmic vibes.
- **🧲 Dynamic Gravity** — Change the gravitational constant on the fly. Make the universe collapse or float apart!
- **💥 Collisions & Particle Explosions** — Planets can smash into each other, exploding into clouds of particles that obey gravity.
- **🪐 Barnes-Hut Physics** — O(N log N) simulation for massive systems. Switch between brute-force and Barnes-Hut at runtime.
- **🎮 Full Terminal UI** — Zoom, pan, focus, toggle trails, see velocity vectors, and get detailed info on any body.
- **⚡ Real-Time & Time Warp** — Simulate from real seconds to cosmic years in a blink.
- **👾 Emoji Planets** — Every planet and the Sun has its own emoji. Particles are tiny dots.

---

## 🕹️ Controls

| Key         | Action                                      |
|-------------|---------------------------------------------|
| Space       | Pause/Resume simulation                     |
| 1-6         | Set simulation speed (1=slow, 6=cosmic)     |
| +/-         | Zoom in/out                                 |
| ←/→         | Change focus to prev/next body              |
| i           | Toggle detailed info panel                  |
| t           | Toggle orbit trails                         |
| v           | Toggle velocity vectors                     |
| r           | Reset simulation                            |
| c           | Clear trails                                |
| b           | Switch physics engine (Direct/Barnes-Hut)   |
| [ / ]       | Decrease/Increase gravity (G)               |
| q           | Quit                                        |

---

## 🧑‍🔬 Physics Engine

- **Newtonian Gravity** (Direct N², for small systems)
- **Barnes-Hut Algorithm** (O(N log N), for big chaos)
- **Realistic Collisions** — Planets have physical radii, collisions are based on actual sizes, not just positions.
- **Particle Explosions** — When planets collide, they explode into dozens of particles, each with its own trajectory and lifetime.

All math and physics are explained in [Mathematics.md](./Mathematics.md).

---

## 🚀 Quick Start

```bash
git clone https://github.com/Raumberg/univers.git
cd univers
cargo run --release
```

---

## 🛠️ Roadmap

- [x] Stable orbits for all planets
- [x] Realistic collisions and particle explosions
- [x] Barnes-Hut physics engine
- [x] Emoji rendering for all bodies
- [x] Dynamic gravity control
- [ ] Black holes and accretion disks
- [ ] Simplified General Relativity
- [ ] Save/load custom universes
- [ ] Mouse controls (drag & drop planets)
- [ ] More chaos!

---

## 📚 More

- For questions, ideas, or collabs — see my profile.
- PRs, issues, and cosmic ideas are welcome!  
  Feel free to fork, hack, and make the universe even weirder.

---

**Unive.rs — because the universe is too boring if you can't break it.**
