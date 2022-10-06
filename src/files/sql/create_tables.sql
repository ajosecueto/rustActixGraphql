CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table public.preferences
(
    preference_id uuid        default uuid_generate_v4() not null,
    created_at    timestamptz default now() not null,
    status        boolean     default true  not null
);

create table public.preference_locales
(
    locale_id     uuid default uuid_generate_v4() not null,
    description   varchar(50)                     not null,
    locale        varchar(5)                      not null,
    video_url     varchar(200),
    preference_id uuid                            not null
);