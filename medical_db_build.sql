DROP DATABASE IF EXISTS medical_db;

CREATE DATABASE IF NOT EXISTS medical_db;

USE medical_db;

CREATE TABLE medical_report_provider (
    provider_id         INT             AUTO_INCREMENT,
    provider_name       VARCHAR(400),
    PRIMARY KEY (provider_id)
);

CREATE TABLE medical_report_type (
    report_type_id      INT          AUTO_INCREMENT,
    report_type_name    VARCHAR(400) NOT NULL,
    PRIMARY KEY (report_type_id)
);

CREATE TABLE medical_report (
    report_id           INT             AUTO_INCREMENT,
    report_date         DATETIME        NOT NULL,
    report_provider     INT             NOT NULL,
    report_type         INT             NOT NULL,
    PRIMARY KEY (report_id),
    CONSTRAINT FOREIGN KEY (report_provider) REFERENCES medical_report_provider (provider_id),
    CONSTRAINT FOREIGN KEY (report_type) REFERENCES medical_report_type (report_type_id)
);

CREATE TABLE provider_physical_address (
    provider_id             INT                 NOT NULL,
    provider_street_address VARCHAR(400)        NOT NULL,
    PRIMARY KEY (provider_id, provider_street_address),
    CONSTRAINT FOREIGN KEY (provider_id) REFERENCES medical_report_provider (provider_id)
);

CREATE TABLE provider_contact_position_classification (
    classification_id               INT             AUTO_INCREMENT,
    classification_name             VARCHAR(100)    NOT NULL,
    classification_description      VARCHAR(400),
    PRIMARY KEY (classification_id)
);

CREATE TABLE provider_contact_name (
    provider_id                      INT             NOT NULL,
    provider_contact_id              INT             AUTO_INCREMENT,
    provider_contact_classification  INT             NOT NULL,
    provider_contact_last_name       VARCHAR(50)     NOT NULL,
    provider_contact_first_name      VARCHAR(50),
    PRIMARY KEY (provider_contact_id),
    CONSTRAINT FOREIGN KEY (provider_id) REFERENCES medical_report_provider (provider_id),
    CONSTRAINT FOREIGN KEY (provider_contact_classification) REFERENCES provider_contact_position_classification (classification_id)
);

CREATE TABLE provider_contact_email (
    provider_id                         INT             NOT NULL,
    provider_contact_email_address      VARCHAR(200)    NOT NULL,
    PRIMARY KEY (provider_id, provider_contact_email_address),
    CONSTRAINT FOREIGN KEY (provider_id) REFERENCES medical_report_provider (provider_id)
);

CREATE TABLE provider_contact_phone (
    provider_id                     INT             NOT NULL,
    provider_contact_phone_number   VARCHAR(200)    NOT NULL,
    PRIMARY KEY (provider_id, provider_contact_phone_number),
    CONSTRAINT FOREIGN KEY (provider_id) REFERENCES medical_report_provider (provider_id)
);

CREATE TABLE medical_report_line_item_unit (
    value_unit_id               INT             AUTO_INCREMENT,
    value_unit_full_name        VARCHAR(100)    NOT NULL,
    value_unit_description      VARCHAR(400),
    value_unit_shorthand        VARCHAR(30)     NOT NULL,
    PRIMARY KEY (value_unit_id)
);

CREATE TABLE medical_report_line_item_classification (
    value_type_id               INT             AUTO_INCREMENT,
    value_type_name             VARCHAR(100)    NOT NULL,
    value_type_abbreviation     VARCHAR(30)     NOT NULL,
    PRIMARY KEY (value_type_id)
);

CREATE TABLE medical_report_line_item (
    report_id           INT         NOT NULL,
    value_type          INT         NOT NULL,
    value_unit          INT         NOT NULL,
    value_measurement   DOUBLE      NOT NULL,
    PRIMARY KEY (report_id, value_type),
    CONSTRAINT FOREIGN KEY (report_id)  REFERENCES medical_report (report_id),
    CONSTRAINT FOREIGN KEY (value_type) REFERENCES medical_report_line_item_classification (value_type_id),
    CONSTRAINT FOREIGN KEY (value_unit) REFERENCES medical_report_line_item_unit (value_unit_id)
);

-- TEST DATA

INSERT INTO medical_report_provider (provider_name) VALUES
    ("Test Provider 1"),
    ("Test Provider 2");

INSERT INTO medical_report_type (report_type_name) VALUES
    ("Renal Function Panel"),
    ("Cystatin C"),
    ("Uric Acid"),
    ("Complete Blood Cell Count with Automated Diff"),
    ("Complete Metabolic Panel");

INSERT INTO medical_report_line_item_unit (value_unit_full_name, value_unit_description, value_unit_shorthand) VALUES
    ("Milligrams per Deciliter", "", "mg/dL"),
    ("Grams per Deciliter", "", "g/dL"),
    ("Milimoles per Liter", "", "mmol/L"),
    ("Mililiters per Minim per Body Surface Area", "", "mL/min/1.73m sq"), -- https://en.wikipedia.org/wiki/Glomerular_filtration_rate#Measurement
    ("Thousand Cells per Cubic Milimeter", "", "K/cubic mm"),
    ("Million Cells per Cubic Milimeter", "", "M/cubic mm"), -- https://www.cdc.gov/cliac/docs/addenda/cliac0313/13A_CLIAC_2013March_UnitsOfMeasure.pdf
    ("Percentage", "", "%"),
    ("Picograms", "", "pg");

INSERT INTO medical_report_line_item_classification (value_type_name, value_type_abbreviation) VALUES
    ("Albumin", "ALB"),
    ("Carbon Dioxide", "CO2"),
    ("Blood Urea Nitrogen", "BUN"),
    ("Calcium", "CA"),
    ("Creatinine", "CREA"),
    ("Glucose", "GLUC"),
    ("Phosphorus", "PHOS"),
    ("Sodium", "NA"),
    ("Potassium", "K"),
    ("Chloride", "CL"),
    ("Estimated Glomerular Filtration Rate", "eGFR"),
    ("Blood Urea Nitrogen-Creatinine Ratio", "B/C RATIO"),
    ("Cystatin C", "CYSC"),
    ("Glomerular Filtration Rate Epidemiology Collaboration", "GFREPI"),
    ("Uric Acid", "URCA"),
    ("White Blood Cell Count", "WBC"),
    ("Red Blood Cell Count", "RBC"),
    ("Hemoglobin", "HGB"),
    ("Hematocrit", "HCT"),
    ("Mean Corpuscular Volume", "MCV"),
    ("Mean Corpuscular Hemoglobin", "MCH"),
    ("Mean Corpuscular Hemoglobin Concentration", "MCHC"),
    ("Platelets", "PLT"),
    ("Red Cell Distribution Width", "RDW"),
    ("Mean Platelet Volume", "MPV"),
    ("Neutrophils", "NEU #"),
    ("Neutrophils", "NEU %"),
    ("Lymphocytes", "LY #"),
    ("Lymphocytes", "LY %"),
    ("Monocytes", "MO #"),
    ("Monocytes", "MO %"),
    ("Eosinophil", "EOS #"),
    ("Eosinophils", "EOS %"),
    ("Basophils", "BASO #"),
    ("Basophils", "BASO %"),
    ("Immunoglobulins", "IG #"),
    ("Immunoglobulins", "IG %");

INSERT INTO provider_contact_position_classification (classification_name) VALUES 
    ("Physician"),
    ("Nephrologist"),
    ("Nurse Practicioner");


INSERT INTO medical_report (report_date, report_provider, report_type) VALUES
    ("2021-06-16", 2, 5);

INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement) VALUES
    (1, 1, 1, 96);

-- INSERT INTO medical_report (report_date, report_provider) VALUES
--     ("2023-9-28", 1);

-- INSERT INTO medical_report_line_item (report_id, value_type, value_unit, value_measurement) VALUES
--     (1, 1, 1, 3.37);



-- TEST QUERY FOR PARITY WITH LAB REPORTS

-- SELECT 
-- 	mrd.report_id,
--   mrvt.value_type_abbreviation,
--   mrd.value_measurement,
--   mrvu.value_unit_shorthand  
-- FROM 
-- 	medical_report_line_item mrd
--   	JOIN medical_report_line_item_unit mrvu
--   		ON mrvu.value_unit_id = mrd.value_type
--    	JOIN medical_report_line_item_classification mrvt
--     	    ON mrvt.value_type_id = mrd.report_id

