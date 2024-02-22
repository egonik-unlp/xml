
CREATE TABLE IF NOT EXISTS "productos" (
	"id"	SERIAL UNIQUE,
	"codigo"	VARCHAR,
	"categoria"	CAT NOT NULL,
	"descripcion"	VARCHAR,
	"presentacion"	VARCHAR,
	"rubro_id"	INTEGER NOT NULL REFERENCES "rubros"("id"),
	"observaciones"	VARCHAR,
	"numero_ingredientes"	INT,
	"score" REAL,
	PRIMARY KEY("id" )
);