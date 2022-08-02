use super::token;

pub struct SourceCode {
    pub source_code: String,
    pub vec: Vec<char>,
}

impl SourceCode {
    pub fn get_code_at_line(&self, line_number: usize) -> String {
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
                line += 1;
            }
        }
        String::from(&self.source_code[start..end])
    }

    pub fn error_string_token(&self, token: &token::Token, message: &str) -> String {
        let line_code = self.get_code_at_line(token.line as usize);
        let mut error_point = " ".repeat(if token.column > 0 {
            token.column as usize - 1
        } else {
            token.column as usize
        });
        error_point.push('^');
        let error_line = token.line + 1;
        let error_column = token.column + 1;
        format!("{message}\nline-{error_line}:column-{error_column}: {token}\n{line_code}\n{error_point}")
    }

    pub fn error_string_manual(
        &self,
        line: usize,
        column: usize,
        at_fault: String,
        message: String,
    ) -> String {
        let line_code = self.get_code_at_line(line as usize);
        let mut error_point = " ".repeat(column);
        error_point.push('^');
        let error_line = line + 1;
        let error_column = column + 1;
        format!(
            "{message}\nline-{error_line}:column-{error_column}: {}\n{line_code}\n{error_point}",
            at_fault.escape_debug()
        )
    }
}
