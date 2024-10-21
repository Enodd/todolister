-- Your SQL goes here
ALTER TABLE "tasks" DROP COLUMN "creator";
ALTER TABLE "tasks" DROP COLUMN "create_date";
ALTER TABLE "tasks" DROP COLUMN "due_date";
ALTER TABLE "tasks" DROP COLUMN "tags";
ALTER TABLE "tasks" ADD COLUMN "user_id" UUID NOT NULL;
ALTER TABLE "tasks" ADD COLUMN "create_date" TIMESTAMPTZ NOT NULL;
ALTER TABLE "tasks" ADD COLUMN "due_date" TIMESTAMPTZ;
ALTER TABLE "tasks" ADD COLUMN "tags" TEXT[] NOT NULL;

ALTER TABLE "users" DROP COLUMN "birthdate";
ALTER TABLE "users" ADD COLUMN "birthdate" TIMESTAMPTZ NOT NULL;

CREATE TABLE "sessions"(
	"id" UUID NOT NULL PRIMARY KEY,
	"session_data" TEXT NOT NULL,
	"expires_at" TIMESTAMPTZ NOT NULL
);

