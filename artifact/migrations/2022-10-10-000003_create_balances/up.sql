CREATE TABLE balances (
  address TEXT NOT NULL,
  token TEXT NOT NULL,
  quantity INTEGER NOT NULL DEFAULT 0 CHECK(quantity >= 0 AND quantity <= 10000),
  PRIMARY KEY (address, token),
  FOREIGN KEY (address) REFERENCES addresses(address),
  FOREIGN KEY (token) REFERENCES tokens(token)
);

CREATE INDEX ix_balances_address ON balances (address);
CREATE INDEX ix_balances_token ON balances (token);
