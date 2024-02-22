CREATE TABLE IF NOT EXISTS "aditivos" (
	"id"	SERIAL UNIQUE,
	"categoria"	CAT,
	"name"	VARCHAR,
	"toxicity"	REAL,
	"sinonimos"	VARCHAR,
	"info_to_report"	VARCHAR,
	"observaciones"	VARCHAR,
	PRIMARY KEY("id" )
);
-- Your SQL goes here
