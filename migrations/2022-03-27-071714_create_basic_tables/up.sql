-- Your SQL goes here

create table users ( -- plural as `user` is a reserved keyword.
  id serial primary key,
  handle varchar(20),
  password text not null,
  name varchar(20), -- fullname
  email text unique,
  created_at timestamp,
  updated_at timestamp
);

insert into users (handle, password) values ('admin', '12345');
insert into users (handle, password) values ('user_albela', '12345');
insert into users (handle, password) values ('user_bhola', '12345');
insert into users (handle, password) values ('user_chhotkin', '12345');
insert into users (handle, password) values ('user_damodar', '12345');
insert into users (handle, password) values ('user_enami', '12345');
insert into users (handle, password) values ('user_farsi', '12345');
insert into users (handle, password) values ('user_gayatri', '12345');


create table app (
  id serial primary key,
  name varchar(100) not null,
  domain text not null,
  owner int references users on update cascade on delete cascade not null,
  deleted boolean default false not null,
  created_at timestamp,
  updated_at timestamp
);


insert into app (name, domain, owner) values ('albela blog"s comment', 'https://albela.com', 2);


create table page (
  id serial primary key,
  app_id int references app on update cascade on delete cascade not null,
  slug text not null, -- slug is used instead of complete url because for blog a user can change the tld or can add it under subdomain easliy.
  created_at timestamp,
  updated_at timestamp
);

insert into page (app_id, slug) values (1, '/first-blog');


create table comment (
  id serial primary key,
  created_by int references users on update cascade on delete cascade not null,
  page_id int references page on update cascade on delete cascade not null,
  parent_id int references comment on update cascade on delete cascade,
  content text not null,
  deleted boolean default false not null,
  created_at timestamp,
  updated_at timestamp
);

insert into comment (created_by, page_id, content) values (1, 1, 'pehla comment');
insert into comment (created_by, page_id, content) values (2, 1, 'dusra comment');
