-- This file should undo anything in `up.sql`

ALTER TABLE "tasks" DROP COLUMN "tags";
ALTER TABLE "tasks" ADD COLUMN "tags" TEXT NOT NULL[];


