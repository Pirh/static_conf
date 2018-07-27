use error::OptionsError;
use validation;

#[derive(Debug)]
pub struct Options {
    pub struct_name: String,
    pub const_name: Option<String>,
    pub generate_const: bool,
    pub derived_traits: Vec<String>,
}

impl Options {
    pub fn validate(&self) -> Result<(), OptionsError> {
        if !validation::valid_identifier(&self.struct_name) {
            return Err(OptionsError::InvalidStructName(self.struct_name.clone()));
        }

        Ok(())
    }

    pub fn real_const_name(&self) -> String {
        self.const_name
            .clone()
            .unwrap_or_else(|| self.struct_name.to_uppercase())
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            struct_name: "Config".to_owned(),
            const_name: None,
            generate_const: true,
            derived_traits: vec![
                "Debug".to_owned(),
                "Clone".to_owned(),
                "Serialize".to_owned(),
                "Deserialize".to_owned(),
            ],
        }
    }
}
