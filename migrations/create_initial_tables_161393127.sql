drop table if exists todo_lists;
drop table if exists todo_items;

create table todo_lists (
  id serial primary key,
  title varchar(150)
);

create table todo_items (
  id serial primary key,
  title varchar(150),
  checked boolean not null default false,
  list_id integer not null,
  foreign key (list_id) references todo_lists(id)
);

insert into todo_lists (title) values ('list 1'), ('list 2');
insert into todo_items (title, list_id)
  values ('item 1', 1), ('item 2', 1), ('item 1', 2), ('item 2', 2);
