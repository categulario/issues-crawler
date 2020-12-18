-- Your SQL goes here
create table issues (
    id bigserial primary key,
    project_id bigint not null references projects(id),
    author varchar(256) not null,
    url varchar(4096) not null,
    title varchar(256) not null,
    content text null
)
