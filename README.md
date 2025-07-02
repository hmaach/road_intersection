# Road Intersection Simulation

This is a small simulation of a road intersection I made using Rust and SDL2.

## Features

- Cars come from four directions: Top, Bottom, Left, Right.
- Traffic lights control the movement.
- Cars stop at red lights and move when it's green.
- You can manually add cars using arrow keys or randomly with the `R` key.

## Controls

- `↑` Add car from Bottom  
- `↓` Add car from Top  
- `←` Add car from Right  
- `→` Add car from Left  
- `R` Add a random car from any direction  
- `Esc` Quit the simulation

## Dependencies

- Rust
- SDL2
- `sdl2` crate
- `rand` crate

## Notes

- Cooldowns prevent cars from being spawned too fast.
- Cars avoid crashing and wait their turn at intersections.

That's it. Just a fun experiment with traffic logic and graphics in Rust :)
