/*
 *
 *
 *
 *
 *
 *
 *
 */

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::error;
use crate::scanner;
use crate::scanner::Token;

lazy_static! {
    // static ref token: Mutex<Token> = Mutex::new(Token::UNDEF);
}

pub fn parse() {
    let mut token = scanner::get_token();

    prog(&mut token);

    println!("finished!");
}

pub fn print_tokens() {
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
        Token::KW(kw) => println!("KW Token: {}", kw),
        Token::ARITH(op) => println!("ARITH Token: {}", op),
        Token::BOOL(op) => println!("BOOL Token: {}", op),
        Token::ID(name) => println!("ID Token: {}", name),
        Token::INTCONST(val) => println!("INTCONST Token: {}", val),
        _ => println!("{:?} Token", token),
    }
    return true;
}

fn match_token(token: &mut Token, to_match: Token) {
    // print_token(token);
    // print_token(&to_match);
    // println!();
    match *token {
        Token::ID(_) => {
            match to_match {
                Token::ID(_) => {
                    // valid
                    *token = scanner::get_token();
                    return;
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token 1"),
            }
        }
        Token::INTCONST(_) => {
            match to_match {
                Token::INTCONST(_) => {
                    // valid
                    *token = scanner::get_token();
                    return;
                }
                _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token 2"),
            }
        }
        _ => {
            if *token == to_match {
                // valid
                *token = scanner::get_token();
                return;
            }
            println!("{:?} {:?}", *token, to_match);
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "match_token 3");
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

            /*

            -------------------------- C CODE ---------------------------

            if (chk_decl_flag && def_locally(symbol_table, lexeme, ET_var))
            {
                sprintf(expected, "The compiler already defined %s", lexeme);
                ERR_message(token, expected);
            }

            ST_Entry *binding = add_id(symbol_table, lexeme, ET_var);

            -------------------------- C CODE ---------------------------

            */

            match_token(token, Token::ID(String::new()));
            func_var(token);
            prog(token);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "prog"),
    }
}

fn func_var(token: &mut Token) {
    match *token {
        Token::SEMI | Token::COMMA => {
            var_decl(token);
            return;
        }
        Token::LPAREN => {
            func_defn(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var"),
    }
}

fn var_decl(token: &mut Token) {
    match *token {
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return;
        }
        Token::COMMA => {
            match_token(token, Token::COMMA);
            match_token(token, Token::ID(String::new()));
            var_decl(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "var_decl"),
    }
}

fn mtype(token: &mut Token) {
    match_token(token, Token::KW(String::from("int")));
}

fn func_defn(token: &mut Token) {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            opt_formals(token);
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

fn opt_formals(token: &mut Token) {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("int") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals");
            }
            mtype(token);
            match_token(token, Token::ID(String::new()));
            formals(token);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_formals"),
    }
}

fn formals(token: &mut Token) {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            opt_formals(token);
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
            match_token(token, Token::ID(String::new()));
            var_decl(token);
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
            match_token(token, Token::ID(String::new()));
            fn_or_assg(token);
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

fn fn_or_assg(token: &mut Token) {
    match *token {
        Token::ASSG => {
            assg_stmt(token);
            return;
        }
        Token::LPAREN => {
            fn_call(token);
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

fn opt_fn_call(token: &mut Token) {
    match *token {
        Token::LPAREN => {
            fn_call(token);
            return;
        }
        Token::ARITH(_) | Token::BOOL(_) | Token::RPAREN | Token::COMMA | Token::SEMI => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_fn_call"),
    }
}

fn fn_call(token: &mut Token) {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            opt_expr_list(token);
            match_token(token, Token::RPAREN);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call"),
    }
}

fn opt_expr_list(token: &mut Token) {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            addsub_exp(token);
            expr_list(token);
            return;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list");
            }
            addsub_exp(token);
            expr_list(token);
            return;
        }
        Token::RPAREN => return,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list"),
    }
}

fn expr_list(token: &mut Token) {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            addsub_exp(token);
            expr_list(token);
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
            match_token(token, Token::ID(String::new()));
            opt_fn_call(token);
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
            match_token(token, Token::ARITH(String::from("+")));
            arith_exp(token);
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
    }
}

fn relop(token: &mut Token) {
    match &*token {
        Token::BOOL(op) => {
            match_token(token, Token::BOOL(op.to_string()));
            return;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop"),
    }
}
