SET statement_timeout TO '10s';

CREATE TABLE IF NOT EXISTS activities (
    type TEXT PRIMARY KEY,
    offers JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS destinations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    pictures JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type TEXT NOT NULL,
    destination UUID REFERENCES destinations (id),
    date_from TIMESTAMP WITH TIME ZONE NOT NULL,
    date_to TIMESTAMP WITH TIME ZONE NOT NULL,
    offers JSONB NOT NULL,
    base_price BIGINT NOT NULL,
    is_favorite BOOLEAN NOT NULL
);
