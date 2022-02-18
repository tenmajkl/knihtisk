use std::{env, fs, path::Path, process::exit};


enum TokenKind {
    PRINT,
    STRING
}

struct Token {
    kind: TokenKind,
    context: String
}

struct Statement {
    tokens: Vec<Token>
}

fn error(message: String) -> ! {
    println!("\x1b[31mChyba! {}\x1b[0m", message);
    exit(-1)
}

fn tokenize(word: &String) -> Token {
    let kind: TokenKind = match word.as_str() {
        "Vyrkni" => TokenKind::PRINT,
        _ => error(format!("Zapsané slovo {} neníž vokabulářem známo!", word))
    };

    Token { 
        kind, 
        context: String::new() 
    }
}

fn lex(program: String) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut last_statement: Statement = Statement { tokens: Vec::new() }; 
    let mut last: String = String::new();
    let mut parsing_string = false;
    let mut statement_end = false;
    for index in 0..program.len() {
        match program.as_bytes()[index] as char {
            '.' => { 
                if program.as_bytes()[index+1] as char == ' ' {
                    statements.push(last_statement);
                    last_statement = Statement { tokens: Vec::new() };
                    statement_end = true;
                } else {
                    error(String::from("Věty musí být oddělenyž škvírou!"));
                }
            },
            
            '"' => {
                if parsing_string {
                    last_statement.tokens.push(Token { kind: TokenKind::STRING, context: last.clone() });
                    last.clear();
                    parsing_string = false;
                } else {
                    parsing_string = true;
                }
            }

            ' ' => { 
                if statement_end {
                    statement_end = false;
                    continue;
                }
                if parsing_string {
                    last.push(' ');
                } else {
                    last_statement.tokens.push(tokenize(&last));
                    last.clear();
                }
            }
            character => {
                last.push(character);
            }
        }
    }
    return statements
}

fn parse_statement(statement: Statement) -> String {
    match statement.tokens[0].kind {
        TokenKind::PRINT => format!("   printf(\"{}\");\n", statement.tokens[1].context),
        TokenKind::STRING => String::new()
    }
}

fn parse(program: Vec<Statement>) -> String {
    let mut result: String = String::from("#include<stdio.h>\n\nint main(void)\n{\n");
    for statement in program {
        result.push_str(&parse_statement(statement));
    }
    result.push_str("}");
    return result
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists() {
        error(format!("Kniha {} nebylaž nalezena v archivu!", filename));
    }

    let code = fs::read_to_string(filename)
        .expect("Kniha jsa psána písmem klínovým a nečitelná jest!");

    let tokens = lex(code);
    println!("Kniha:\n{}", parse(tokens)); // TODO file writing
}
