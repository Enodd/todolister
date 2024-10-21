-- Your SQL goes here
create table users (
    id uuid primary key,
    name varchar(40) not null,
    login varchar(40) not null,
    password varchar(32) not null,
    birthdate timestamptz not null
);

create table tasks (
    id uuid primary key,
    title varchar(40) not null,
    description text not null, 
    create_date timestamptz not null,
    due_date timestamptz,
    tags text not null [] not null,
    user_id uuid not null,
    constraint user_id
        foreign key(user_id)
            references users(id)
);