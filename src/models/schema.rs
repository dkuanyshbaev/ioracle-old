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
