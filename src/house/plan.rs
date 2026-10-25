use super::*;

const HT: f32 = 0.16;
const WX: f32 = P_X1 + HT;
const LV: f32 = 5.48;
const DN: f32 = 7.98;
const KX: f32 = 9.16;
const FB: f32 = 4.28;
const CB: f32 = 5.84;
const BR_X: f32 = 9.48;
const BH_X: f32 = 6.28;
const UH: f32 = 6.2;
const BRX: f32 = BH_X + 0.08;
const MW_X: f32 = 8.28;
const CL_Z: f32 = 8.48;

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
    v.push(Room::new(0.0, BAY_X1, U_Z0, MZ1, 0, Mat::Tile, Mat::Plaster));
    v.push(Room::new(MX0, P_X1, P_Z0, P_Z1, 0, Mat::Concrete, Mat::Siding));
    let mut hall = Room::new(MX0, P_X1, H_Z0, MZ1, 0, Mat::Floor, Mat::Plaster);
    hall.hole_ceil = Some(Hole {
        x0: MX0,
        x1: S_X1,
        z0: HOLE_Z0,
        z1: S_END,
    });
    v.push(hall);
    v.push(Room::new(WX, MX1, 0.0, LV - HT * 0.5, 0, Mat::Floor, Mat::Plaster));
    v.push(Room::new(WX, MX1, LV + HT * 0.5, DN - HT * 0.5, 0, Mat::Floor, Mat::Plaster));
    v.push(Room::new(KX, MX1, DN - HT * 0.5, MZ1, 0, Mat::Tile, Mat::Plaster));
    v.push(Room::new(WX, KX - 0.16, DN + HT * 0.5, MZ1, 0, Mat::Tile, Mat::Plaster));

    let mut up_hall = Room::new(MX0, UH, 0.0, MZ1, 1, Mat::Floor, Mat::Plaster);
    up_hall.hole_floor = Some(Hole {
        x0: MX0,
        x1: S_X1,
        z0: HOLE_Z0,
        z1: S_END,
    });
    v.push(up_hall);
    v.push(Room::new(BRX, BR_X - HT * 0.5, 0.0, FB - HT * 0.5, 1, Mat::Floor, Mat::Paper));
    v.push(Room::new(BR_X + HT * 0.5, MX1, 0.0, FB - HT * 0.5, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(UH, MX1, FB + HT * 0.5, CB - HT * 0.5, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(BRX, MW_X - HT * 0.5, CB + HT * 0.5, CL_Z - HT * 0.5, 1, Mat::Tile, Mat::Plaster));
    v.push(Room::new(BRX, MW_X - HT * 0.5, CL_Z + HT * 0.5, MZ1, 1, Mat::Floor, Mat::Plaster));
    v.push(Room::new(MW_X + HT * 0.5, MX1, CB + HT * 0.5, MZ1, 1, Mat::Floor, Mat::Paper));
    v
}

fn win(c: f32, hw: f32, base: f32) -> Op {
    Op::new(c - hw, c + hw, base + WIN_SILL, base + WIN_HEAD, Kind::Window)
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

fn cased(c: f32, hw: f32, base: f32) -> Op {
    Op::new(c - hw, c + hw, base, base + DOOR_H, Kind::Opening)
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
    gw.ops = vec![win(2.4, 0.6, 0.0), small(8.2, 0.45, 0.0)];
    gw.breaks = vec![0.0, G_Z1, U_Z0, G_Z1 + 0.3];
    v.push(gw);

    let mut bb = Wall::ext(Axis::X, MZ1, OUT_Z1, -EXT, BAY_X1, GROUND, CEIL0);
    bb.ops = vec![Op::new(2.4, 3.3, 0.0, 2.3, Kind::ExtDoor)];
    v.push(bb);

    let mut ff = Wall::ext(Axis::X, -EXT, 0.0, P_X1, OUT_X1, GROUND, TOP);
    ff.cap_lo = true;
    ff.ops = vec![
        win(9.2, 0.9, 0.0),
        win(12.4, 0.7, 0.0),
        win(14.4, 0.5, 0.0),
        win(8.5, 0.6, u),
        win(12.4, 0.7, u),
    ];
    ff.breaks = vec![WX, KX - 0.16, KX];
    v.push(ff);

    let mut fp = Wall::ext(Axis::X, -EXT, 0.0, MX0, P_X1, CEIL0, TOP);
    fp.ops = vec![win(4.7, 0.45, u)];
    v.push(fp);

    let mut east = Wall::ext(Axis::Z, MX1, OUT_X1, -EXT, OUT_Z1, GROUND, TOP);
    east.ops = vec![
        win(2.7, 0.6, 0.0),
        win(6.7, 0.55, 0.0),
        Op::new(MZ1 - 2.075, MZ1 - 0.865, 1.075, 2.345, Kind::ModelWindow),
        win(2.1, 0.6, u),
        win(8.2, 0.7, u),
    ];
    east.breaks = vec![0.0, LV - HT * 0.5, DN - HT * 0.5, MZ1];
    v.push(east);

    let mut back = Wall::ext(Axis::X, MZ1, OUT_Z1, BAY_X1, OUT_X1, GROUND, TOP);
    back.ops = vec![
        win(5.7, 0.45, 0.0),
        small(8.2, 0.35, 0.0),
        win(4.7, 0.45, u),
        win(7.0, 0.5, u),
        win(11.8, 0.8, u),
    ];
    back.breaks = vec![MX0, P_X1, WX, KX - 0.16, KX];
    v.push(back);

    v.push(Wall::ext(Axis::Z, BAY_X1, MX0, OUT_Z0, P_Z0, GROUND, CEIL0));
    let mut party = Wall::ext(Axis::Z, BAY_X1, MX0, P_Z0, OUT_Z1, GROUND, TOP);
    party.ops = vec![cased(9.6, 0.5, 0.0), win(7.2, 0.45, u)];
    party.breaks = vec![0.0, G_Z1, U_Z0, S_Z0, S_END];
    v.push(party);

    let mut porch_back = Wall::ext(Axis::X, P_Z1, H_Z0, MX0, P_X1 + 0.03, 0.0, CEIL0);
    porch_back.ops = vec![Op::new(5.0, 6.0, 0.0, 2.3, Kind::ExtDoor)];
    v.push(porch_back);

    let mut hall_e = Wall::new(Axis::Z, P_X1, WX, 0.0, MZ1 + 0.03, 0.0, CEIL0);
    hall_e.ops = vec![
        cased(4.4, 0.75, 0.0),
        cased(6.7, 0.75, 0.0),
        door(9.2, 0.0),
    ];
    hall_e.breaks = vec![P_Z1, H_Z0, LV - HT * 0.5, LV + HT * 0.5, DN - HT * 0.5, DN + HT * 0.5];
    v.push(hall_e);

    let mut lv = part(Axis::X, LV, P_X1, MX1 + 0.03, 0.0, CEIL0);
    lv.ops = vec![cased(10.8, 1.4, 0.0)];
    v.push(lv);

    v.push(part(Axis::X, DN, P_X1, KX - 0.16, 0.0, CEIL0));

    let mut be = part(Axis::Z, KX - 0.08, DN - HT * 0.5, MZ1 + 0.03, 0.0, CEIL0);
    be.cap_lo = true;
    v.push(be);

    let mut gb = Wall::new(Axis::X, G_Z1, U_Z0, -0.03, BAY_X1 + 0.03, 0.0, CEIL0);
    gb.ops = vec![door(2.9, 0.0)];
    v.push(gb);

    let mut sw = Wall::new(Axis::Z, B_X1, B_X1 + HT, B_HZ0 - HT, G_Z1 + 0.03, 0.0, CEIL0);
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

    let mut hf = part(Axis::Z, BH_X, -0.03, FB - HT * 0.5, u, TOP);
    hf.breaks = vec![];
    v.push(hf);

    let mut fb = part(Axis::X, FB, UH, MX1 + 0.03, u, TOP);
    fb.ops = vec![cased(7.3, 0.6, u), cased(10.6, 0.6, u)];
    fb.cap_lo = true;
    v.push(fb);

    v.push(part(Axis::Z, BR_X, -0.03, FB - HT * 0.5, u, TOP));

    let mut cb = part(Axis::X, CB, UH, MX1 + 0.03, u, TOP);
    cb.ops = vec![cased(10.5, 0.7, u)];
    v.push(cb);

    let mut hb = part(Axis::Z, BH_X, CB - HT * 0.5, MZ1 + 0.03, u, TOP);
    hb.ops = vec![door(7.1, u)];
    v.push(hb);

    v.push(part(Axis::X, CL_Z, UH, MW_X, u, TOP));

    let mut mw = part(Axis::Z, MW_X, CB - HT * 0.5, MZ1 + 0.03, u, TOP);
    mw.ops = vec![cased(9.4, 0.5, u)];
    v.push(mw);

    v
}

pub(super) fn lights() -> Vec<LightSpec> {
    let warm = [1.0, 0.72, 0.44];
    let amber = [1.0, 0.58, 0.28];
    let sick = [0.8, 0.92, 0.78];
    let cold = [0.72, 0.84, 1.0];
    let sky = [0.66, 0.74, 0.9];
    let p = |name: &'static str, x: f32, y: f32, z: f32, color: [f32; 3], lumens: f32, range: f32| {
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
        p("porch_light", 6.9, 2.3, 0.9, warm, 260.0, 5.5),
        p("hall_entry", 5.5, 2.5, 3.0, warm, 430.0, 8.5),
        p("hall_back", 5.5, 2.5, 8.6, warm, 240.0, 6.5),
        p("living_lamp", 13.4, 1.15, 1.6, amber, 420.0, 9.0),
        p("dining_pendant", 11.4, 2.1, 6.7, amber, 320.0, 7.0),
        p("kitchen_ceiling", 12.2, 2.6, 9.0, sick, 620.0, 7.5),
        p("garage_bulb", 1.9, 2.4, 2.4, cold, 460.0, 8.0),
        p("utility_bulb", 1.9, 2.4, 8.0, sick, 300.0, 6.0),
        p("basement_bulb", 2.3, -0.65, 2.6, sick, 360.0, 6.5),
        p("basement_bulb_2", 5.4, -0.65, 7.2, sick, 360.0, 8.0),
        p("upper_hall", 4.7, 5.15, 2.4, warm, 330.0, 7.5),
        p("upper_landing", 4.7, 5.15, 9.0, warm, 200.0, 6.0),
        p("mb_lamp", 12.6, 3.75, 9.7, amber, 320.0, 7.0),
        w("win_living_a", 9.2, 1.6, 0.6, 430.0, 8.0),
        w("win_living_b", 12.0, 1.5, 0.5, 260.0, 7.0),
        w("win_living_c", 14.4, 1.5, 0.5, 200.0, 6.0),
        w("win_dining", 15.0, 1.5, 6.7, 200.0, 6.0),
        w("win_kitchen", 15.0, 1.8, 8.8, 180.0, 5.5),
        w("win_hall_back", 5.7, 1.5, 10.0, 220.0, 6.0),
        w("win_garage", 0.6, 1.5, 2.4, 160.0, 5.0),
        w("win_up_hall_front", 4.7, 4.4, 0.6, 260.0, 6.5),
        w("win_up_hall_back", 4.7, 4.4, 9.9, 200.0, 6.0),
        w("win_br2", 8.5, 4.4, 0.6, 300.0, 6.5),
        w("win_study", 12.4, 4.4, 0.6, 300.0, 7.0),
        w("win_mb_back", 11.8, 4.4, 10.0, 380.0, 8.0),
        w("win_mb_east", 15.0, 4.4, 8.2, 220.0, 6.0),
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
        s("living_tv_console", "TV on low console 1.4 x 0.45 m", [14.6, 0.0, 3.7], [15.18, 1.1, 5.1]),
        s("living_coffee_table", "coffee table 1.1 x 0.55 m", [9.7, 0.0, 1.5], [10.8, 0.45, 2.2]),
        s("living_armchair", "armchair", [11.4, 0.0, 3.6], [12.3, 0.9, 4.5]),
        s("living_rug", "rug 3 x 2.4 m", [8.9, 0.0, 0.7], [11.9, 0.02, 3.3]),
        s("living_side_table", "side table with lamp", [8.0, 0.0, 3.6], [8.5, 0.6, 4.1]),
        s("living_bookshelf", "low bookshelf", [9.0, 0.0, 4.9], [10.2, 1.3, 5.35]),
        s("entry_coat_rack", "coat hooks or coat rack", [3.7, 0.0, 1.9], [4.1, 1.9, 3.1]),
        s("entry_shoe_rack", "shoe rack", [3.7, 0.0, 3.2], [4.1, 0.5, 4.3]),
        s("entry_bench", "small bench", [4.3, 0.0, 4.2], [5.5, 0.5, 4.7]),
        s("entry_mirror_console", "console table with mirror", [7.3, 0.0, 2.8], [7.65, 1.8, 3.4]),
        s("dining_table", "dining table 1.5 x 0.9 m", [10.6, 0.0, 6.3], [12.1, 0.76, 7.2]),
        s("dining_chair_1", "chair", [10.8, 0.0, 5.78], [11.25, 0.9, 6.25]),
        s("dining_chair_2", "chair", [11.45, 0.0, 5.78], [11.9, 0.9, 6.25]),
        s("dining_chair_3", "chair", [10.8, 0.0, 7.25], [11.25, 0.9, 7.75]),
        s("dining_chair_4", "chair", [11.45, 0.0, 7.25], [11.9, 0.9, 7.75]),
        s("dining_sideboard", "sideboard", [12.8, 0.0, 5.62], [14.0, 0.9, 6.07]),
        s("kitchen_fridge", "refrigerator", [9.2, 0.0, 8.0], [9.85, 1.8, 8.6]),
        s("bath_tub", "bathtub 1.5 x 0.7 m", [7.4, 0.0, 9.6], [8.9, 0.55, 10.3]),
        s("bath_toilet", "toilet", [8.3, 0.0, 8.1], [8.9, 0.8, 8.8]),
        s("bath_sink", "vanity with mirror", [7.7, 0.0, 8.1], [8.5, 0.9, 8.6]),
        s("utility_washer", "washing machine", [0.6, 0.0, 9.55], [1.25, 0.95, 10.2]),
        s("utility_dryer", "dryer", [1.3, 0.0, 9.55], [1.9, 0.95, 10.2]),
        s("utility_shelf", "shelving unit", [0.05, 0.0, 6.2], [0.5, 1.9, 7.6]),
        s("utility_water_heater", "water heater", [0.05, 0.0, 8.0], [0.5, 1.5, 8.6]),
        s("garage_car", "family car 4.5 x 1.8 m", [1.5, 0.0, -0.4], [3.4, 1.5, 4.1]),
        s("garage_workbench", "workbench", [1.6, 0.0, 4.75], [3.4, 0.9, 5.35]),
        s("garage_shelves", "metal shelving", [3.0, 0.0, 0.5], [3.45, 1.9, 2.3]),
        s("garage_bins", "storage bins and tools", [0.1, 0.0, -0.9], [0.9, 1.2, 1.4]),
        s("basement_shelves", "metal shelving", [0.1, f, 2.2], [0.5, f + 1.9, 5.8]),
        s("basement_boxes", "crates and boxes", [3.0, f, 6.0], [5.2, f + 0.9, 7.6]),
        s("basement_furnace", "furnace or boiler", [5.8, f, 0.2], [7.3, f + 1.6, 1.2]),
        s("basement_workshop", "old table or workbench", [5.0, f, 9.0], [7.3, f + 0.9, 10.2]),
        s("br2_bed", "single bed 0.95 x 2.0 m", [6.5, u, 0.2], [7.45, u + 0.6, 2.2]),
        s("br2_desk", "desk and chair", [8.3, u, 0.15], [9.15, u + 0.78, 1.3]),
        s("br2_wardrobe", "wardrobe 1.2 x 0.55 m", [8.1, u, 3.55], [9.3, u + 2.0, 4.1]),
        s("study_desk", "desk 1.4 x 0.7 m", [11.4, u, 0.15], [12.8, u + 0.76, 0.85]),
        s("study_chair", "office chair", [11.7, u, 0.9], [12.3, u + 1.0, 1.5]),
        s("study_bookcase", "bookcase", [13.2, u, 3.6], [14.7, u + 2.0, 4.15]),
        s("mb_nightstand_a", "nightstand", [10.7, u, 9.7], [11.2, u + 0.6, 10.25]),
        s("mb_nightstand_b", "nightstand", [12.5, u, 9.7], [13.0, u + 0.6, 10.25]),
        s("mb_wardrobe", "wardrobe 1.4 x 0.6 m", [8.45, u, 6.6], [9.05, u + 2.1, 8.0]),
        s("closet_shelves", "closet rails and shelves", [6.5, u, 8.6], [8.1, u + 2.0, 10.25]),
        s("bath2_tub", "tub or shower 1.5 x 0.75 m", [6.5, u, 6.0], [7.9, u + 0.6, 6.75]),
        s("bath2_toilet", "toilet", [7.4, u, 7.6], [8.1, u + 0.8, 8.35]),
        s("bath2_sink", "vanity with mirror", [6.5, u, 7.7], [7.3, u + 0.9, 8.35]),
        s("upper_hall_console", "narrow console or bookcase", [3.7, u, 9.2], [4.2, u + 1.8, 10.2]),
        s("front_door_mat", "door mat", [4.6, 0.0, 0.5], [5.8, 0.03, 1.2]),
        s("porch_chair", "porch chair or planter", [3.8, 0.0, 0.2], [4.4, 0.9, 0.8]),
        s("mailbox", "mailbox on post", [2.2, GROUND, -7.0], [2.6, 1.2, -6.7]),
        s("street_lamp", "street light", [6.0, GROUND, -8.8], [6.4, 5.0, -8.5]),
        s("trash_bins", "two wheeled bins", [15.8, GROUND, 5.0], [16.8, 1.0, 6.4]),
        s("ac_unit", "air-conditioner condenser", [15.8, GROUND, 8.4], [16.7, 0.7, 9.4]),
        s("front_tree", "bare deciduous tree", [8.5, GROUND, -5.5], [9.5, 8.0, -4.5]),
        s("fence_west", "fence sections", [LOT_X0, GROUND, 0.0], [LOT_X0 + 0.1, 1.8, LOT_Z1]),
        s("fence_east", "fence sections", [LOT_X1 - 0.1, GROUND, 0.0], [LOT_X1, 1.8, LOT_Z1]),
        s("fence_back", "fence sections", [LOT_X0, GROUND, LOT_Z1 - 0.1], [LOT_X1, 1.8, LOT_Z1]),
        s("neighbor_west", "neighbouring house in fog", [-22.0, GROUND, -2.0], [-9.0, 7.5, 9.0]),
        s("neighbor_east", "neighbouring house in fog", [23.0, GROUND, -2.0], [36.0, 7.5, 9.0]),
        s("street_parked_car", "parked car at the curb", [8.0, GROUND, -12.4], [10.0, 1.5, -9.2]),
    ]
}
