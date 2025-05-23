-- Your SQL goes here
CREATE TABLE "users"(
	"id" VARCHAR NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"first_name" VARCHAR NOT NULL,
	"last_name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"role" VARCHAR NOT NULL,
	"password_hash" VARCHAR NOT NULL,
	"is_connected" BOOL NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"last_connected_at" TIMESTAMP NOT NULL
);

