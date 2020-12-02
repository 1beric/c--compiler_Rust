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

use crate::asm_gen;
use crate::ast::ASTNode;
use crate::error;
use crate::scanner;
use crate::scanner::Token;
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

    if *super::gen_code.lock().unwrap() {
        println!(".align 2\n.data\n_nl :.asciiz \"\\n\"\n.align 2\n.text\n#println : print out an integer followed by a newline\n_println :\nli $v0, 1\nlw $a0, 0($sp)\nsyscall\nli $v0, 4\nla $a0, _nl\nsyscall\njr $ra\n");
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

fn prog<'a>(token: &mut Token) {
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
    };
    return;
}

fn func_var<'a>(token: &mut Token, id: &mut String) {
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
        }
        Token::LPAREN => {
            let mut root = func_defn(token, id);
            if *super::print_ast.lock().unwrap() {
                root.print();
            }
            if *super::gen_code.lock().unwrap() {
                asm_gen::generate(&mut root);
            }
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_var"),
    };
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

fn func_defn<'a>(token: &mut Token, id: &mut String) -> ASTNode<'a> {
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
            for mut param in params.clone() {
                symbols.lock().unwrap().add_param(&mut param);
            }
            match_token(token, Token::RPAREN);
            match_token(token, Token::LBRACE);
            opt_var_decls(token);
            let body = opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return ASTNode::new_FUNC_DEFN(id.clone(), params, body);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "func_defn"),
    };
    return ASTNode::NULL;
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

fn opt_stmt_list<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::LBRACE | Token::SEMI => {
            let head = stmt(token);
            let next = opt_stmt_list(token);
            if head == ASTNode::NULL {
                return next;
            }
            return ASTNode::new_STMT_LIST(head, next);
        }
        Token::KW(kw) => {
            if *kw == String::from("return")
                || *kw == String::from("while")
                || *kw == String::from("if")
            {
                let head = stmt(token);
                let next = opt_stmt_list(token);
                if head == ASTNode::NULL {
                    return next;
                }
                return ASTNode::new_STMT_LIST(head, next);
            }
            error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list");
        }
        Token::RBRACE => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_stmt_list"),
    };
    return ASTNode::NULL;
}

fn stmt<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) => match match_token(token, Token::ID(String::new())) {
            Token::ID(mut s) => {
                let node = fn_or_assg(token, &mut s);
                match_token(token, Token::SEMI);
                return node;
            }
            _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt"),
        },
        Token::KW(kw) => {
            if *kw == String::from("return") {
                return return_stmt(token);
            } else if *kw == String::from("while") {
                return while_stmt(token);
            } else if *kw == String::from("if") {
                return if_stmt(token);
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt");
            }
        }
        Token::LBRACE => {
            match_token(token, Token::LBRACE);
            let list = opt_stmt_list(token);
            match_token(token, Token::RBRACE);
            return list;
        }
        Token::SEMI => {
            match_token(token, Token::SEMI);
            return ASTNode::NULL;
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "stmt"),
    };
    return ASTNode::NULL;
}

fn fn_or_assg<'a>(token: &mut Token, id: &mut String) -> ASTNode<'a> {
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
            return assg_stmt(token, id.clone());
        }
        Token::LPAREN => {
            return fn_call(token, id);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_or_assg"),
    };
    return ASTNode::NULL;
}

fn if_stmt<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("if") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt");
            }
            match_token(token, Token::KW(String::from("if")));
            match_token(token, Token::LPAREN);
            let condition = or_exp(token);
            match_token(token, Token::RPAREN);
            let then_stmt = stmt(token);
            let else_stmt = opt_else(token);
            return ASTNode::new_IF(condition, then_stmt, else_stmt);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "if_stmt"),
    }
    return ASTNode::NULL;
}

fn opt_else<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::KW(kw) => {
            if *kw == String::from("if")
                || *kw == String::from("return")
                || *kw == String::from("while")
            {
                return ASTNode::NULL;
            }
            if *kw != String::from("else") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else");
            }
            match_token(token, Token::KW(String::from("else")));
            return stmt(token);
        }
        Token::ID(_) | Token::LBRACE | Token::SEMI | Token::RBRACE => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_else"),
    };
    return ASTNode::NULL;
}

fn while_stmt<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("while") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt");
            }
            match_token(token, Token::KW(String::from("while")));
            match_token(token, Token::LPAREN);
            let condition = or_exp(token);
            match_token(token, Token::RPAREN);
            let body = stmt(token);
            return ASTNode::new_WHILE(condition, body);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "while_stmt"),
    }
    return ASTNode::NULL;
}

fn return_stmt<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::KW(kw) => {
            if *kw != String::from("return") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt");
            }
            match_token(token, Token::KW(String::from("return")));
            let expr = opt_arith_exp(token);
            match_token(token, Token::SEMI);
            return ASTNode::new_RETURN(expr);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "return_stmt"),
    }
    return ASTNode::NULL;
}

fn assg_stmt<'a>(token: &mut Token, id: String) -> ASTNode<'a> {
    match *token {
        Token::ASSG => {
            match_token(token, Token::ASSG);
            return ASTNode::new_ASSG(id, addsub_exp(token));
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "assg_stmt"),
    }
    return ASTNode::NULL;
}

fn opt_fn_call<'a>(token: &mut Token, id: &mut String) -> ASTNode<'a> {
    match *token {
        Token::LPAREN => {
            return fn_call(token, id);
        }
        Token::ARITH(_) | Token::BOOL(_) | Token::RPAREN | Token::COMMA | Token::SEMI => {
            return ASTNode::new_ID(id.clone());
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_fn_call"),
    }
    return ASTNode::new_ID(id.clone());
}

fn fn_call<'a>(token: &mut Token, id: &mut String) -> ASTNode<'a> {
    match *token {
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let mut nargs = 0;
            let args = opt_expr_list(token, &mut nargs);
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
            return ASTNode::new_FUNC_CALL(id.clone(), args);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "fn_call"),
    }
    return ASTNode::NULL;
}

fn opt_expr_list<'a>(token: &mut Token, nargs: &mut u32) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list");
            }
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::RPAREN => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_expr_list"),
    }
    return ASTNode::NULL;
}

fn expr_list<'a>(token: &mut Token, nargs: &mut u32) -> ASTNode<'a> {
    match *token {
        Token::COMMA => {
            match_token(token, Token::COMMA);
            let head = addsub_exp(token);
            *nargs = *nargs + 1;
            let next = expr_list(token, nargs);
            return ASTNode::new_EXPR_LIST(head, next);
        }
        Token::RPAREN => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "expr_list"),
    }
    return ASTNode::NULL;
}

fn or_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = and_exp(token);
            return or_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp");
            }
            let left = and_exp(token);
            return or_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_exp"),
    }
    return ASTNode::NULL;
}

fn or_no_lr<'a>(token: &mut Token, left: ASTNode<'a>) -> ASTNode<'a> {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("||") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr");
            }
            match_token(token, Token::BOOL(String::from("||")));
            let and_expr = and_exp(token);
            return or_no_lr(token, ASTNode::new_BOOL(String::from("||"), left, and_expr));
        }
        Token::RPAREN => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "or_no_lr"),
    }
    return left;
}

fn and_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let bool_expr = bool_exp(token);
            return and_no_lr(token, bool_expr);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp");
            }
            let bool_expr = bool_exp(token);
            return and_no_lr(token, bool_expr);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_exp"),
    }
    return ASTNode::NULL;
}

fn and_no_lr<'a>(token: &mut Token, left: ASTNode<'a>) -> ASTNode<'a> {
    match token {
        Token::BOOL(bl) => {
            if *bl != String::from("&&") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr");
            }
            match_token(token, Token::BOOL(String::from("&&")));
            let bool_expr = bool_exp(token);
            return and_no_lr(
                token,
                ASTNode::new_BOOL(String::from("&&"), left, bool_expr),
            );
        }
        Token::RPAREN => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "and_no_lr"),
    }
    return left;
}

fn bool_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let op1 = addsub_exp(token);
            let op = relop(token);
            let op2 = addsub_exp(token);
            return ASTNode::new_BOOL(op, op1, op2);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp");
            }
            let op1 = addsub_exp(token);
            let op = relop(token);
            let op2 = addsub_exp(token);
            return ASTNode::new_BOOL(op, op1, op2);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "bool_exp"),
    }
    return ASTNode::NULL;
}

fn opt_arith_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            return addsub_exp(token);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp");
            }
            return addsub_exp(token);
        }
        Token::SEMI => return ASTNode::NULL,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "opt_arith_exp"),
    }
    return ASTNode::NULL;
}

fn addsub_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = muldiv_exp(token);
            return addsub_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp");
            }
            let left = muldiv_exp(token);
            return addsub_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_exp"),
    }
    return ASTNode::NULL;
}

fn addsub_no_lr<'a>(token: &mut Token, left: ASTNode<'a>) -> ASTNode<'a> {
    match token.clone() {
        Token::ARITH(ar) => {
            if *ar == String::from("+") {
                match_token(token, Token::ARITH(String::from("+")));
            } else if *ar == String::from("-") {
                match_token(token, Token::ARITH(String::from("-")));
            } else {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr");
            }
            let op2 = muldiv_exp(token);
            return addsub_no_lr(token, ASTNode::new_ARITH(ar.clone(), left, op2));
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "addsub_no_lr"),
    }
    return left;
}

fn muldiv_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token {
        Token::ID(_) | Token::INTCONST(_) | Token::LPAREN => {
            let left = arith_exp(token);
            return muldiv_no_lr(token, left);
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp");
            }
            let left = arith_exp(token);
            return muldiv_no_lr(token, left);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_exp"),
    }
    return ASTNode::NULL;
}

fn muldiv_no_lr<'a>(token: &mut Token, left: ASTNode<'a>) -> ASTNode<'a> {
    match token.clone() {
        Token::ARITH(ar) => {
            if *ar == String::from("*") {
                match_token(token, Token::ARITH(String::from("*")));
            } else if *ar == String::from("/") {
                match_token(token, Token::ARITH(String::from("/")));
            } else {
                return left;
            }
            let op2 = arith_exp(token);
            return muldiv_no_lr(token, ASTNode::new_ARITH(ar.clone(), left, op2));
        }
        Token::RPAREN | Token::COMMA | Token::SEMI | Token::BOOL(_) => return left,
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "muldiv_no_lr"),
    }
    return left;
}

fn arith_exp<'a>(token: &mut Token) -> ASTNode<'a> {
    match token.clone() {
        Token::ID(_) => match match_token(token, Token::ID(String::new())) {
            Token::ID(mut s) => {
                return opt_fn_call(token, &mut s);
            }
            _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
        },
        Token::INTCONST(val) => {
            match_token(token, Token::INTCONST(0));
            return ASTNode::new_INTCONST(val);
        }
        Token::LPAREN => {
            match_token(token, Token::LPAREN);
            let expr = addsub_exp(token);
            match_token(token, Token::RPAREN);
            return expr;
        }
        Token::ARITH(ar) => {
            if *ar != String::from("-") {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp");
            }
            match_token(token, Token::ARITH(String::from("-")));
            let expr = arith_exp(token);
            return ASTNode::new_ARITH(String::from("UMINUS"), expr, ASTNode::NULL);
        }
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "arith_exp"),
    };
    return ASTNode::NULL;
}

fn relop(token: &mut Token) -> String {
    match token.clone() {
        Token::BOOL(op) => match op.as_str() {
            ">" => {
                match_token(token, Token::BOOL(String::from(">")));
                return op.clone();
            }
            ">=" => {
                match_token(token, Token::BOOL(String::from(">=")));
                return op.clone();
            }
            "<" => {
                match_token(token, Token::BOOL(String::from("<")));
                return op.clone();
            }
            "<=" => {
                match_token(token, Token::BOOL(String::from("<=")));
                return op.clone();
            }
            "!=" => {
                match_token(token, Token::BOOL(String::from("!=")));
                return op.clone();
            }
            "==" => {
                match_token(token, Token::BOOL(String::from("==")));
                return op.clone();
            }
            _ => {
                error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop");
                return String::new();
            }
        },
        _ => error::print_err_rule(*scanner::line.lock().unwrap(), token, "relop"),
    };
    return String::new();
}
