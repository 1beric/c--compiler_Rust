/*
 * ./src/ast.rs
 * Brandon Erickson --- brandonscotterickson@gmail.com
 * This file implements the scanner for the C-- language. The language is defined at
 * http://www2.cs.arizona.edu/classes/cs453/fall20/PROJECT/SPEC/cminusminusspec.html#lexical
 * This file contains the Abstract Syntax Tree implementation for C--
 */

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode {
    FUNC_DEFN {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
    },
    FUNC_CALL {
        name: String,
        args: Box<ASTNode>,
    },
    STMT_LIST {
        head: Box<ASTNode>,
        next: Box<ASTNode>,
    },
    EXPR_LIST {
        head: Box<ASTNode>,
        next: Box<ASTNode>,
    },
    INTCONST {
        val: i32,
    },
    ID {
        name: String,
    },
    BOOL {
        op: String,
        op1: Box<ASTNode>,
        op2: Box<ASTNode>,
    },
    ARITH {
        op: String,
        op1: Box<ASTNode>,
        op2: Box<ASTNode>,
    },
    ASSG {
        op1: String,
        op2: Box<ASTNode>,
    },
    IF {
        condition: Box<ASTNode>,
        then_stmt: Box<ASTNode>,
        else_stmt: Box<ASTNode>,
    },
    WHILE {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    RETURN {
        expr: Box<ASTNode>,
    },
    NULL,
}

impl ASTNode {
    pub fn new_FUNC_DEFN(name: String, params: Vec<String>, body: ASTNode) -> ASTNode {
        ASTNode::FUNC_DEFN {
            name,
            params,
            body: Box::new(body),
        }
    }
    pub fn new_FUNC_CALL(name: String, args: ASTNode) -> ASTNode {
        ASTNode::FUNC_CALL {
            name,
            args: Box::new(args),
        }
    }
    pub fn new_STMT_LIST(head: ASTNode, next: ASTNode) -> ASTNode {
        ASTNode::STMT_LIST {
            head: Box::new(head),
            next: Box::new(next),
        }
    }
    pub fn new_EXPR_LIST(head: ASTNode, next: ASTNode) -> ASTNode {
        ASTNode::EXPR_LIST {
            head: Box::new(head),
            next: Box::new(next),
        }
    }
    pub fn new_INTCONST(val: i32) -> ASTNode {
        ASTNode::INTCONST { val }
    }
    pub fn new_ID(name: String) -> ASTNode {
        ASTNode::ID { name }
    }
    pub fn new_BOOL(op: String, op1: ASTNode, op2: ASTNode) -> ASTNode {
        ASTNode::BOOL {
            op,
            op1: Box::new(op1),
            op2: Box::new(op2),
        }
    }
    pub fn new_ARITH(op: String, op1: ASTNode, op2: ASTNode) -> ASTNode {
        ASTNode::ARITH {
            op,
            op1: Box::new(op1),
            op2: Box::new(op2),
        }
    }
    pub fn new_ASSG(op1: String, op2: ASTNode) -> ASTNode {
        ASTNode::ASSG {
            op1,
            op2: Box::new(op2),
        }
    }
    pub fn new_IF(condition: ASTNode, then_stmt: ASTNode, else_stmt: ASTNode) -> ASTNode {
        ASTNode::IF {
            condition: Box::new(condition),
            then_stmt: Box::new(then_stmt),
            else_stmt: Box::new(else_stmt),
        }
    }
    pub fn new_WHILE(condition: ASTNode, body: ASTNode) -> ASTNode {
        ASTNode::WHILE {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
    pub fn new_RETURN(expr: ASTNode) -> ASTNode {
        ASTNode::RETURN {
            expr: Box::new(expr),
        }
    }

    pub fn print(&mut self) {
        self.print_format(0, true);
    }
    fn print_format(&mut self, n: u32, nl: bool) {
        let indent_amt = n * 4;
        match self {
            ASTNode::FUNC_DEFN {
                name, params, body, ..
            } => {
                println!("FUNC_DEFN: {}", name);
                print!("  formals: ");
                for p_index in 0..params.len() {
                    print!("{}", params[p_index]);
                    if p_index < params.len() - 1 {
                        print!(", ");
                    }
                }
                println!("\n  body:");
                body.print_format(n + 1, true);
                println!("/* end FUNC_DEFN: {} */", name);
            }
            ASTNode::FUNC_CALL { name, args, .. } => {
                indent(indent_amt);
                print!("{}(", name);
                args.print_format(0, false);
                print!(")");
                if nl {
                    println!();
                }
            }
            ASTNode::STMT_LIST { head, next, .. } => {
                indent(indent_amt);
                // println!("{{");
                head.print_format(n, nl);
                next.print_format(n, nl);
                // indent(indent_amt);
                // println!("}}");
            }
            ASTNode::IF {
                condition,
                then_stmt,
                else_stmt,
                ..
            } => {
                indent(indent_amt);
                print!("if (");
                condition.print_format(0, false);
                println!("):");
                indent(indent_amt);
                println!("then:");
                then_stmt.print_format(n + 1, nl);
                indent(indent_amt);
                println!("else:");
                else_stmt.print_format(n + 1, nl);
                indent(indent_amt);
                println!("/* end IF */");
            }
            ASTNode::ASSG { op1, op2, .. } => {
                indent(indent_amt);
                print!("{} = ", op1);
                op2.print_format(0, false);
                println!();
            }
            ASTNode::WHILE {
                condition, body, ..
            } => {
                indent(indent_amt);
                print!("while (");
                condition.print_format(0, false);
                println!("):");
                body.print_format(n + 1, true);
                indent(indent_amt + 4);
                println!("/* end WHILE */");
            }
            ASTNode::RETURN { expr, .. } => {
                indent(indent_amt);
                print!("return: ");
                expr.print_format(0, false);
                println!();
            }
            ASTNode::EXPR_LIST { head, next, .. } => {
                head.print_format(0, false);
                match **next {
                    ASTNode::NULL => {}
                    _ => print!(", "),
                }
                next.print_format(0, false);
            }
            ASTNode::ID { name, .. } => {
                print!("{}", name);
            }
            ASTNode::INTCONST { val, .. } => {
                print!("{}", val);
            }
            ASTNode::ARITH { op, op1, op2, .. } => match (*op).as_str() {
                "UMINUS" => {
                    print!("-(");
                    op1.print_format(0, false);
                    print!(")");
                }
                "-" | "+" | "*" | "/" => {
                    print!("(");
                    op1.print_format(0, false);
                    print!(" {} ", *op);
                    op2.print_format(0, false);
                    print!(")");
                }
                _ => print!("({})", *op),
            },
            ASTNode::BOOL { op, op1, op2, .. } => match (*op).as_str() {
                "==" | "!=" | ">" | ">=" | "<" | "<=" => {
                    op1.print_format(0, false);
                    print!(" {} ", *op);
                    op2.print_format(0, false);
                }
                "&&" | "||" => {
                    print!("(");
                    op1.print_format(0, false);
                    print!(") {} (", *op);
                    op2.print_format(0, false);
                    print!(")");
                }
                _ => print!("({})", *op),
            },
            ASTNode::NULL => {
                return;
            }
        }
    }
}
fn indent(num: u32) {
    let mut n = num;
    while n > 0 {
        n -= 1;
        print!("{}", ' ');
    }
}
