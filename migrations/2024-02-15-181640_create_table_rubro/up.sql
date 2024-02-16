-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "rubro" (
	"id"	SERIAL UNIQUE,
	"name"		VARCHAR,
	"score"		REAL,
	-- FOREIGN KEY("ingrediente_id") REFERENCES "ingrediente"("id"),
	PRIMARY KEY("id")
);