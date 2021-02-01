-- Your SQL goes here
create table user
(
    id int not null,
    name varchar(255) not null,
    cin varchar(255) not null,
    constraint user_pk
        primary key (id)
);

