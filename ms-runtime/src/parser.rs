use crate::sexpr::SExpr;

pub(crate) struct Parser {
    source: String,
    position: usize,
}

impl Parser {
    pub(crate) fn new(source: &str) -> Parser {
        Parser {
            source: source.to_string(),
            position: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<SExpr>, String> {
        let mut sexprs = vec![];

        loop {
            let Some(sexpr) = self.parse_sexp()? else {
                break;
            };

            sexprs.push(sexpr);
        }

        Ok(sexprs)
    }

    fn parse_sexp(&mut self) -> Result<Option<SExpr>, String> {
        let Some(token) = self.next_token() else {
            return Ok(None);
        };

        if token == "(" {
            let mut args = vec![];

            loop {
                let Some(token) = self.next_token() else {
                    return Err("Unexpected end of input".to_string());
                };

                if token == ")" {
                    break;
                }

                if token == "(" {
                    self.position -= 1;

                    let Some(inner_sexpr) = self.parse_sexp()? else {
                        return Err("Expected S-expression".to_string());
                    };

                    args.push(inner_sexpr);
                } else {
                    args.push(SExpr::Atom(token));
                }
            }

            Ok(Some(SExpr::List(args)))
        } else {
            return Err(format!("Unexpected token: {}", token));
        }
    }

    fn next_token(&mut self) -> Option<String> {
        let mut token = String::new();

        loop {
            let Some(char) = self.source.chars().nth(self.position) else {
                break;
            };

            match char {
                '(' | ')' => {
                    if !token.is_empty() {
                        break;
                    }

                    token.push(char);

                    self.position += 1;
                    break;
                }
                '"' => {
                    if !token.is_empty() {
                        break;
                    }

                    self.position += 1;

                    loop {
                        let Some(char) = self.source.chars().nth(self.position) else {
                            return None;
                        };

                        if char == '"' {
                            self.position += 1;
                            break;
                        }

                        token.push(char);
                        self.position += 1;
                    }

                    break;
                }
                ';' => {
                    if !token.is_empty() {
                        break;
                    }

                    self.position += 1;

                    loop {
                        let Some(char) = self.source.chars().nth(self.position) else {
                            return None;
                        };

                        if char == '\n' {
                            break;
                        }

                        self.position += 1;
                    }
                }
                ' ' | '\n' | '\r' | '\t' => {
                    if !token.is_empty() {
                        break;
                    }

                    self.position += 1;
                }
                _ => {
                    token.push(char);
                    self.position += 1;
                }
            }
        }

        if token.is_empty() {
            None
        } else {
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_next_token() {
        let mut parser = Parser::new(" test(fn add);Hello\nfield.get\"Test\"");

        assert_eq!(parser.next_token(), Some("test".to_string()));
        assert_eq!(parser.next_token(), Some("(".to_string()));
        assert_eq!(parser.next_token(), Some("fn".to_string()));
        assert_eq!(parser.next_token(), Some("add".to_string()));
        assert_eq!(parser.next_token(), Some(")".to_string()));
        assert_eq!(parser.next_token(), Some("field.get".to_string()));
        assert_eq!(parser.next_token(), Some("Test".to_string()));
    }
}
