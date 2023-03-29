use thiserror::Error;

use plantuml_encoding::encode_plantuml_deflate;

#[derive(Error, Debug)]
#[error("Error encoding PlantUML source: {0}")]
pub struct PlantumlEncodingError(String);

pub trait SourceEncode {
    fn encode_source(&self, source: &str) -> Result<String, PlantumlEncodingError>;
}

#[derive(Default)]
pub struct SourceEncoder {}
impl SourceEncode for SourceEncoder {
    fn encode_source(&self, source: &str) -> Result<String, PlantumlEncodingError> {
        encode_plantuml_deflate(source).map_err(|e| PlantumlEncodingError(e.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            SourceEncoder::default()
                .encode_source("@startuml\nBob -> Alice : hello\n@enduml")
                .unwrap(),
            "0IO0sVz0StHXSdHrRMmAGczY82q-845iQMDb83eWQ6LiR6yAG6LkP7LjR000"
        );
    }
}
