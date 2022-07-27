CREATE TYPE action AS ENUM ('deny', 'read', 'modify', 'admin');

ALTER TABLE users ADD COLUMN default_access action not null default 'modify';

CREATE TABLE IF NOT EXISTS permissions (
    id serial primary key,
    applies_to text references users (username),
    path text not null,
    action action not null,
    affects_children boolean not null default false,
    unique (applies_to, path, action, affects_children)
);
