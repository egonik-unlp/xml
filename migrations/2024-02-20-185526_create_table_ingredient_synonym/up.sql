-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "sinonimo_ingredientes" (
	"id"	SERIAL UNIQUE,
	"ingrediente_id"	INTEGER NOT NULL,
	"sinonimo"	VARCHAR UNIQUE ,
	"existe"	BOOLEAN,
	FOREIGN KEY("ingrediente_id") REFERENCES "ingredientes"("id"),
	PRIMARY KEY("id")
);