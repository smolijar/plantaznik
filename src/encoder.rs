use thiserror::Error;

use plantuml_encoding::{encode_plantuml_deflate, FromPlantumlError};

#[derive(Error, Debug)]
#[error("Error encoding PlantUML source: {0}")]
pub struct PlantumlEncodingError(String);

fn encode(source: &str) -> Result<String, FromPlantumlError> {
    encode_plantuml_deflate(source)
}

pub trait SourceEncode {
    fn src_to_img(&self, source: &str) -> Result<String, PlantumlEncodingError>;
}

#[derive(Default)]
pub struct SourceEncoder {}
impl SourceEncode for SourceEncoder {
    fn src_to_img(&self, source: &str) -> Result<String, PlantumlEncodingError> {
        // TODO: Alt from title/filename?
        let encoded = encode(source).map_err(|e| PlantumlEncodingError(e.0))?;
        Ok(format!(
            "![](https://www.plantuml.com/plantuml/svg/{encoded})"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("@startuml\nBob -> Alice : hello\n@enduml").unwrap(),
            "0IO0sVz0StHXSdHrRMmAGczY82q-845iQMDb83eWQ6LiR6yAG6LkP7LjR000"
        );
    }

    #[test]
    fn test_src_to_img() {
        assert_eq!(
            SourceEncoder::default()
                .src_to_img("@startuml\n@enduml")
                .unwrap(),
            "![](https://www.plantuml.com/plantuml/svg/SoWkIImgAStDuN98pKi1qW00)"
        );
    }
}
