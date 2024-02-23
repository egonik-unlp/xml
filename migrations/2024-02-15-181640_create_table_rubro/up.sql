-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "rubros" (
	"id"	SERIAL UNIQUE,
	"name"		VARCHAR UNIQUE,
	"score"		REAL,
	-- FOREIGN KEY("ingrediente_id") REFERENCES "ingrediente"("id"),
	PRIMARY KEY("id")
);