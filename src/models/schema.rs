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
    hexagrams (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        description -> Text,
    }
}

table! {
    settings (id) {
        id -> Integer,
        file_name -> Text,
        heaven_pin -> Integer,
        heaven_colour -> Text,
        cloud_pin -> Integer,
        cloud_colour -> Text,
        sun_pin -> Integer,
        sun_colour -> Text,
        wind_pin -> Integer,
        wind_colour -> Text,
        thunder_sound -> Text,
        thunder_colour -> Text,
        water_pin -> Integer,
        water_colour -> Text,
        mountain_sound -> Text,
        mountain_colour -> Text,
        earth_pin -> Integer,
        earth_colour -> Text,
        multiply -> Float,
        bias -> Float,
        threshold -> Float,
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
    hexagrams,
    settings,
    trigrams,
);
