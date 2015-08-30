CREATE TABLE power (
    time TIMESTAMP PRIMARY KEY NOT NULL,
    peak INT NOT NULL CHECK (peak >= 0),
    offpeak INT NOT NULL CHECK (offpeak >= 0)
);
