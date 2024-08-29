# Univers
=====================================================  

Unive.rs is a CLI tool to become a god and simulate your own star system. Built entirely in Rust, empowered by ratatui interface and GPU-accelerated linear algebra computations.
## Gravity engine
Gravity can be simulated in two ways: 
- **Newton's Law of Universal Gravitation**
- **Simplified Einstein's General Relativity.**
  
All mathematical approaches are stated in ./Mathematics.md.  
There are also several sidesteps for reducing computation complexity which are also stated there


### Status:
* **Ongoing** 🌊
* **Master Branch:** build successful 🚀

### Plans:
* 🤯 -> **Dynamic Gravity:** Manipulating gravity based on the CPU usage. If your CPU usage is high, the system will collapse and re-rendered again 💥
* 💥 -> **Collisions**
* 🕳️ -> **Black Holes**

### TODO List:
- [x] Implement codebase for celestial bodies and system
- [x] Implement a base physics engine
- [ ] Implement a ratatui interactable interface
- [ ] Enchanse physics with Simplified GR
- [ ] Implement collisions
- [ ] ... and many more!

### Build and run
*Want to build and run Univers yourself? Here's how:*
```bash
git clone <repo>
cd <repo>
cargo build --release && cargo run
# using python visualization to check the gravity engine:
python observe.py
```

Feel free to contribute and build the universe with me! My contacts are in my profile.
