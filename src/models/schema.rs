table! {
    answers (id) {
        id -> Integer,
        uuid -> Text,
        email -> Text,
        question -> Text,
        answer -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    bindings (id) {
        id -> Integer,
        default_colour -> Text,
        resting_colour -> Text,
        heaven_pin -> Integer,
        heaven_colour -> Text,
        cloud_pin -> Integer,
        cloud_colour -> Text,
        sun_colour -> Text,
        thunder_colour -> Text,
        wind_pin -> Integer,
        wind_colour -> Text,
        water_pin -> Integer,
        water_colour -> Text,
        mountain_pin -> Integer,
        mountain_colour -> Text,
        multiply -> Text,
        bias -> Text,
        threshold -> Text,
        led_freq -> Integer,
        led_cycles -> Text,
        fan_freq -> Integer,
        fan_cycles -> Text,
    }
}

table! {
    hexagrams (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        description -> Text,
    }
}

table! {
    trigrams (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    answers,
    bindings,
    hexagrams,
    trigrams,
);
