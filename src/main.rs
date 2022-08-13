use cfpl::{self, statement::Statement};

fn main() {
    cfpl::file("./test_source_codes/0.cfpl");

    // let x = cfpl::statement::print::Print {
    //     expression: Box::new(cfpl::expression::variable::Variable {
    //         name: cfpl::token::Token::new(
    //             cfpl::token_type::TokenType::Identifier,
    //             String::from("asdf"),
    //             1,
    //             3,
    //         ),
    //     }),
    // };
}
