CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table users (
    id uuid primary key default uuid_generate_v4(),
	username varchar (50) unique not null,
	password varchar (128) not null,
	email varchar (355) unique not null,
	created timestamp with time zone not null default current_timestamp,
	enabled bool not null default true
);
