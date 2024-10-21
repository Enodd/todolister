-- This file should undo anything in `up.sql`
ALTER TABLE "tasks" DROP COLUMN "user_id";
ALTER TABLE "tasks" DROP COLUMN "create_date";
ALTER TABLE "tasks" DROP COLUMN "due_date";
ALTER TABLE "tasks" DROP COLUMN "tags";
ALTER TABLE "tasks" ADD COLUMN "creator" UUID NOT NULL;
ALTER TABLE "tasks" ADD COLUMN "create_date" TIMESTAMP NOT NULL;
ALTER TABLE "tasks" ADD COLUMN "due_date" TIMESTAMP;
ALTER TABLE "tasks" ADD COLUMN "tags" TEXT[];

ALTER TABLE "users" DROP COLUMN "birthdate";
ALTER TABLE "users" ADD COLUMN "birthdate" TIMESTAMP NOT NULL;

DROP TABLE IF EXISTS "sessions";
