/*
 *
 *
 *
 *
 *
 *
 *
 */

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

use crate::error;
use crate::scanner;
use crate::scanner::Token;
// use crate::symbol_table;
use crate::symbol_table::SymbolTable;

lazy_static! {
    static ref symbols: Mutex<SymbolTable<'static>> = Mutex::new(SymbolTable::init_global());
}

pub fn parse() {
    let mut token = scanner::get_token();

    if *super::chk_decl.lock().unwrap() {
        symbols
            .lock()
            .unwrap()
            .add_function(&mut String::from("println"), &mut 1);
    }

    prog(&mut token);

    println!("finished!");
}

pub fn print_tokens() {
    scanner::reset_file();
    loop {
        let tok = scanner::get_token();
        if !print_token(&tok) {
            break;
        }
    }
    scanner::reset_file();
}

fn print_token(token: &Token) -> bool {
    match token {
        Token::UNDEF => {
            println!("UNDEF TOKEN");
            return false;
        }
        Token::EOF => {
            println!("END OF FILE");
            return false;
        }
        _ => println!("{:?}", token),
    }
    return true;
}

fn match_token(token: &mut Token, to_match: Token) -> Token {
    // print_token(&to_match);
    // print_token(token);
    // println!();
    match token {
        Token::ID(_) => {
            match to_match {
                Token::ID(_) => {
                    // valid
                    let out = token.clone();
                    *token = scanner::get_token();
                    return out;
                }
                _ => {
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
                    return Token::UNDEF;
                }
            }
        }
        Token::INTCONST(_) => {
            match to_match {
                Token::INTCONST(_) => {
                    // valid
                    let out = token.clone();
                    *token = scanner::get_token();
                    return out;
                }
                _ => {
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
                    return Token::UNDEF;
                }
            }
        }
        _ => {
            if *token == to_match {
                // valid
                let out = token.clone();
                *token = scanner::get_token();
                return out;
            }
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token");
            return Token::UNDEF;
        }
    }
}

fn prog(token: &mut Token) {
    if *token == Token::EOF {
        return;
    }

    match token {
        Token::KW(kw) => {
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "prog");
            }

            mtype(token);

            let mut id: String;
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => id = s,
                _ => id = String::new(),
            }

            func_var(token, &mut id);
            prog(token);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "prog"),
    }
}

fn func_var(token: &mut Token, id: &mut String) {
    match *token {
        Token::SEMI | Token::COMMA => {
            if *super::chk_decl.lock().unwrap() && symbols.lock().unwrap().global_var_def(id) {
                eprintln!("cannot redefine global var: {}", id);
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var");
            }
            {
                symbols.lock().unwrap().add_global(id);
            }
            var_decl(token, true);
            return;
        }
        Token::LPAREN => {
            func_defn(token, id);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var"),
    }
}

fn var_decl(token: &mut Token, global: bool) {
    match *token {
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return;
        }
        Token::COMMA => {
            match_token(token, Token::COMMA);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    if global {
                        // global
                        if *super::chk_decl.lock().unwrap()
                            && symbols.lock().unwrap().global_var_def(&mut s)
                        {
                            eprintln!("cannot redefine global var: {}", s);
                            error::print_err_rule(
                                *scanner::line.lock().unwrap(),
                                token,
                                "var_decl",
                            );
                        }
                        {
                            symbols.lock().unwrap().add_global(&mut s);
                        }
                    } else {
                        // body
                        if *super::chk_decl.lock().unwrap()
                            && symbols.lock().unwrap().body_var_param_def(&mut s)
                        {
                            eprintln!("cannot redefine body var: {}", s);
                            error::print_err_rule(
                                *scanner::line.lock().unwrap(),
                                token,
                                "var_decl",
                            );
                        }
                        {
                            symbols.lock().unwrap().add_body_var(&mut s);
                        }
                    }
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "var_decl"),
            }
            var_decl(token, global);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "var_decl"),
    }
}

fn mtype(token: &mut Token) {
    match_token(token, Token::KW(String::from("int")));
}

fn func_defn(token: &mut Token, id: &mut String) {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let mut params = Vec::new();
            opt_formals(token, &mut params);
            if *super::chk_decl.lock().unwrap()
                && symbols
                    .lock()
                    .unwrap()
                    .function_def(id, &mut (params.len() as u32))
            {
                eprintln!("cannot redefine function: {}", id);
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_defn");
            }
            {
                symbols
                    .lock()
                    .unwrap()
                    .add_function(id, &mut (params.len() as u32));
            }
            for mut param in params {
                symbols.lock().unwrap().add_param(&mut param);
            }
            match_token(token, Token::RPAREN);
            match_token(token, Token::LBRACE);
            opt_var_decls(token);
            opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_defn"),
    }
}

fn opt_formals(token: &mut Token, params: &mut Vec<String>) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals");
            }
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => params.push(s),
                _ => params.push(String::new()),
            }
            formals(token, params);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals"),
    }
}

fn formals(token: &mut Token, params: &mut Vec<String>) {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(s) => params.push(s),
                _ => params.push(String::new()),
            }
            formals(token, params);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "formals"),
    }
}

fn opt_var_decls(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw == String::from("if")
                || *kw == String::from("return")
                || *kw == String::from("while")
            {
                return;
            }
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls");
            }
            mtype(token);
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    if *super::chk_decl.lock().unwrap()
                        && symbols.lock().unwrap().global_var_def(&mut s)
                    {
                        eprintln!("cannot redefine global var: {}", &mut s);
                        error::print_err_rule(
                            *scanner::line.lock().unwrap(),
                            token,
                            "opt_var_decls",
                        );
                    }
                    {
                        symbols.lock().unwrap().add_global(&mut s);
                    }
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls"),
            }
            var_decl(token, false);
            opt_var_decls(token);
            return;
        }
        Token::ID(_) | Token::LBRACE | Token::SEMI | Token::RBRACE => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_var_decls"),
    }
}

fn opt_stmt_list(token: &mut Token) {
    match token {
        Token::ID(_) | Token::LBRACE | Token::SEMI => {
            stmt(token);
            opt_stmt_list(token);
            return;
        }
        Token::KW(kw) => {
            if *kw == String::from("return")
                || *kw == String::from("while")
                || *kw == String::from("if")
            {
                stmt(token);
                opt_stmt_list(token);
                return;
            }
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list");
        }
        Token::RBRACE => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list"),
    }
}

fn stmt(token: &mut Token) {
    match token {
        Token::ID(_) => {
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    fn_or_assg(token, &mut s);
                }
                _ => std::process::exit(1),
            }
            match_token(token, Token::SEMI);
            return;
        }
        Token::KW(kw) => {
            if *kw == String::from("return") {
                return_stmt(token);
            } else if *kw == String::from("while") {
                while_stmt(token);
            } else if *kw == String::from("if") {
                if_stmt(token);
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt");
            }
            return;
        }
        Token::LBRACE => {
            match_token(token, Token::LBRACE);
            opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return;
        }
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt"),
    }
}

fn fn_or_assg(token: &mut Token, id: &mut String) {
    match *token {
        Token::ASSG => {
            {
                let mut sym = symbols.lock().unwrap();
                if *super::chk_decl.lock().unwrap()
                    && !sym.global_var_def(id)
                    && !sym.body_var_param_def(id)
                {
                    eprintln!("cannot assign to a var that has not been defined: {}", id);
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_or_assg");
                }
            }
            assg_stmt(token);
            return;
        }
        Token::LPAREN => {
            fn_call(token, id);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_or_assg"),
    }
}

fn if_stmt(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("if") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt");
            }
            match_token(token, Token::KW(String::from("if")));
            match_token(token, Token::LPAREN);
            or_exp(token);
            match_token(token, Token::RPAREN);
            stmt(token);
            opt_else(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt"),
    }
}

fn opt_else(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw == String::from("if")
                || *kw == String::from("return")
                || *kw == String::from("while")
            {
                return;
            }
            if *kw != String::from("else") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else");
            }
            match_token(token, Token::KW(String::from("else")));
            stmt(token);
            return;
        }
        Token::ID(_) | Token::LBRACE | Token::SEMI | Token::RBRACE => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else"),
    }
}

fn while_stmt(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("while") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt");
            }
            match_token(token, Token::KW(String::from("while")));
            match_token(token, Token::LPAREN);
            or_exp(token);
            match_token(token, Token::RPAREN);
            stmt(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt"),
    }
}

fn return_stmt(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("return") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt");
            }
            match_token(token, Token::KW(String::from("return")));
            opt_arith_exp(token);
            match_token(token, Token::SEMI);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt"),
    }
}

fn assg_stmt(token: &mut Token) {
    match *token {
        Token::ASSG => {
            match_token(token, Token::ASSG);
            addsub_exp(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "assg_stmt"),
    }
}

fn opt_fn_call(token: &mut Token, id: &mut String) {
    match *token {
        Token::LPAREN => {
            fn_call(token, id);
            return;
        }
        Token::ARITH(_) | Token::BOOL(_) | Token::RPAREN | Token::COMMA | Token::SEMI => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_fn_call"),
    }
}

fn fn_call(token: &mut Token, id: &mut String) {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let mut nargs = 0;
            opt_expr_list(token, &mut nargs);
            if *super::chk_decl.lock().unwrap()
                && !symbols.lock().unwrap().function_def(id, &mut nargs)
            {
                eprintln!(
                    "cannot call a function that has not been defined: {}, {}",
                    id, nargs
                );
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call");
            }
            match_token(token, Token::RPAREN);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call"),
    }
}

fn opt_expr_list(token: &mut Token, nargs: &mut u32) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            addsub_exp(token);
            *nargs = *nargs + 1;
            expr_list(token, nargs);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list");
            }
            addsub_exp(token);
            *nargs = *nargs + 1;
            expr_list(token, nargs);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list"),
    }
}

fn expr_list(token: &mut Token, nargs: &mut u32) {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            addsub_exp(token);
            *nargs = *nargs + 1;
            expr_list(token, nargs);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "expr_list"),
    }
}

fn or_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            and_exp(token);
            or_no_lr(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp");
            }
            and_exp(token);
            or_no_lr(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp"),
    }
}

fn or_no_lr(token: &mut Token) {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("||") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr");
            }
            match_token(token, Token::BOOL(String::from("||")));
            and_exp(token);
            or_no_lr(token);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr"),
    }
}

fn and_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            bool_exp(token);
            and_no_lr(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp");
            }
            bool_exp(token);
            and_no_lr(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp"),
    }
}

fn and_no_lr(token: &mut Token) {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("&&") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr");
            }
            match_token(token, Token::BOOL(String::from("&&")));
            bool_exp(token);
            and_no_lr(token);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr"),
    }
}

fn bool_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            addsub_exp(token);
            relop(token);
            addsub_exp(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp");
            }
            addsub_exp(token);
            relop(token);
            addsub_exp(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp"),
    }
}

fn opt_arith_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            addsub_exp(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp");
            }
            addsub_exp(token);
            return;
        }
        Token::SEMI => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp"),
    }
}

fn addsub_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            muldiv_exp(token);
            addsub_no_lr(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp");
            }
            muldiv_exp(token);
            addsub_no_lr(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp"),
    }
}

fn addsub_no_lr(token: &mut Token) {
    match token {
        Token::ARITH(ar) => {
            if *ar == String::from("+") {
                match_token(token, Token::ARITH(String::from("+")));
            } else if *ar == String::from("-") {
                match_token(token, Token::ARITH(String::from("-")));
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr");
            }
            muldiv_exp(token);
            addsub_no_lr(token);
            return;
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr"),
    }
}

fn muldiv_exp(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            arith_exp(token);
            muldiv_no_lr(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp");
            }
            arith_exp(token);
            muldiv_no_lr(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp"),
    }
}

fn muldiv_no_lr(token: &mut Token) {
    match token {
        Token::ARITH(ar) => {
            if *ar == String::from("*") {
                match_token(token, Token::ARITH(String::from("*")));
            } else if *ar == String::from("/") {
                match_token(token, Token::ARITH(String::from("/")));
            } else {
                return;
            }
            arith_exp(token);
            muldiv_no_lr(token);
            return;
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_no_lr"),
    }
}

fn arith_exp(token: &mut Token) {
    match token {
        Token::ID(_) => {
            match match_token(token, Token::ID(String::new())) {
                Token::ID(mut s) => {
                    opt_fn_call(token, &mut s);
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
            }
            return;
        }
        Token::INTCONST(_) => {
            match_token(token, Token::INTCONST(0));
            return;
        }
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            addsub_exp(token);
            match_token(token, Token::RPAREN);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp");
            }
            match_token(token, Token::ARITH(String::from("-")));
            arith_exp(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
    }
}

fn relop(token: &mut Token) {
    match token {
        Token::BOOL(op) => {
            match op.as_str() {
                ">" => match_token(token, Token::BOOL(String::from(">"))),
                ">=" => match_token(token, Token::BOOL(String::from(">="))),
                "<" => match_token(token, Token::BOOL(String::from("<"))),
                "<=" => match_token(token, Token::BOOL(String::from("<="))),
                "!=" => match_token(token, Token::BOOL(String::from("!="))),
                "==" => match_token(token, Token::BOOL(String::from("=="))),
                _ => {
                    error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop");
                    return;
                }
            };
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop"),
    }
}
