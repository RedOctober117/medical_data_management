pub mod db_lib {
    use sqlx::types::chrono::NaiveDateTime;

    pub struct MedicalReportProvider {
        pub provider_id: i32,
        pub provider_name: Option<String>,
    }
    
    pub struct MedicalReportType {
        pub report_type_id: i32,
        pub report_type_name: String,
    }
    
    pub struct MedicalReport {
        pub report_id: i32,
        pub report_date: NaiveDateTime,
        pub report_provider: i32,
        pub report_type: i32,
    }
    
    pub struct ProviderPhysicalAddress {
        pub provider_id: i32,
        pub provider_street_address: String,
    }
    
    pub struct ProviderContactPositionClassification {
        pub classification_id: i32,
        pub classification_name: String,
        pub classification_description: Option<String>,
    }
    
    pub struct ProviderContactName {
        pub provider_id: i32,
        pub provider_contact_id: i32,
        pub provider_contact_classification: i32,
        pub provider_contact_last_name: String,
        pub provider_contact_first_name: Option<String>,
    }
    
    pub struct ProviderContactEmail {
        pub provider_id: i32,
        pub provider_contact_email_address: String,
    }
    
    pub struct ProviderContactPhone {
        pub provider_id: i32,
        pub provider_contact_phone_number: String,
    }
    
    pub struct MedicalReportLineItemUnit {
        pub value_unit_id: i32,
        pub value_unit_full_name: String,
        pub value_unit_description: Option<String>,
        pub value_unit_shorthand: String,
    }
    
    pub struct MedicalReportLineItemClassification {
        pub value_type_id: i32,
        pub value_type_name: String,
        pub value_type_abbreviation: String,
    }
    
    pub struct MedicalReportLineItem {
        pub report_id: i32,
        pub value_type: i32,
        pub value_unit: i32,
        pub value_measurement: f32,
    }
    
    
    impl std::fmt::Display for MedicalReportProvider {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, provider_name: {})", 
                        self.provider_id, self.provider_name.clone().unwrap())
        }
    }
    
    impl std::fmt::Display for MedicalReportType {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(report_type_id: {}, report_type_name: {})", 
                        self.report_type_id, self.report_type_name.clone())
        }
    }
    
    impl std::fmt::Display for MedicalReport {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, report_date: {}, report_provider: {}, report_type: {})", 
                        self.report_id, self.report_date.clone(), self.report_provider, self.report_type)
        }
    }
    
    impl std::fmt::Display for ProviderPhysicalAddress {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, provider_street_address: {})", 
                        self.provider_id, self.provider_street_address.clone())
        }
    }
    
    impl std::fmt::Display for ProviderContactPositionClassification {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(classification_id: {}, classification_name: {}, classification_description: {})", 
                        self.classification_id, self.classification_name.clone(), self.classification_description.clone().unwrap())
        }
    }
    
    impl std::fmt::Display for ProviderContactName {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, provider_contact_id: {}, provider_contact_classification: {}, provider_contact_last_name: {}, provider_contact_first_name: {})", 
                        self.provider_id, self.provider_contact_id, self.provider_contact_classification, self.provider_contact_last_name.clone(), self.provider_contact_first_name.clone().unwrap())
        }
    }
    
    impl std::fmt::Display for ProviderContactEmail {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, provider_contact_email_address: {})", 
                        self.provider_id, self.provider_contact_email_address.clone())
        }
    }
    
    impl std::fmt::Display for ProviderContactPhone {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(provider_id: {}, provider_contact_phone_number: {})", 
                        self.provider_id, self.provider_contact_phone_number.clone())
        }
    }
    
    impl std::fmt::Display for MedicalReportLineItemUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(value_unit_id: {}, value_unit_full_name: {}, value_unit_description: {}, value_unit_shorthand: {})", 
                        self.value_unit_id, self.value_unit_full_name.clone(), self.value_unit_description.clone().unwrap(), self.value_unit_shorthand.clone())
        }
    }
    
    impl std::fmt::Display for MedicalReportLineItemClassification {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(value_type_id: {}, value_type_name: {}, value_type_abbreviation: {})", 
                        self.value_type_id, self.value_type_name.clone(), self.value_type_abbreviation.clone())
        }
    }
    
    impl std::fmt::Display for MedicalReportLineItem {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(report_id: {}, value_type: {}, value_unit: {}, value_measurement: {})", 
                        self.report_id, self.value_type, self.value_unit, self.value_measurement)
        }
    }
}