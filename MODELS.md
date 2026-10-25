# Restless Night: какие 3D-модели нужны

Дом: два этажа, гараж на одну машину и подвал под гаражом и прихожей (около 7.4 x 10.3 м).

Координаты (метры): X вдоль улицы (0 = западная стена гаража), Z от улицы вглубь участка (фасад основного объёма Z = 0, улица на отрицательном Z), Y вверх. Пол 1 этажа Y = 0, пол 2 этажа Y = 3.0, пол подвала Y = -2.7, земля снаружи Y = -0.36.

Формат: GLB (glTF 2.0), 1 единица = 1 метр, материалы metallic-roughness. Если модель в другом масштабе, пришлите её, я подберу scale и точку привязки по размерам.

В игре F3 показывает жёлтыми рамками все свободные места, [ и ] меняют экспозицию, F5 переключает качество, E открывает и закрывает двери.

## Прихожая

Сразу за входной дверью (X 3.7..7.7, Z 1.8..5.0) оставлено свободное место: вешалка и обувница у западной стены, скамейка, подставка для зонтов и консоль с зеркалом у восточной стены. Лестница начинается дальше в доме (Z 5.0).

## Уже стоят (ваши модели)

| Модель | Где |
|---|---|
| modern__sofa.glb | гостиная, у стены прихожей (X 8.5, Z 1.95), лицом на восток |
| trim_sheet__example_dresser.glb | комод в прихожей (X 7.7, Z 2.2) и комод в главной спальне (Y 3.0) |
| bed.glb | главная спальня |
| wooden_kitchen.glb | кухня, юго-восточный угол основного объёма |

street_light.glb лежит в папке, но не поставлен: место под фонарь есть в списке ниже. fairheaven_sw_84_-_low_poly_model.glb не используется.

## Свободные места

Колонки: имя места, что нужно, нижний угол (x, y, z) и размер (X x Y x Z) в метрах.

### 1 этаж и гараж

| Место | Что нужно | Угол | Размер |
|---|---|---|---|
| living_tv_console | TV on low console 1.4 x 0.45 m | 14.60, 0.00, 3.70 | 0.58 x 1.10 x 1.40 |
| living_coffee_table | coffee table 1.1 x 0.55 m | 9.70, 0.00, 1.50 | 1.10 x 0.45 x 0.70 |
| living_armchair | armchair | 11.40, 0.00, 3.60 | 0.90 x 0.90 x 0.90 |
| living_rug | rug 3 x 2.4 m | 8.90, 0.00, 0.70 | 3.00 x 0.02 x 2.60 |
| living_side_table | side table with lamp | 8.00, 0.00, 3.60 | 0.50 x 0.60 x 0.50 |
| living_bookshelf | low bookshelf | 9.00, 0.00, 4.90 | 1.20 x 1.30 x 0.45 |
| entry_coat_rack | coat hooks or coat rack | 3.70, 0.00, 1.90 | 0.40 x 1.90 x 1.20 |
| entry_shoe_rack | shoe rack | 3.70, 0.00, 3.20 | 0.40 x 0.50 x 1.10 |
| entry_bench | small bench | 4.30, 0.00, 4.20 | 1.20 x 0.50 x 0.50 |
| entry_mirror_console | console table with mirror | 7.30, 0.00, 2.80 | 0.35 x 1.80 x 0.60 |
| dining_table | dining table 1.5 x 0.9 m | 10.60, 0.00, 6.30 | 1.50 x 0.76 x 0.90 |
| dining_chair_1 | chair | 10.80, 0.00, 5.78 | 0.45 x 0.90 x 0.47 |
| dining_chair_2 | chair | 11.45, 0.00, 5.78 | 0.45 x 0.90 x 0.47 |
| dining_chair_3 | chair | 10.80, 0.00, 7.25 | 0.45 x 0.90 x 0.50 |
| dining_chair_4 | chair | 11.45, 0.00, 7.25 | 0.45 x 0.90 x 0.50 |
| dining_sideboard | sideboard | 12.80, 0.00, 5.62 | 1.20 x 0.90 x 0.45 |
| kitchen_fridge | refrigerator | 9.20, 0.00, 8.00 | 0.65 x 1.80 x 0.60 |
| bath_tub | bathtub 1.5 x 0.7 m | 7.40, 0.00, 9.60 | 1.50 x 0.55 x 0.70 |
| bath_toilet | toilet | 8.30, 0.00, 8.10 | 0.60 x 0.80 x 0.70 |
| bath_sink | vanity with mirror | 7.70, 0.00, 8.10 | 0.80 x 0.90 x 0.50 |
| utility_washer | washing machine | 0.60, 0.00, 9.55 | 0.65 x 0.95 x 0.65 |
| utility_dryer | dryer | 1.30, 0.00, 9.55 | 0.60 x 0.95 x 0.65 |
| utility_shelf | shelving unit | 0.05, 0.00, 6.20 | 0.45 x 1.90 x 1.40 |
| utility_water_heater | water heater | 0.05, 0.00, 8.00 | 0.45 x 1.50 x 0.60 |
| garage_car | family car 4.5 x 1.8 m | 1.50, 0.00, -0.40 | 1.90 x 1.50 x 4.50 |
| garage_workbench | workbench | 1.60, 0.00, 4.75 | 1.80 x 0.90 x 0.60 |
| garage_shelves | metal shelving | 3.00, 0.00, 0.50 | 0.45 x 1.90 x 1.80 |
| garage_bins | storage bins and tools | 0.10, 0.00, -0.90 | 0.80 x 1.20 x 2.30 |

### Подвал

| Место | Что нужно | Угол | Размер |
|---|---|---|---|
| basement_shelves | metal shelving | 0.10, -2.70, 2.20 | 0.40 x 1.90 x 3.60 |
| basement_boxes | crates and boxes | 3.00, -2.70, 6.00 | 2.20 x 0.90 x 1.60 |
| basement_furnace | furnace or boiler | 5.80, -2.70, 0.20 | 1.50 x 1.60 x 1.00 |
| basement_workshop | old table or workbench | 5.00, -2.70, 9.00 | 2.30 x 0.90 x 1.20 |

### 2 этаж

| Место | Что нужно | Угол | Размер |
|---|---|---|---|
| br2_bed | single bed 0.95 x 2.0 m | 6.50, 3.00, 0.20 | 0.95 x 0.60 x 2.00 |
| br2_desk | desk and chair | 8.30, 3.00, 0.15 | 0.85 x 0.78 x 1.15 |
| br2_wardrobe | wardrobe 1.2 x 0.55 m | 8.10, 3.00, 3.55 | 1.20 x 2.00 x 0.55 |
| study_desk | desk 1.4 x 0.7 m | 11.40, 3.00, 0.15 | 1.40 x 0.76 x 0.70 |
| study_chair | office chair | 11.70, 3.00, 0.90 | 0.60 x 1.00 x 0.60 |
| study_bookcase | bookcase | 13.20, 3.00, 3.60 | 1.50 x 2.00 x 0.55 |
| mb_nightstand_a | nightstand | 10.70, 3.00, 9.70 | 0.50 x 0.60 x 0.55 |
| mb_nightstand_b | nightstand | 12.50, 3.00, 9.70 | 0.50 x 0.60 x 0.55 |
| mb_wardrobe | wardrobe 1.4 x 0.6 m | 8.45, 3.00, 6.60 | 0.60 x 2.10 x 1.40 |
| closet_shelves | closet rails and shelves | 6.50, 3.00, 8.60 | 1.60 x 2.00 x 1.65 |
| bath2_tub | tub or shower 1.5 x 0.75 m | 6.50, 3.00, 6.00 | 1.40 x 0.60 x 0.75 |
| bath2_toilet | toilet | 7.40, 3.00, 7.60 | 0.70 x 0.80 x 0.75 |
| bath2_sink | vanity with mirror | 6.50, 3.00, 7.70 | 0.80 x 0.90 x 0.65 |
| upper_hall_console | narrow console or bookcase | 3.70, 3.00, 9.20 | 0.50 x 1.80 x 1.00 |

### Улица и участок

| Место | Что нужно | Угол | Размер |
|---|---|---|---|
| front_door_mat | door mat | 4.60, 0.00, 0.50 | 1.20 x 0.03 x 0.70 |
| porch_chair | porch chair or planter | 3.80, 0.00, 0.20 | 0.60 x 0.90 x 0.60 |
| mailbox | mailbox on post | 2.20, -0.36, -7.00 | 0.40 x 1.56 x 0.30 |
| street_lamp | street light | 6.00, -0.36, -8.80 | 0.40 x 5.36 x 0.30 |
| trash_bins | two wheeled bins | 15.80, -0.36, 5.00 | 1.00 x 1.36 x 1.40 |
| ac_unit | air-conditioner condenser | 15.80, -0.36, 8.40 | 0.90 x 1.06 x 1.00 |
| front_tree | bare deciduous tree | 8.50, -0.36, -5.50 | 1.00 x 8.36 x 1.00 |
| fence_west | fence sections | -5.50, -0.36, 0.00 | 0.10 x 2.16 x 20.60 |
| fence_east | fence sections | 19.40, -0.36, 0.00 | 0.10 x 2.16 x 20.60 |
| fence_back | fence sections | -5.50, -0.36, 20.50 | 25.00 x 2.16 x 0.10 |
| neighbor_west | neighbouring house in fog | -22.00, -0.36, -2.00 | 13.00 x 7.86 x 11.00 |
| neighbor_east | neighbouring house in fog | 23.00, -0.36, -2.00 | 13.00 x 7.86 x 11.00 |
| street_parked_car | parked car at the curb | 8.00, -0.36, -12.40 | 2.00 x 1.86 x 3.20 |
