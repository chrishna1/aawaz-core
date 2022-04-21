-- Your SQL goes here

create table users ( -- plural as `user` is a reserved keyword.
  id serial primary key,
  username varchar(20) not null,
  password text not null,
  name varchar(100), -- fullname
  email text unique not null, -- not null until we implement anonymous user
  created_at timestamp not null,
  updated_at timestamp
);


create table app (
  id serial primary key,
  name varchar(100) not null,
  domain varchar(500) not null, -- how to deal with domain longer than that?
  owner int references users on delete cascade not null,
  is_deleted boolean default false not null,
  created_at timestamp not null,
  updated_at timestamp
);



create table page (
  id serial primary key,
  app_id int references app on delete cascade not null,
  slug text not null, -- slug is used instead of complete url because for blog a user can change the tld or can add it under subdomain easliy.
  created_at timestamp not null,
  updated_at timestamp
);



create table comment (
  id serial primary key,
  user_id int references users on delete cascade not null,
  page_id int references page on delete cascade not null,
  parent_id int references comment,
  content text not null,
  is_deleted boolean default false not null,
  created_at timestamp not null,
  updated_at timestamp
);
