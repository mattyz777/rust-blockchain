-- supabase
create table if not exists public.t_users (
    id         bigserial    primary key,
    username   varchar(100) not null,
    password   varchar(255) not null,
    is_deleted boolean      not null default false,
    created_at timestamptz  not null default now(),
    updated_at timestamptz           default null
);



