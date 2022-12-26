CREATE TABLE tokens (
  token TEXT PRIMARY KEY NOT NULL,
  flags INTEGER NOT NULL DEFAULT 0 CHECK(flags >= 0)
);

INSERT INTO tokens (`token`, `flags`) VALUES ('BTC', 1);
