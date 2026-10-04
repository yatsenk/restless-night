# Restless Night

A first-person atmospheric autumn horror prototype. Walk through a procedurally generated suburban neighborhood buried in cold fog, with Halloween-decorated yards, chunked world streaming, and physics.

Built with Rust, [Bevy](https://bevy.org) 0.16 and [Avian](https://github.com/Jondolf/avian) 0.3.

## Features

- Infinite procedural city streamed in 80 m chunks
- Detailed houses with porches, shutters, shingles and chimneys
- Car roads and pedestrian sidewalks separated by a grass strip
- Fenced yards with jack-o'-lanterns, gravestones, ghosts, hay bales and dead trees
- Level of detail: distant objects switch to simplified meshes
- Thick fog, flashlight, falling leaves
- Physics: collisions with the world, kickable pumpkins

## Getting started

Requires Rust 1.85 or newer.

```
cargo run
```

## Controls

| Key | Action |
|-----|--------|
| W A S D | Move |
| Left Shift | Run |
| Mouse | Look |
| F | Toggle flashlight |
| Esc / Left click | Release / capture cursor |

## Project layout

```
src/
  main.rs    app setup, chunk streaming, LOD, player
  houses.rs  procedural house meshes
  props.rs   yard and street props
  mesh.rs    mesh builder
  util.rs    RNG and color helpers
```

## Configuration

Tunable constants live at the top of `src/main.rs`: fog distance, chunk size, load and unload distances, LOD thresholds, player speed.

## Status

Early prototype. No interiors, enemies, or gameplay yet.
