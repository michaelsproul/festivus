-- Note: Ensure that Postgres's timezone is set to your true local timezone.
CREATE TABLE power (
    time TIMESTAMP WITH TIME ZONE PRIMARY KEY NOT NULL,
    ch1 INT NOT NULL CHECK (ch1 >= 0),
    ch2 INT NOT NULL CHECK (ch2 >= 0),
    ch3 INT NOT NULL CHECK (ch3 >= 0)
);
