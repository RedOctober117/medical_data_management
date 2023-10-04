## Introduction
This is my medical_db prototype. The purpose of this program is to allow for more succinct storage of blood work and the analysis thereof. There are three current components:
- query_builder
- egfr_calculator
- medical_db_build.sql

`query_builder` is designed to make it faster to write test queries, as many tables are almost entirely foreign key integer ids. `egfr_calculator` is intented to test the various equations posed for EGFR, but is not being used or updated at this time. `medical_db_build.sql` is the main database, designed on the principle that every entry belongs to the same table, with a FK id to a report table. I encourage you to read the build script yourself and also take a look at the generated ERD.