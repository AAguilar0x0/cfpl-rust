use super::source_code;
use super::token;
use super::token_type;

pub fn lexical_analysis(cfpl_source_code: &source_code::SourceCode) -> Result<(), String> {
    cfpl_source_code.error_manual(
        1,
        1,
        String::from("START"),
        String::from("Expected variable name."),
    )?;
    Ok(())
}
