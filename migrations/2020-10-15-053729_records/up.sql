create table if not exists records (
    id integer not null primary key,
    uuid text not null,
    email text not null,
    question text not null,
    answer text not null,
    hexagram text not null,
    related text not null
)
