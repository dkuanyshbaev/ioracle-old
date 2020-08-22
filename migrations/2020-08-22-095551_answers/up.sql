create table if not exists answers (
    id integer not null primary key,
    uuid text not null,
    email text not null,
    question text not null,
    answer text not null,
    created_at datetime default current_timestamp
)
