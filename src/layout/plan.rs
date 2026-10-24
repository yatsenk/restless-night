use super::*;

const HT: f32 = 0.16;
const WX: f32 = P_X1 + HT;

pub(super) fn rooms() -> Vec<Room> {
    let mut v = Vec::new();
    let mut garage = Room::new(0.0, BAY_X1, G_Z0, G_Z1, 0, Mat::Concrete, Mat::Plaster);
    garage.hole_floor = Some(Hole {
        x0: 0.0,
        x1: B_X1,
        z0: B_HZ0,
        z1: B_ZT,
    });
    v.push(garage);
    v.push(Room::new(
        0.0,
        BAY_X1,
        U_Z0,
        MZ1,
        0,
        Mat::Tile,
        Mat::Plaster,
    ));
    v.push(Room::new(
        MX0,
        P_X1,
        P_Z0,
        P_Z1,
        0,
        Mat::Concrete,
        Mat::Siding,
    ));
    let mut hall = Room::new(MX0, P_X1, H_Z0, MZ1, 0, Mat::Floor, Mat::Plaster);
    hall.hole_ceil = Some(Hole {
        x0: MX0,
        x1: S_X1,
        z0: HOLE_Z0,
        z1: S_END,
    });
    v.push(hall);
    v.push(Room::new(WX, MX1, 0.0, 4.8, 0, Mat::Floor, Mat::Plaster));
    v.push(Room::new(WX, MX1, 4.96, 6.6, 0, Mat::Floor, Mat::Plaster));
    v.push(Room::new(9.16, MX1, 6.6, MZ1, 0, Mat::Tile, Mat::Plaster));
    v.push(Room::new(WX, 9.0, 6.76, MZ1, 0, Mat::Tile, Mat::Plaster));

    let mut up_hall = Room::new(MX0, 5.7, 0.0, MZ1, 1, Mat::Floor, Mat::Plaster);
    up_hall.hole_floor = Some(Hole {
        x0: MX0,
        x1: S_X1,
        z0: HOLE_Z0,
        z1: S_END,
    });
    v.push(up_hall);
    v.push(Room::new(5.86, 9.0, 0.0, 3.5, 1, Mat::Floor, Mat::Paper));
    v.push(Room::new(9.16, MX1, 0.0, 3.5, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(5.7, MX1, 3.66, 4.76, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(5.86, 8.0, 4.92, 7.0, 1, Mat::Tile, Mat::Plaster));
    v.push(Room::new(5.86, 8.0, 7.16, MZ1, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(8.16, MX1, 4.92, MZ1, 1, Mat::Floor, Mat::Paper));
    v
}

fn win(c: f32, hw: f32, base: f32) -> Op {
    Op::new(
        c - hw,
        c + hw,
        base + WIN_SILL,
        base + WIN_HEAD,
        Kind::Window,
    )
}

fn small(c: f32, hw: f32, base: f32) -> Op {
    Op::new(c - hw, c + hw, base + 1.4, base + 2.1, Kind::Window)
}

fn door(c: f32, base: f32) -> Op {
    Op::new(
        c - OPEN_W * 0.5,
        c + OPEN_W * 0.5,
        base,
        base + DOOR_H,
        Kind::Door,
    )
}

fn cased(c: f32, hw: f32) -> Op {
    Op::new(c - hw, c + hw, 0.0, DOOR_H, Kind::Opening)
}

fn part(axis: Axis, c: f32, a0: f32, a1: f32, y0: f32, y1: f32) -> Wall {
    Wall::new(axis, c - HALF, c + HALF, a0, a1, y0, y1)
}

pub(super) fn walls() -> Vec<Wall> {
    let u = UPPER;
    let mut v = Vec::new();

    let mut gf = Wall::ext(Axis::X, G_Z0 - EXT, G_Z0, -EXT, MX0, GROUND, CEIL0);
    gf.ops = vec![Op::new(0.5, 3.0, 0.0, 2.15, Kind::GarageDoor)];
    gf.breaks = vec![BAY_X1];
    v.push(gf);

    let mut gw = Wall::ext(Axis::Z, -EXT, 0.0, G_Z0 - EXT, OUT_Z1, GROUND, CEIL0);
    gw.ops = vec![win(2.4, 0.6, 0.0), small(7.2, 0.45, 0.0)];
    gw.breaks = vec![0.0, G_Z1, U_Z0, G_Z1 + 0.3];
    v.push(gw);

    let mut bb = Wall::ext(Axis::X, MZ1, OUT_Z1, -EXT, BAY_X1, GROUND, CEIL0);
    bb.ops = vec![Op::new(2.4, 3.3, 0.0, 2.2, Kind::ExtDoor)];
    v.push(bb);

    let mut ff = Wall::ext(Axis::X, -EXT, 0.0, P_X1, OUT_X1, GROUND, TOP);
    ff.cap_lo = true;
    ff.ops = vec![
        win(8.4, 0.9, 0.0),
        win(11.0, 0.6, 0.0),
        win(7.6, 0.6, u),
        win(10.9, 0.65, u),
    ];
    ff.breaks = vec![WX, 9.0, 9.16];
    v.push(ff);

    let mut fp = Wall::ext(Axis::X, -EXT, 0.0, MX0, P_X1, CEIL0, TOP);
    fp.ops = vec![win(4.7, 0.45, u)];
    v.push(fp);

    let mut east = Wall::ext(Axis::Z, MX1, OUT_X1, -EXT, OUT_Z1, GROUND, TOP);
    east.ops = vec![
        win(5.7, 0.55, 0.0),
        Op::new(6.725, 7.935, 1.075, 2.345, Kind::ModelWindow),
        win(1.75, 0.6, u),
        win(6.5, 0.6, u),
    ];
    east.breaks = vec![0.0, 4.8, 6.6, 8.8];
    v.push(east);

    let mut back = Wall::ext(Axis::X, MZ1, OUT_Z1, BAY_X1, OUT_X1, GROUND, TOP);
    back.ops = vec![
        win(5.2, 0.45, 0.0),
        small(8.0, 0.35, 0.0),
        win(4.7, 0.45, u),
        win(10.4, 0.7, u),
    ];
    back.breaks = vec![MX0, P_X1, WX, 9.0, 9.16];
    v.push(back);

    v.push(Wall::ext(Axis::Z, BAY_X1, MX0, OUT_Z0, P_Z0, GROUND, CEIL0));
    let mut party = Wall::ext(Axis::Z, BAY_X1, MX0, P_Z0, OUT_Z1, GROUND, TOP);
    party.ops = vec![cased(7.2, 0.5), win(7.4, 0.45, u)];
    party.breaks = vec![0.0, G_Z1, U_Z0, S_Z0, S_END];
    v.push(party);

    let mut porch_back = Wall::ext(Axis::X, P_Z1, H_Z0, MX0, P_X1 + 0.03, 0.0, CEIL0);
    porch_back.ops = vec![Op::new(4.75, 5.65, 0.0, 2.2, Kind::ExtDoor)];
    v.push(porch_back);

    let mut hall_e = Wall::new(Axis::Z, P_X1, WX, 0.0, MZ1 + 0.03, 0.0, CEIL0);
    hall_e.ops = vec![cased(4.0, 0.6), door(7.66, 0.0)];
    hall_e.breaks = vec![P_Z1, H_Z0, 4.8, 4.96, 6.6, 6.76];
    v.push(hall_e);

    let mut lv = part(Axis::X, 4.88, P_X1, MX1 + 0.03, 0.0, CEIL0);
    lv.ops = vec![cased(10.0, 1.2)];
    v.push(lv);

    v.push(part(Axis::X, 6.68, P_X1, 9.0, 0.0, CEIL0));

    let mut be = part(Axis::Z, 9.08, 6.6, MZ1 + 0.03, 0.0, CEIL0);
    be.cap_lo = true;
    v.push(be);

    let mut gb = Wall::new(Axis::X, G_Z1, U_Z0, -0.03, BAY_X1 + 0.03, 0.0, CEIL0);
    gb.ops = vec![door(2.9, 0.0)];
    v.push(gb);

    let mut sw = Wall::new(
        Axis::Z,
        B_X1,
        B_X1 + HT,
        B_HZ0 - HT,
        G_Z1 + 0.03,
        0.0,
        CEIL0,
    );
    sw.ops = vec![door(4.86, 0.0)];
    v.push(sw);
    v.push(Wall::new(
        Axis::X,
        B_HZ0 - HT,
        B_HZ0,
        -0.03,
        B_X1 + HT,
        0.0,
        CEIL0,
    ));

    let mut hf = part(Axis::Z, 5.78, -0.03, 3.5, u, TOP);
    hf.breaks = vec![];
    v.push(hf);

    let mut fb = part(Axis::X, 3.58, 5.7, MX1 + 0.03, u, TOP);
    fb.ops = vec![door(7.0, u), door(10.9, u)];
    fb.cap_lo = true;
    v.push(fb);

    v.push(part(Axis::Z, 9.08, -0.03, 3.5, u, TOP));

    let mut cb = part(Axis::X, 4.84, 5.7, MX1 + 0.03, u, TOP);
    cb.ops = vec![door(9.9, u)];
    v.push(cb);

    let mut hb = part(Axis::Z, 5.78, 4.76, MZ1 + 0.03, u, TOP);
    hb.ops = vec![door(5.95, u)];
    v.push(hb);

    v.push(part(Axis::X, 7.08, 5.7, 8.16, u, TOP));

    let mut mw = part(Axis::Z, 8.08, 4.76, MZ1 + 0.03, u, TOP);
    mw.ops = vec![door(7.9, u)];
    v.push(mw);

    v
}

pub(super) fn lights() -> Vec<LightSpec> {
    let warm = [1.0, 0.72, 0.44];
    let amber = [1.0, 0.58, 0.28];
    let sick = [0.8, 0.92, 0.78];
    let cold = [0.72, 0.84, 1.0];
    let sky = [0.66, 0.74, 0.9];
    let p =
        |name: &'static str, x: f32, y: f32, z: f32, color: [f32; 3], lumens: f32, range: f32| {
            LightSpec {
                pos: v3(x, y, z),
                color,
                lumens,
                range,
                name,
                kind: LightKind::Practical,
            }
        };
    let w = |name: &'static str, x: f32, y: f32, z: f32, lumens: f32, range: f32| LightSpec {
        pos: v3(x, y, z),
        color: sky,
        lumens,
        range,
        name,
        kind: LightKind::Window,
    };
    vec![
        p("porch_light", 6.3, 2.15, 0.9, warm, 260.0, 5.5),
        p("hall_entry", 5.3, 2.35, 3.2, warm, 430.0, 8.0),
        p("hall_back", 5.3, 2.35, 7.2, warm, 240.0, 6.0),
        p("living_lamp", 11.9, 1.15, 1.0, amber, 380.0, 7.5),
        p("dining_pendant", 8.9, 2.0, 5.8, amber, 300.0, 6.0),
        p("kitchen_ceiling", 10.9, 2.4, 7.6, sick, 620.0, 7.0),
        p("garage_bulb", 1.9, 2.3, 2.4, cold, 460.0, 8.0),
        p("utility_bulb", 1.9, 2.3, 7.2, sick, 240.0, 5.0),
        p("basement_bulb", 2.3, -0.65, 2.6, sick, 360.0, 6.5),
        p("upper_hall", 4.7, 4.95, 1.9, warm, 330.0, 7.5),
        p("upper_landing", 4.7, 4.95, 7.4, warm, 200.0, 6.0),
        p("mb_lamp", 11.9, 3.55, 7.9, amber, 300.0, 6.5),
        w("win_living_a", 8.4, 1.6, 0.6, 430.0, 7.5),
        w("win_living_b", 11.0, 1.5, 0.5, 240.0, 6.0),
        w("win_dining", 12.1, 1.5, 5.7, 200.0, 5.5),
        w("win_kitchen", 12.1, 1.8, 7.3, 180.0, 5.0),
        w("win_hall_back", 5.2, 1.5, 8.3, 220.0, 6.0),
        w("win_garage", 0.6, 1.5, 2.4, 160.0, 5.0),
        w("win_up_hall_front", 4.7, 4.2, 0.6, 260.0, 6.5),
        w("win_up_hall_back", 4.7, 4.2, 8.2, 200.0, 6.0),
        w("win_br2", 7.6, 4.2, 0.6, 300.0, 6.5),
        w("win_study", 10.9, 4.2, 0.6, 300.0, 6.5),
        w("win_mb_back", 10.4, 4.2, 8.2, 380.0, 7.5),
        w("win_mb_east", 12.1, 4.2, 6.5, 220.0, 5.5),
    ]
}

pub(super) fn slots() -> Vec<Slot> {
    let s = |name: &'static str, need: &'static str, min: [f32; 3], max: [f32; 3]| Slot {
        name,
        need,
        min: v3(min[0], min[1], min[2]),
        max: v3(max[0], max[1], max[2]),
    };
    let u = UPPER;
    let f = B_FLOOR;
    vec![
        s(
            "living_tv_console",
            "TV on low console 1.4 x 0.45 m",
            [12.2, 0.0, 1.2],
            [12.68, 1.1, 2.6],
        ),
        s(
            "living_coffee_table",
            "coffee table 1.1 x 0.55 m",
            [9.6, 0.0, 1.5],
            [10.7, 0.45, 2.1],
        ),
        s(
            "living_armchair",
            "armchair",
            [10.4, 0.0, 3.1],
            [11.3, 0.9, 4.0],
        ),
        s(
            "living_rug",
            "rug 2.4 x 1.8 m",
            [9.0, 0.0, 1.0],
            [11.4, 0.02, 2.8],
        ),
        s(
            "living_side_table",
            "side table with lamp",
            [7.1, 0.0, 3.3],
            [7.6, 0.6, 3.8],
        ),
        s(
            "living_bookshelf",
            "low bookshelf",
            [7.2, 0.0, 0.15],
            [8.4, 1.3, 0.5],
        ),
        s(
            "hall_shoe_rack",
            "shoe rack or bench",
            [5.0, 0.0, 1.9],
            [5.9, 0.5, 2.3],
        ),
        s(
            "hall_coat_hooks",
            "coat hooks or rack",
            [6.4, 0.0, 5.2],
            [6.78, 1.8, 5.9],
        ),
        s(
            "dining_table",
            "dining table 1.5 x 0.9 m",
            [8.2, 0.0, 5.2],
            [9.7, 0.76, 6.1],
        ),
        s(
            "dining_chair_1",
            "chair",
            [8.3, 0.0, 4.98],
            [8.75, 0.9, 5.25],
        ),
        s(
            "dining_chair_2",
            "chair",
            [9.15, 0.0, 4.98],
            [9.6, 0.9, 5.25],
        ),
        s(
            "dining_chair_3",
            "chair",
            [8.3, 0.0, 6.05],
            [8.75, 0.9, 6.55],
        ),
        s(
            "dining_chair_4",
            "chair",
            [9.15, 0.0, 6.05],
            [9.6, 0.9, 6.55],
        ),
        s(
            "dining_sideboard",
            "sideboard",
            [7.05, 0.0, 5.0],
            [7.5, 0.9, 6.2],
        ),
        s(
            "kitchen_fridge",
            "refrigerator",
            [9.2, 0.0, 6.7],
            [9.85, 1.8, 7.3],
        ),
        s(
            "bath_tub",
            "bathtub 1.5 x 0.7 m",
            [7.0, 0.0, 6.8],
            [8.5, 0.55, 7.5],
        ),
        s("bath_toilet", "toilet", [8.4, 0.0, 8.0], [8.95, 0.8, 8.75]),
        s(
            "bath_sink",
            "vanity with mirror",
            [7.0, 0.0, 8.0],
            [7.9, 0.9, 8.75],
        ),
        s(
            "utility_washer",
            "washing machine",
            [0.6, 0.0, 8.1],
            [1.25, 0.95, 8.75],
        ),
        s("utility_dryer", "dryer", [1.3, 0.0, 8.1], [1.9, 0.95, 8.75]),
        s(
            "utility_shelf",
            "shelving unit",
            [0.05, 0.0, 5.8],
            [0.5, 1.9, 7.2],
        ),
        s(
            "utility_water_heater",
            "water heater",
            [0.05, 0.0, 7.4],
            [0.5, 1.5, 8.0],
        ),
        s(
            "garage_car",
            "family car 4.5 x 1.8 m",
            [1.5, 0.0, -0.4],
            [3.4, 1.5, 4.1],
        ),
        s(
            "garage_workbench",
            "workbench",
            [1.6, 0.0, 4.75],
            [3.4, 0.9, 5.35],
        ),
        s(
            "garage_shelves",
            "metal shelving",
            [3.0, 0.0, 0.5],
            [3.45, 1.9, 2.3],
        ),
        s(
            "garage_bins",
            "storage bins and tools",
            [0.1, 0.0, -0.9],
            [0.9, 1.2, 1.4],
        ),
        s(
            "basement_shelves",
            "metal shelving",
            [0.1, f, 0.2],
            [0.5, f + 1.9, 1.3],
        ),
        s(
            "basement_boxes",
            "crates and boxes",
            [1.4, f, 3.8],
            [3.4, f + 0.9, 5.3],
        ),
        s(
            "basement_furnace",
            "furnace or boiler",
            [2.4, f, 0.1],
            [3.4, f + 1.6, 0.9],
        ),
        s(
            "br2_bed",
            "single bed 0.95 x 2.0 m",
            [5.95, u, 0.2],
            [6.9, u + 0.6, 2.2],
        ),
        s(
            "br2_desk",
            "desk and chair",
            [8.1, u, 0.15],
            [8.95, u + 0.78, 1.3],
        ),
        s(
            "br2_wardrobe",
            "wardrobe 1.2 x 0.55 m",
            [7.2, u, 2.9],
            [8.4, u + 2.0, 3.45],
        ),
        s(
            "study_desk",
            "desk 1.4 x 0.7 m",
            [10.6, u, 0.15],
            [12.0, u + 0.76, 0.85],
        ),
        s(
            "study_chair",
            "office chair",
            [10.9, u, 0.9],
            [11.5, u + 1.0, 1.5],
        ),
        s(
            "study_bookcase",
            "bookcase",
            [9.25, u, 2.85],
            [10.6, u + 2.0, 3.45],
        ),
        s(
            "mb_nightstand_a",
            "nightstand",
            [10.3, u, 8.2],
            [10.8, u + 0.6, 8.75],
        ),
        s(
            "mb_nightstand_b",
            "nightstand",
            [12.0, u, 8.2],
            [12.65, u + 0.6, 8.75],
        ),
        s(
            "mb_wardrobe",
            "wardrobe 1.4 x 0.6 m",
            [8.2, u, 6.2],
            [8.8, u + 2.1, 7.6],
        ),
        s(
            "closet_shelves",
            "closet rails and shelves",
            [5.9, u, 7.2],
            [7.95, u + 2.0, 8.75],
        ),
        s(
            "bath2_tub",
            "tub or shower 1.5 x 0.75 m",
            [5.9, u, 6.1],
            [7.95, u + 0.6, 6.95],
        ),
        s(
            "bath2_toilet",
            "toilet",
            [7.4, u, 4.95],
            [7.95, u + 0.8, 5.6],
        ),
        s(
            "bath2_sink",
            "vanity with mirror",
            [5.9, u, 4.95],
            [6.8, u + 0.9, 5.5],
        ),
        s(
            "upper_hall_console",
            "narrow console or bookcase",
            [3.7, u, 7.3],
            [4.2, u + 1.8, 8.7],
        ),
        s(
            "front_door_mat",
            "door mat",
            [4.6, 0.0, 0.5],
            [5.8, 0.03, 1.2],
        ),
        s(
            "porch_chair",
            "porch chair or planter",
            [3.8, 0.0, 0.2],
            [4.4, 0.9, 0.8],
        ),
        s(
            "mailbox",
            "mailbox on post",
            [2.2, GROUND, -7.0],
            [2.6, 1.2, -6.7],
        ),
        s(
            "street_lamp",
            "street light",
            [6.0, GROUND, -8.8],
            [6.4, 5.0, -8.5],
        ),
        s(
            "trash_bins",
            "two wheeled bins",
            [13.4, GROUND, 4.0],
            [14.4, 1.0, 5.4],
        ),
        s(
            "ac_unit",
            "air-conditioner condenser",
            [13.4, GROUND, 6.8],
            [14.3, 0.7, 7.8],
        ),
        s(
            "front_tree",
            "bare deciduous tree",
            [8.5, GROUND, -5.5],
            [9.5, 8.0, -4.5],
        ),
        s(
            "fence_west",
            "fence sections",
            [LOT_X0, GROUND, 0.0],
            [LOT_X0 + 0.1, 1.8, LOT_Z1],
        ),
        s(
            "fence_east",
            "fence sections",
            [LOT_X1 - 0.1, GROUND, 0.0],
            [LOT_X1, 1.8, LOT_Z1],
        ),
        s(
            "fence_back",
            "fence sections",
            [LOT_X0, GROUND, LOT_Z1 - 0.1],
            [LOT_X1, 1.8, LOT_Z1],
        ),
        s(
            "neighbor_west",
            "neighbouring house in fog",
            [-22.0, GROUND, -2.0],
            [-9.0, 7.5, 9.0],
        ),
        s(
            "neighbor_east",
            "neighbouring house in fog",
            [20.0, GROUND, -2.0],
            [33.0, 7.5, 9.0],
        ),
        s(
            "street_parked_car",
            "parked car at the curb",
            [8.0, GROUND, -12.4],
            [10.0, 1.5, -9.2],
        ),
    ]
}
