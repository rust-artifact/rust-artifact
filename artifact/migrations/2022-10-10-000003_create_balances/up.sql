CREATE TABLE balances (
  address TEXT NOT NULL,
  token TEXT NOT NULL,
  quantity UNSIGNED INT8 NOT NULL,
  PRIMARY KEY (address, token)
);

CREATE INDEX ix_balances_address ON balances (address);
CREATE INDEX ix_balances_token ON balances (token);
