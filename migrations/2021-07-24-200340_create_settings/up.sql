create table settings (
    id uuid primary key default gen_random_uuid(),
    data jsonb not null,
    user_id text unique not null check (user_id != ''),
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);
