create table if not exists bindings (
    id integer not null primary key,
    default_colour text not null,
    resting_colour text not null,
    heaven_pin integer not null,
    heaven_colour text not null,
    cloud_pin integer not null,
    cloud_colour text not null,
    sun_colour text not null,
    thunder_colour text not null,
    wind_pin integer not null,
    wind_colour text not null,
    water_pin integer not null,
    water_colour text not null,
    mountain_pin integer not null,
    mountain_colour text not null,
    multiply real not null,
    bias real not null,
    threshold real not null
);

insert or ignore into bindings (
    id,
    default_colour,
    resting_colour,
    heaven_pin,
    heaven_colour,
    cloud_pin,
    cloud_colour,
    sun_colour,
    thunder_colour,
    wind_pin,
    wind_colour,
    water_pin,
    water_colour,
    mountain_pin,
    mountain_colour,
    multiply,
    bias,
    threshold
) values (
    1, -- id
    "#FFFFFF", -- default_colour
    "#FFFFFF", -- resting_colour
    0, -- heaven_pin
    "#FFFFFF", -- heaven_colour
    0, -- cloud_pin
    "#FFFFFF", -- cloud_colour
    "#FFFFFF", -- sun_colour
    "#FFFFFF", -- thunder_colour
    0, -- wind_pin
    "#FFFFFF", -- wind_colour
    0, -- water_pin
    "#FFFFFF", -- water_colour
    0, -- mountain_pin
    "#FFFFFF", -- mountain_colour
    0.0, -- multiply
    0.0, -- bias
    0.0 -- threshold
);
