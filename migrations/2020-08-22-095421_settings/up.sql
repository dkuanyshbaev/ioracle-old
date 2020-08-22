create table if not exists settings (
    id integer not null primary key,
    file_name text not null,
    heaven_pin integer not null,
    heaven_colour text not null,
    cloud_pin integer not null,
    cloud_colour text not null,
    sun_pin integer not null,
    sun_colour text not null,
    wind_pin integer not null,
    wind_colour text not null,
    thunder_sound text not null,
    thunder_colour text not null,
    water_pin integer not null,
    water_colour text not null,
    mountain_sound text not null,
    mountain_colour text not null,
    earth_pin integer not null,
    earth_colour text not null,
    multiply real not null,
    bias real not null,
    threshold real not null
)

-- insert or ignore into settings (id, name, image, description) values (1, "", "1.png", "");
