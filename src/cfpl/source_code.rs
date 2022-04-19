use super::token;

pub struct SourceCode {
    pub source_code: String,
    pub vec: Vec<char>,
}

impl SourceCode {
    fn get_code_at_line(&self, line_number: usize) -> String {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut line: usize = 0;
        for (index, character) in self.source_code.chars().enumerate() {
            if character == '\n' || index == self.source_code.len() - 1 {
                if line > 0 {
                    start = end + 1;
                }
                if index == self.source_code.len() - 1 {
                    end = index + 1;
                } else {
                    end = index;
                }
                if line == line_number {
                    break;
                }
                line = line + 1;
            }
        }
        String::from(&self.source_code[start..end])
    }

    pub fn error_token(&self, token: token::Token, message: String) -> Result<(), String> {
        let line_code = self.get_code_at_line(token.line as usize);
        let mut error_point = " ".repeat(if token.column > 0 {
            token.column as usize - 1
        } else {
            token.column as usize
        });
        error_point.push_str("^");
        let error_line = token.line + 1;
        let error_column = token.column + 1;
        Err(format!("{message}\n[line: {error_line} column: {error_column}] on {token}\n{line_code}\n{error_point}"))
    }

    pub fn error_manual(
        &self,
        line: usize,
        column: usize,
        at_fault: String,
        message: String,
    ) -> Result<(), String> {
        let line_code = self.get_code_at_line(line as usize);
        let mut error_point = " ".repeat(if column > 0 { column - 1 } else { column });
        error_point.push_str("^");
        let error_line = line + 1;
        let error_column = column + 1;
        Err(format!("{message}\nline-{error_line}:column-{error_column}: {at_fault}\n{line_code}\n{error_point}"))
    }
}
