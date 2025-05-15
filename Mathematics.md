# 🚀 The Mathematics of Univers: Physics for the Reckless

> "If you can't break the universe, what's the point of simulating it?"

---

## 🌌 Newton's Law of Universal Gravitation (a.k.a. Why Shit Falls)

```
      m1      m2
      |        |
      *--------*
         r

    F = G * (m1 * m2) / r^2
```
- **F**: Gravitational force (the cosmic glue)
- **G**: Gravitational constant (tweak it in the sim, break everything)
- **m1, m2**: Masses of the bodies (Sun, planet, particle, whatever)
- **r**: Distance between centers (not edges, you cheater)

**In Univers, you can crank G up or down. Want a black hole? Go nuts. Want planets to drift like lost socks? Lower G.**

---

## 🏃‍♂️ Equations of Motion (How Planets Actually Move)

We update every body's position and velocity every tick:

```
    a = F / m
    v(t+dt) = v(t) + a * dt
    x(t+dt) = x(t) + v(t+dt) * dt
```
- **a**: Acceleration (from all those F's)
- **v**: Velocity
- **x**: Position
- **dt**: Time step (set by sim speed)

**Pro tip:** If you make dt too big, planets will yeet themselves into the void. That's not a bug, that's physics (sort of).

---

## 🧮 Integration: Euler/Verlet (a.k.a. Good Enough™)

We use a simple Euler/Verlet method. Why? Because it's fast, and you want chaos, not a PhD.

- Calculate all forces
- Update velocities
- Update positions
- Repeat until the heat death of your terminal

**Want more accuracy?**
- Use a smaller dt
- Or implement Runge-Kutta yourself (but why?)

---

## 💥 Collisions & Explosions (Because Space is Violent)

Planets have radii (calculated from mass & density). If two overlap:
- They explode into a cloud of particles
- Particles are real bodies: they feel gravity, they die after a while
- Yes, you can make Saturn go supernova. Try it.

---

## 🪐 Barnes-Hut Algorithm (a.k.a. Fast N-Body for Lazy Bastards)

Simulating every pair is O(N²). That's slow as hell. Barnes-Hut is O(N log N):

```
   +---+---+
   |   |   |
   +---+---+
   |   |   |
   +---+---+
```
- Divide space into cells
- Approximate distant cells as one big mass
- Only do full calc for close neighbors
- Result: 10,000 bodies? No problem. (Well, almost)

**Switch between direct and Barnes-Hut in real time. See what breaks.**

---

## 🕹️ Hacking the Physics (a.k.a. Make It Weird)

- **Crank G up**: Everything collapses. Black hole party.
- **Crank G down**: Planets drift apart. Solar system becomes a disco.
- **Add a massive body**: Watch orbits go nuts.
- **Spam particles**: Lag your terminal, crash your OS, blame Rust.
- **Set dt to 9000**: Instant entropy.

---

## ⚠️ How to Break the Simulation

- Set G to something stupid (like 1e10). Enjoy the fireworks.
- Add two Suns. See who wins.
- Make all planets the same mass. Chaos.
- Set dt to 0.00001. Watch nothing happen. Set dt to 1. Watch everything die.
- Try to simulate Pluto. It will still get kicked out.

---

## 🧑‍🔬 FAQ

**Q: Why not use Runge-Kutta or symplectic integrators?**
A: Because you want to see planets explode, not write a thesis.

**Q: Why 2D?**
A: Because 3D in a terminal is for masochists.

**Q: Can I add black holes?**
A: Not yet. But you can fake it by cranking G and adding a massive body.

---

## 🤘 Final Words

This isn't a physics textbook. This is a playground. Break stuff. Make new laws. If you want more math, PRs welcome. If you want more chaos, just ask.

---

*Univers: Because the universe is too boring if you can't break it.*
