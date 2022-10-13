CREATE TABLE tokens (
  token TEXT PRIMARY KEY NOT NULL,
  flags UNSIGNED INT4 NOT NULL
);

INSERT INTO tokens (`token`, `flags`) VALUES ('BTC', 0);
INSERT INTO tokens (`token`, `flags`) VALUES ('ART', 0);
