-- Your SQL goes here

create table users ( -- plural as `user` is a reserved keyword.
  id serial primary key,
  ext_id UUID unique not null, -- id used externally(shown to the client)
  username varchar(20) not null,
  password text not null,
  name varchar(100), -- fullname
  -- https://dba.stackexchange.com/a/37021/177682
  email varchar(320) unique not null, -- not null until we implement anonymous user
  created_at timestamp not null,
  updated_at timestamp
);


create table app (
  id serial primary key,
  -- https://stackoverflow.com/q/52414414/7699859
  ext_id UUID unique not null, -- id used externally(shown to the client)
  name varchar(100) not null,
  domain varchar(500) not null, -- how to deal with domain longer than that?
  owner int references users on delete cascade not null,
  is_deleted boolean default false not null,
  created_at timestamp not null,
  updated_at timestamp
);



create table page (
  id serial primary key,
  ext_id UUID unique not null, -- id used externally(shown to the client)
  app_id int references app on delete cascade not null,
  slug text not null, -- slug is used instead of complete url because for blog a user can change the tld or can add it under subdomain easliy.
  created_at timestamp not null,
  updated_at timestamp
);

-- preferred over table unique constraint
CREATE UNIQUE INDEX app_slug_uniq_idx ON page(app_id, slug);

create table comment (
  id serial primary key,
  ext_id UUID unique not null, -- id used externally(shown to the client)
  user_id int references users on delete cascade not null,
  page_id int references page on delete cascade not null,
  parent_id int references comment,
  content text not null,
  is_deleted boolean default false not null,
  created_at timestamp not null,
  updated_at timestamp
);
