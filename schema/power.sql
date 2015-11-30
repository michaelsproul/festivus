-- Note: Ensure that Postgres's timezone is set to your true local timezone.
CREATE TABLE power (
    time TIMESTAMP WITH TIME ZONE PRIMARY KEY NOT NULL,
    total INT NOT NULL CHECK (total >= 0),
    hot_water INT NOT NULL CHECK (hot_water >= 0),
    solar INT NOT NULL CHECK (solar >= 0)
);
