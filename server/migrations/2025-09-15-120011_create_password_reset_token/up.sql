-- Your SQL goes here

CREATE TABLE "password_reset_tokens"(
	"user_id" VARCHAR NOT NULL PRIMARY KEY,
	"token_hash" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);


