CREATE TABLE tokens (
  token TEXT PRIMARY KEY NOT NULL,
  flags INTEGER NOT NULL DEFAULT 0 CHECK(flags >= 0)
);

INSERT INTO tokens (`token`, `flags`) VALUES ('A', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('B', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('C', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('D', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('E', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('F', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('G', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('H', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('I', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('J', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('K', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('L', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('M', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('N', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('O', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('P', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('Q', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('R', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('S', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('T', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('U', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('V', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('W', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('X', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('Y', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('Z', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('0', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('1', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('2', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('3', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('4', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('5', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('6', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('7', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('8', 3);
INSERT INTO tokens (`token`, `flags`) VALUES ('9', 3);
