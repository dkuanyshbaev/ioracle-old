create table if not exists trigrams (
    id integer not null primary key,
    name text not null,
    image text not null,
    description text not null
);

insert or ignore into trigrams (id, name, image, description) values (1, "", "1.png", "");
insert or ignore into trigrams (id, name, image, description) values (2, "", "2.png", "");
insert or ignore into trigrams (id, name, image, description) values (3, "", "3.png", "");
insert or ignore into trigrams (id, name, image, description) values (4, "", "4.png", "");
insert or ignore into trigrams (id, name, image, description) values (5, "", "5.png", "");
insert or ignore into trigrams (id, name, image, description) values (6, "", "6.png", "");
insert or ignore into trigrams (id, name, image, description) values (7, "", "7.png", "");
insert or ignore into trigrams (id, name, image, description) values (8, "", "8.png", "");
