#[derive(Debug)]
pub struct Ledger {
    pub directives: Vec<DatedDirective>,
}

#[derive(Debug)]
pub struct DatedDirective {
    pub date: String,
    pub directive: Directive,
}
#[derive(Debug)]
pub enum Directive {
    Open(OpenDirective), //Close(CloseDirective)
                         //Note(NoteDirective)
                         //Balance(BalanceDirective)
}

#[derive(Debug)]
pub struct OpenDirective {
    pub account: Account,
    pub currencies: Vec<Currency>,
    pub booking_method: Option<BookingMethod>,
}
#[derive(Debug)]
pub struct Account {
    pub name: Vec<String>,
}
#[derive(Debug)]
pub struct Currency {
    pub name: String,
}
#[derive(Debug)]
pub struct BookingMethod {
    pub name: String,
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_open_account() {
        let ast = parser::LedgerParser::new()
            .parse("2023-01-12 open Liabilities:CreditCard:CapitalOne USD");

        expect![[r#"
            Ok(
                Ledger {
                    directives: [
                        DatedDirective {
                            date: "2023-01-12",
                            directive: Open(
                                OpenDirective {
                                    account: Account {
                                        name: [
                                            "Liabilities",
                                            "CreditCard",
                                            "CapitalOne",
                                        ],
                                    },
                                    currencies: [
                                        Currency {
                                            name: "USD",
                                        },
                                    ],
                                    booking_method: None,
                                },
                            ),
                        },
                    ],
                },
            )
        "#]]
        .assert_debug_eq(&ast);
    }
}
