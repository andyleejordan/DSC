use std::collections::HashMap;

use crate::sshdconfig_error::SshdConfigError;
use crate::config::subcontainer::{KeywordType, SubContainer, UpdateKind};
use crate::config::utils::{export_json, export_sshd_config, validate_config};

/// ConfigData is the top-level object that contains all information for sshd_config
pub struct ConfigData {
    pub config_lookup: HashMap<String, SubContainer>,
    config_filepath: String,
}

impl ConfigData {
    pub fn new() -> Self {
        let mut config_lookup = HashMap::new();
        // TODO: import const_keywords mod & use VALID_KEYWORDS to initialize config_lookup
        // initialize config_filepath to default location based on the OS
        // need an empty temp file to run sshd -T with to get defaults
        let temp_filepath = "not implemented yet".to_string();
        let (is_valid, defaults) = validate_config(&temp_filepath);
        // parsing defaults here might be similar to import_sshd_config?
        Self {
            config_lookup,
            config_filepath: "not implemented yet".to_string(),
        }
    }

    /// take input input and update config_lookup
    pub fn import_sshd_config(&self, data: &String) {
        // here we first update config_lookup from text
        // any user input makes that key no longer default
        // then export config to temp file
        // and run sshd -T
        // we also could pass in the filepath and run sshd -T directly
        // but then would need to go back and mark any defaults
        // specifically called out in text file
    }

    /// take input data and update config_lookup
    pub fn import_json(&self, data: &String) {
        // TODO: think of better way to validate json
        // here we first update config_lookup from json
        // any user input makes that key no longer default
        // then export config to temp file
        // and run sshd -T
    }

    /// apply_config will be called from set
    /// it will restart sshd (platform specific)
    /// return: bool indicating success/failure
    /// potentially a status code for the return instead?
    fn apply_config(&self) -> bool {
        false
    }

    /// backup_file will be called from set
    /// needs to backup original sshd_config
    /// if a backup does not already exist
    /// return: bool indicating success/failure
    fn backup_file(&self) -> bool {
        false
    }

    /// file_check will be called from import_sshd_config
    /// to check if the input file was generated by the tool
    /// if it was, compare hash and file contents
    /// to verify no external modifications were made
    /// return: bool indicating if valid file content
    fn file_check(&self) -> bool {
        false
    }

    /// compare will be called from set & test
    /// return: hashmap with subcontainer values from self and hashmap with updateKind
    /// for any <keyword, values> that differ between self & config,
    /// the <keyword, updateKind> is needed for set, can be ignored for test
    fn compare(&self, config: &ConfigData) -> (Option<HashMap<String, SubContainer>>, Option<HashMap<String, UpdateKind>>) {
        (None, None)
    }

    /// update will be called from set
    /// it will call add/remove/modify accordingly
    /// return: bool indicating success/failure
    fn update(&self, config: &HashMap<String, SubContainer>, update_kind: &HashMap<String, UpdateKind>) -> bool {
        false
    }

    /// modify is intended to be called from set
    /// when a keyword that is already defined in ConfigData needs to be changed
    /// # Example
    /// cd = ConfigData::new();
    /// cd.modify("Port".to_string(), KeywordType::KeywordValue("1234".to_string()))
    fn modify(&mut self, keyword: &String, args: KeywordType) {

    }

    /// add is intended to be called from set
    /// when a keyword & its args are not already defined in ConfigData and need to be added
    /// # Example
    /// cd = ConfigData::new();
    /// cd.add("Port".to_string(), KeywordType::KeywordValue("1234".to_string()))
    fn add(&mut self, keyword: &String, args: KeywordType) {

    }

    /// remove is intended to be called from set
    /// when a keyword & its args are already defined in ConfigData but need to be removed
    /// # Example
    /// cd = ConfigData::new();
    /// cd.remove("Port".to_string(), KeywordType::KeywordValue("1234".to_string()))
    fn remove(&mut self, keyword: &String, args: KeywordType) {

    }
}

impl Default for ConfigData {
    fn default() -> Self {
        ConfigData::new()
    }
}

pub trait Invoke {
    fn get(&self, keywords: &Option<Vec<String>>) -> Result<(), SshdConfigError>; 
    fn set(&self, other: &ConfigData) -> Result<(), SshdConfigError>;
    fn test(&self, other: &ConfigData) -> Result<(), SshdConfigError>;
}

impl Invoke for ConfigData {
    /// # Example
    /// cd = ConfigData::new();
    /// cd.import_sshd_config("PasswordAuthentication yes /r/n Port 1234")
    /// cd.get()
    /// returns {"PasswordAuthentication": "yes", "Port": 1234}
    /// cd.get(vec!["Port".to_string()])
    /// returns {"Port": 1234}
    fn get(&self, keywords: &Option<Vec<String>>) -> Result<(), SshdConfigError> {
        self.file_check();
        export_json(&self.config_lookup, keywords);
        Ok(())
    }
    /// # Example
    /// cd = ConfigData::new();
    /// cd.import_sshd_config("PasswordAuthentication yes") // existing config
    /// cd2 = ConfigData::new();
    /// cd2.import_sshd_config("PasswordAuthentication no") // input config
    /// cd.set(&cd2);
    /// expected outcomes: backup sshd_config if necessary, 
    /// update keyword(s) in sshd_config & restart sshd
    fn set(&self, other: &ConfigData) -> Result<(), SshdConfigError> {
        self.file_check();
        let (diff, update_kind) = other.compare(self);
        match diff {
            Some(diff) => {
                match update_kind {
                    Some(update_kind) => {
                        self.update(&diff, &update_kind);
                        self.backup_file();
                        // TODO: confirm if a temporary file is required to pass into SSHD -T
                        let temp_filepath = "temp file path".to_string();
                        export_sshd_config(&self.config_lookup, &temp_filepath);
                        let (is_valid, _) = validate_config(&temp_filepath);
                        // remove temp file after use 
                        if is_valid {
                            export_sshd_config(&self.config_lookup, &self.config_filepath);
                            self.apply_config();
                        }
                    }
                    None => {
                        println!("failed to parse update kind");
                    }
                }
            } 
            None => {
                println!("{{}}");
            }
        }
        Ok(())
    }
    /// # Example
    /// cd = ConfigData::new();
    /// cd.import_sshd_config("PasswordAuthentication yes") // existing config
    /// cd2 = ConfigData::new();
    /// cd2.import_sshd_config("PasswordAuthentication no") // input config
    /// cd.test(&cd2);
    /// expected return: {"PasswordAuthentication": "yes"}
    fn test(&self, other: &ConfigData) -> Result<(), SshdConfigError> {
        self.file_check();
        let (diff, _) = self.compare(other);
        match diff {
            Some(diff) => {
                export_json(&diff, &None);
            } 
            None => {
                println!("{{}}");
            }
        }
        Ok(())
    }
}
