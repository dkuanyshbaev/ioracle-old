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
        multiply -> Float,
        bias -> Float,
        threshold -> Float,
    }
}

table! {
    hexagrams (id) {
        id -> Integer,
        binary -> Text,
        king_wen_order -> Integer,
        shao_yong_order -> Integer,
        gua -> Text,
        pin_yin -> Text,
        character -> Text,
        wilheim -> Text,
        huang -> Text,
        hatcher -> Text,
        no2do -> Text,
        inner_ba_gua -> Text,
        outer_ba_gua -> Text,
        host_yao -> Text,
    }
}

table! {
    records (id) {
        id -> Integer,
        uuid -> Text,
        email -> Text,
        question -> Text,
        answer -> Text,
        hexagram -> Text,
    }
}

table! {
    trigrams (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        binary -> Text,
        no -> Text,
        wen -> Text,
        host -> Text,
        element -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    answers,
    bindings,
    hexagrams,
    records,
    trigrams,
);
