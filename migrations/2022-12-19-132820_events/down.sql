-- This file should undo anything in `up.sql`
-- Note the order in which these are dropped
-- The table events is dropped before the enums since it depends on them
-- Otherwise, attempting to run this code would cause an error because of referential integrity
DROP TABLE events;

DROP TYPE visibility_type;
DROP TYPE recurrence_type;
