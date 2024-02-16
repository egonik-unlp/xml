CREATE TABLE IF NOT EXISTS "ingrediente" (
	"id"	SERIAL UNIQUE,
	"categoria"	CAT NOT NULL,
	"actual_name"	VARCHAR,
	"info_para_reporte"	VARCHAR,
	"cita"	VARCHAR,
	"cancer_risk"	REAL,
	"development_risk"	REAL,
	"allergies_risk"	REAL,
	"endocryne_risk"	REAL,
	"prohibited_risk"	REAL,
	"env_risk"	REAL,
	"total_risk"	REAL,
	PRIMARY KEY("id" )
);
