-- Your SQL goes here

ALTER TABLE "tasks" DROP COLUMN "tags";
ALTER TABLE "tasks" ADD COLUMN "tags" TEXT[];


