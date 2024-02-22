CREATE TABLE IF NOT EXISTS "ingredientes_productos" (
	"id"	SERIAL UNIQUE,
	"producto_id"	INTEGER NOT NULL REFERENCES "productos"("id"),
	"ingrediente_id"	INTEGER REFERENCES "ingredientes"("id"),
	"aditivo_id"	INTEGER REFERENCES "aditivos"("id"),
	PRIMARY KEY("id")
	);