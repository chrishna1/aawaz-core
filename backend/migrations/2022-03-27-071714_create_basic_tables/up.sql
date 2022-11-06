-- Your SQL goes here

create table users ( -- plural as `user` is a reserved keyword.
  id serial primary key,
  ext_id UUID unique default gen_random_uuid () not null, -- id used externally(shown to the client)
  username varchar(50) not null,
  password text,
  name varchar(100), -- fullname
  -- https://dba.stackexchange.com/a/37021/177682
  email varchar(320) unique not null, -- not null until we implement anonymous user
  created_at timestamp default now() not null,
  updated_at timestamp
);


-- TODO - use enum please.. requires diesel upgrade and an external crate..
-- create type oauth_provider as enum('apple', 'google', 'github', 'gitlab', 'twitter');
-- provider oauth_provider not null,

create table oauth (
    id serial primary key,
    user_id int references users on delete cascade not null,
    provider_id text not null, -- github_id, twitter_id, etc
    -- it is more reliable than email!! latter can be changed!! which can create a new account while logging in!!
    provider varchar(20) not null,
    access_token text not null,
    refresh_token text,
    expires_at int,
    created_at timestamp default now() not null,
    updated_at timestamp
);

create unique index lower_case_username ON users (lower(username));
create unique index lower_case_email ON users (lower(email));

-- store oauth states here temporarily.. could have been stored directly in state(base64 encoded json) and passed around but that seems less secure
create table oauth_states(
    id serial primary key,
    state_id text unique not null,
    state_data JSONB not null
);


create table app (
  id serial primary key,
  -- https://stackoverflow.com/q/52414414/7699859
  ext_id UUID unique default gen_random_uuid () not null, -- id used externally(shown to the client)
  name varchar(100) not null,
  domain varchar(500) not null, -- how to deal with domain longer than that?
  owner int references users on delete cascade not null,
  is_deleted boolean default false not null,
  created_at timestamp default now() not null,
  updated_at timestamp
);

create unique index app_domain_uniq_idx ON app (lower(domain));


create table page (
  id serial primary key,
  ext_id UUID unique default gen_random_uuid () not null, -- id used externally(shown to the client)
  app_id int references app on delete cascade not null,
  path text not null,
  title text null, -- useful in admin panel(for approving the comment), sending notification, etc.
  created_at timestamp default now() not null,
  updated_at timestamp
);

-- preferred over table unique constraint
CREATE UNIQUE INDEX app_path_uniq_idx ON page(app_id, path);

create table comment (
  id serial primary key,
  ext_id UUID unique default gen_random_uuid () not null, -- id used externally(shown to the client)
  user_id int references users on delete cascade not null,
  page_id int references page on delete cascade not null,
  parent_id int references comment,
  content text not null,
  is_deleted boolean default false not null,
  created_at timestamp default now() not null,
  updated_at timestamp
);


SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('app');
SELECT diesel_manage_updated_at('page');
SELECT diesel_manage_updated_at('comment');
