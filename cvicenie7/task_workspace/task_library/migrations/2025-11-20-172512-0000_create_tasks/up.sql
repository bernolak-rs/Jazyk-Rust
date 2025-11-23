-- Your SQL goes here
CREATE TABLE tasks (
  id integer primary key autoincrement not null,
  nazov char(50) not null,
  popis text not null,
  priorita integer not null,
  planovany_zaciatok char(10) not null,
  skutocny_zaciatok char(10),
  planovane_trvanie integer not null,
  skutocne_trvanie integer
)
