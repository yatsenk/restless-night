# Restless Night

A first-person atmospheric autumn horror prototype. Walk through a procedurally generated suburban neighborhood under a cold moon, with Halloween-decorated yards, chunked world streaming, and physics.

Built with Rust, [Bevy](https://bevy.org) 0.16 and [Avian](https://github.com/Jondolf/avian) 0.3.

## Features

- Infinite procedural city streamed in 80 m chunks
- Late-October evening, around 19:00: dark sky, moonlight, street lamps, mostly unlit windows
- Detailed houses with porches, shutters, shingles and chimneys
- Car roads and pedestrian sidewalks separated by a grass strip
- Detailed ground: mottled lawn, grass tufts, fallen leaves, cracked asphalt, tiled sidewalks
- Yard clutter near houses (vines, rocks, twigs, mushrooms, leaf drifts) that disappears with distance
- Open yards with jack-o'-lanterns, gravestones, ghosts, hay bales, bare and leafy trees
- Level of detail with dithered cross-fade (no popping), lightweight ground and props at range, and merged primitive proxies for chunks out to about 300 m
- Volumetric fog with moonlight shafts, procedural moon with craters and stars, flashlight, falling leaves
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

## Configuration

Tunable constants live at the top of `src/main.rs`: mist density, haze range, moonlight and moon shadows and ambient level, lamp brightness, share of lit windows, chunk size, load and unload distances, LOD thresholds, player speed.

## Status

Early prototype. No interiors, enemies, or gameplay yet.
