create table if not exists hexagrams (
    id integer not null primary key,
    binary text not null,
    king_wen_order integer not null,
    shao_yong_order integer not null,
    gua text not null,
    pin_yin text not null,
    character text not null,
    wilheim text not null,
    huang text not null,
    hatcher text not null,
    no2do text not null,
    inner_ba_gua text not null,
    outer_ba_gua text not null,
    host_yao text not null
);

-- create table if not exists hexagrams (
--     id integer not null primary key,
--     name text not null,
--     image text not null,
--     description text not null
-- );
-- insert or ignore into hexagrams (id, name, image, description) values (1, "", "1.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (2, "", "2.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (3, "", "3.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (4, "", "4.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (5, "", "5.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (6, "", "6.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (7, "", "7.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (8, "", "8.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (9, "", "9.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (10, "", "10.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (11, "", "11.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (12, "", "12.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (13, "", "13.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (14, "", "14.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (15, "", "15.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (16, "", "16.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (17, "", "17.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (18, "", "18.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (19, "", "19.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (20, "", "20.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (21, "", "21.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (22, "", "22.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (23, "", "23.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (24, "", "24.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (25, "", "25.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (26, "", "26.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (27, "", "27.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (28, "", "28.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (29, "", "29.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (30, "", "30.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (31, "", "31.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (32, "", "32.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (33, "", "33.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (34, "", "34.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (35, "", "35.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (36, "", "36.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (37, "", "37.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (38, "", "38.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (39, "", "39.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (40, "", "40.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (41, "", "41.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (42, "", "42.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (43, "", "43.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (44, "", "44.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (45, "", "45.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (46, "", "46.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (47, "", "47.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (48, "", "48.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (49, "", "49.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (50, "", "50.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (51, "", "51.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (52, "", "52.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (53, "", "53.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (54, "", "54.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (55, "", "55.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (56, "", "56.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (57, "", "57.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (58, "", "58.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (59, "", "59.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (60, "", "60.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (61, "", "61.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (62, "", "62.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (63, "", "63.png", "");
-- insert or ignore into hexagrams (id, name, image, description) values (64, "", "64.png", "");
