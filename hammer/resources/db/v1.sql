BEGIN;
CREATE TABLE if NOT EXISTS spells
(
  id integer PRIMARY KEY,
  name text,
  class text,
  school text,
  tag1 integer,
  tag2 integer
);
COMMIT;
