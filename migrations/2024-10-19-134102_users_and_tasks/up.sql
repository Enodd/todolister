-- Your SQL goes here
create table users (
    id uuid primary key,
    name varchar(40) not null,
    login varchar(40) not null,
    password varchar(32) not null,
    birthdate timestamp not null
);

create table tasks (
    id uuid primary key,
    title varchar(40) not null,
    description text not null, 
    create_date timestamp not null,
    due_date timestamp,
    tags text[],
    creator uuid not null,
    constraint creator
        foreign key(creator)
            references users(id)
);