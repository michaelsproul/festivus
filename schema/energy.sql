CREATE TABLE energy (
    day DATE PRIMARY KEY NOT NULL,
    energy BIGINT NOT NULL CHECK (energy >= 0)
);
