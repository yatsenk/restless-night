# Restless Night — suburban house (design document)

Coordinates: metres. X runs along the street (west to east, 0 = garage west wall inner face), Z runs from the street into the lot (Z = 0 is the front wall of the main block, the street is at negative Z, the rear yard at positive Z), Y is up. Ground-floor slab Y = 0, upper-floor slab Y = 2.8, ground outside Y = -0.36, basement slab Y = -2.28. The house faces the street at -Z. Player spawn: (5.6, 0, 3.0) in the hall, looking at the stairs/back of the house.

## 1. Concept

A two-storey, ~1985 American builder house on a narrow residential lane: front-gable single-car garage pushed forward to the west, a recessed covered porch with three steps in the middle, a long side-gabled main roof behind it. Grey-green vinyl lap siding, white trim, dark-grey asphalt shingles, concrete foundation band, concrete driveway. Late-October overcast afternoon, damp lawn with leaves, fog swallowing everything beyond ~30 m. Inside it is lived-in and ordinary (empty slots for the user's furniture, warm lamps in a few rooms, cool daylight from the windows). Nothing in the geometry tells a story; the unease comes from scale, silence, long sightlines and the cold light.

Facade is asymmetric: garage bay (x -0.3..3.8, projects 1.0 m forward of the porch wall), porch (x 3.66..6.8), living/dining wall (x 6.8..13.0) with a wide picture window and a smaller one, three upper windows over the bedrooms.

## 2. Areas and dimensions

| Space | Size (m) | Area m² |
|---|---|---|
| Hall + stair | 3.14 x 7.0 (z 1.8..8.8) | 22.0 |
| Living room | 5.74 x 4.8 | 27.6 |
| Dining | 5.74 x 1.64 | 9.4 |
| Kitchen (open to dining) | 3.54 x 2.2 | 7.8 |
| Ground bathroom | 2.04 x 2.04 | 4.2 |
| Laundry/utility | 3.5 x 3.1 | 10.9 |
| **Main floor (excluding garage)** | | **≈ 82** |
| Garage | 3.5 x 6.4 (z -1.0..5.4) | 22.4 |
| Upper hall + landing | 2.04 x 8.8 (with stair well) | 18.0 |
| Bedroom 2 | 3.14 x 3.5 | 11.0 |
| Bedroom 3 / study | 3.54 x 3.5 | 12.4 |
| Upper corridor | 7.0 x 1.1 | 7.7 |
| Upstairs bathroom | 2.14 x 2.08 | 4.5 |
| Walk-in closet (off main bedroom) | 2.14 x 1.64 | 3.5 |
| Main bedroom | 4.54 x 3.88 | 17.6 |
| **Upper floor** | | **≈ 75** |
| **Living area total** | | **≈ 157** |
| Basement (under garage, utility) | 1.0..3.5 x 0..5.4 | ≈ 15 |

Ceilings: ground 2.5 m (floor to ceiling), upper 2.4 m (slab top 2.8 to ceiling 5.2). Entrance door 0.9 m wide x 2.2 m; interior doors 0.82 m clear, 2.15 m high; cased openings 1.0–2.4 m; garage→utility door 0.82 m; stairs 0.9 m wide, 16 risers of 0.175 m, treads 0.26 m, rail 0.92 m; basement stairs 12 risers 0.19 m, tread 0.25 m. Windows: sill 0.9 m, head 2.1 m. Garage door 2.5 m wide x 2.15 m.

## 3. Ground floor plan (not to scale; numbers are X / Z)

```
 Z
 -1.0 ┌──────────┐ garage door (0.5..3.0)       porch 3.66..6.8
      │  GARAGE  │──┐  ┌─ ─ ─ ─ ─ ┐
  1.8 │ 3.5x6.4  │  │  │ entry    │ HALL 3.14 wide ─ 6.8   LIVING x 6.96..12.7
      │ (car)    │  │S │ door     │ cased opening z4.0  ┌──────────────────────┐
  2.3 │          │  │T │ 4.75..   │                      │ z 0..4.8  27.6 m²   │
      │ stair    │  │A │ 5.65     │                      │ sofa, TV, lamp      │
  4.3 │ down     │  │I │          │                      └───────cased x 10.0───┘
      │ (x0..1)  │  │R │          │                      ┌──────────────────────┐
  5.4 ├────door──┤  │0.9│         │                      │ DINING z 4.96..6.6   │
  5.7 │ UTILITY  │  │  │          │  bath door z7.66     ├────────┬─────────────┤
      │ 3.5x3.1  │  │  │          │  ┌─BATH──┐ x6.96..9.0│        │ KITCHEN     │
  8.8 └──────────┴──┴──┴──────────┴──└───────┘───────────┴────────┴─────────────┘
     x0         3.5 3.66          6.8        9.0         9.16               12.7
```

Walls: exterior 0.30 m, interior partitions 0.16 m. Open plan: hall→living (cased opening 1.2 m at z 4.0), living→dining (2.4 m cased opening at x 10.0), dining open to kitchen. Real doors on the ground floor: bathroom (z 7.66 off the hall), garage→utility (x 2.9), basement-stair door in the garage (z 4.86), utility→rear (ext door x 2.4..3.3), entrance. Utility connects to the hall through a cased opening in the party wall (z 7.2).

## 4. Upper floor plan

```
 Z
  0.0 ┌───────┬──────────┬───────────┐
      │ HALL  │ BEDROOM 2│ BEDROOM 3 │  windows to the street
      │ (stair│ 5.86..9.0│ 9.16..12.7│
      │ well  │ door z7.0│ door x10.9│
  3.5 │ 2.69..│          │           │
      │ 6.2)  ├──door────┴───door────┤
  3.66│       │ CORRIDOR 1.1 m       │
  4.76│       ├────────┬──door 9.9───┤
      │       │ BATH 2 │ MAIN BEDROOM│  window east (z 6.5), window back
  5.78│       │door    │ 8.16..12.7  │
  7.08│       ├────────┤ door 7.9 to │
      │LANDING│ CLOSET │ closet      │
  8.8 └───────┴────────┴─────────────┘
      3.66   5.7       8.16          12.7
```

The stair arrives at the back (z 6.2) and the hall wraps forward along the party wall to the front window (z 0). The landing looks across the corridor into the back window.

## 5. Elevations (text)

Front (from the street, x increases to the right):
- x -0.3..3.8: garage, front gable roof (pitch 0.40) with siding in the gable triangle, sectional door 2.5 x 2.15, driveway sloping to the lane.
- x 3.66..6.8: porch recess 1.8 m deep, flat porch beam at ceiling height carrying the upper wall, two white posts at x 3.95 and 6.5, three concrete steps, entrance door set back, porch light on the wall.
- x 6.8..13.0: main wall: picture window 1.8 m (x 7.5..9.3) and window 1.2 m (x 10.4..11.6), upper windows 1.2 m (7.0..8.2) and 1.3 m (10.25..11.55), gutter at eave, downspout at the east corner.
- Roof: side-gable, ridge along X, pitch 0.42, overhang 0.6 m eaves / 0.45 m rake, ridge cap.
Side (east): siding, kitchen window (user's kitchen model supplies the glass), dining window, two upper windows, gable with a small ridge cap, rear stoop of two steps visible at the back, AC condenser slot.
Foundation band: concrete up to Y 0.12 (0.48 m exposed above ground).

## 6. Rooms: furniture, lighting, sightlines, sound

| Room | Furniture (slots; user's models) | Light | Sightline | Sound |
|---|---|---|---|---|
| Hall | shoe rack, coat hooks, dresser (placed) | hall_entry 2700K ceiling | straight run to back window (z 8.8), stairs up on the right | wooden floor creak, fridge hum from kitchen |
| Living | sofa (placed), armchair, TV console, coffee table, rug, side table + lamp, bookshelf | warm lamp 2200K + window fill | wide picture window to fog and the empty street | TV static (optional), clock |
| Dining | table, 4 chairs, sideboard | pendant | cased opening to living, kitchen at the end | chair scrape, window wind |
| Kitchen | user's L-shaped kitchen (placed), fridge slot | cool fluorescent 4000K | window above the sink faces east side yard | fridge hum, tap drip, ticking |
| Ground bath | tub, toilet, vanity | none by default (door shut) | door only | pipes |
| Utility | washer, dryer, shelf, water heater | bulb | rear door, window | boiler tick |
| Garage | car, workbench, shelves, bins | cold bulb | long deep room, daylight only through the door seams and one window | echo, drip |
| Basement | shelves, boxes, furnace | single bulb | stairs drop into a low concrete room | pipe knock, furnace |
| Upper hall | console | warm bulb + front window fill | stair rail, long run to landing | stair creak |
| Bedroom 2 | bed, desk, wardrobe | desk lamp slot | window to street | |
| Study (3) | desk, chair, bookcase | window | street | paper, clock |
| Main bedroom | bed (placed), dresser (placed), nightstands, wardrobe | bedside lamp | back window | |
| Bath 2 / closet | tub, toilet, vanity / rails | none | | drip |

Basement: justified only because the earlier request placed a basement door in the garage; kept small (stair + one room under the garage), no other connection.

## 7. Materials and palette

| Mat | Texture | Tint |
|---|---|---|
| Siding | siding.ktx2 (lap, algae and drip stains) | grey-green |
| Roof | shingle.ktx2 | dark grey |
| Trim/Frame | plain | off-white (exterior casings, window frames, posts); dark-brown interior trim |
| Concrete | concrete.ktx2 | foundation, garage, driveway, steps |
| Grass | grass.ktx2 (leaf speckles) | muted green-olive |
| Asphalt | asphalt.ktx2 | road |
| Floor/Wood | wood_floor.ktx2 | warm brown |
| Paper | wallpaper.ktx2 | bedrooms 2 and master |
| Plaster | plaster.ktx2 | all other walls |
| Tile | tile.ktx2 | bathrooms, kitchen |
| Door | wood | dark brown |
| Glass | transparent, reflective | |

## 8. Lighting plan

- Directional "overcast" light 26 lux, cool blue-grey, one cascade, shadows on (cheap, 34 m range).
- Ambient 3.0 cool; fog density 0.02, haze colour 0.38/0.40/0.43.
- 12 practical lights (warm/amber/green-white/cold) with shadow budget (nearest N get shadows depending on quality setting).
- 12 window-fill lights (cool sky colour, no shadows) just inside each window to simulate diffuse daylight.
- Flashlight from the player (existing).
- Exposure keys [ and ] still available.

## 9. Audio plan (no audio files in the project yet)

Ambient: wind in fog (loop), distant traffic hiss (very low), dripping gutter near downspouts. Interior: fridge hum (kitchen), clock tick (living), boiler tick (utility), pipe knock (basement, rare), floor and stair creaks per surface (wood vs. concrete vs. tile footsteps), door creak/latch (existing doors have states open/close for hooking). All sources are listed with positions in the table above; to add them put .ogg files in assets/audio and spawn `AudioPlayer` with spatial settings at those positions.

## 10. Exterior and street

Lot x -5.5..17.0, z -8.4..19.1 (invisible boundary walls 1.8 m high keep the player inside; fences are the user's slots). Front: lawn, concrete driveway 3.2 m wide with slope to the lane, path to porch steps, sidewalk (1.4 m), curb, asphalt road going into fog. Rear yard: lawn, stoop at the utility door, side passage on the east (x 12.7..13.0 + lot). Slots for the street lamp, mailbox, parked car, trees, neighbours, bins, AC unit.

## 11. Collision and navigation

- Player: circle 0.3 m radius, 1.78 m height, step 0.31 m, snap-down 0.5 m.
- Walls, foundations, slabs, door leaves, stair stringers and furniture boxes are AABB solids; stairs and the driveway are ramps; the stair hole in the upper slab starts where the head clears the slab.
- Verified in a headless harness: stairs up/down at 30–144 fps and two walking speeds, basement stairs both ways, porch steps, driveway ramp, rear stoop, all 8 doors by E key and by pushing, walls blocking, lot boundary blocking.

## 12. Implementation (files)

- src/sim/: math.rs, collision.rs, doors.rs (pure logic, no Bevy)
- src/house/: mod.rs (house builder, walls/openings), plan.rs (rooms, walls, lights, slots), shell.rs (roofs, porch, ground, basement), stairs.rs, geom.rs (materials and mesh builder), models.rs (placed user models)
- src/game/: player, hud, atmosphere, quality, debug, props, world (Bevy systems)
- assets/textures: siding, shingle, grass, asphalt plus the older ones.

## 13. Implemented / not implemented

Implemented: the whole shell, rooms, stairs, basement, porch, roofs, driveway, road, lawn, collisions, doors, lighting, textures.
Not implemented (waiting for user's models): all furniture except the sofa, hall dresser, main bedroom dresser/bed and kitchen; car, mailbox, fences, trees, neighbours, street lamp, curtains, lamps. Audio files. Interior details like switches/outlets (small, skipped).

## 14. Limitations and assumptions

- I could not compile or run Bevy in my sandbox. The geometry was tested with a headless harness and a software renderer, so lighting values, texture look and performance are unverified in the real engine.
- Brief conflicts: the latest brief wins on the exterior (visible house, lawn, street); the "only the user's own models" rule is kept, so every prop is an empty slot; physical doors kept (E key, push); basement kept small.
- Invisible boundary walls enclose the lot.
- Kitchen is only 7.8 m² of floor because it is open to the dining area; the user's L-shaped model fits along the back and east walls.
