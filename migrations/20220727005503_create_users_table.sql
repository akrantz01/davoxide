CREATE TABLE IF NOT EXISTS users (
    username text not null primary key,
    name text not null,
    access_token text default null
);
