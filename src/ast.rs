/*




*/

use crate::asm_gen::Quad;
use crate::symbol_table::Entry;

#[derive(PartialEq, Clone, Debug)]
pub enum ASTNode<'a> {
    FUNC_DEFN {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    FUNC_CALL {
        name: String,
        args: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    STMT_LIST {
        head: Box<ASTNode<'a>>,
        next: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    EXPR_LIST {
        head: Box<ASTNode<'a>>,
        next: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    INTCONST {
        val: i32,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    ID {
        name: String,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    BOOL {
        op: String,
        op1: Box<ASTNode<'a>>,
        op2: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    ARITH {
        op: String,
        op1: Box<ASTNode<'a>>,
        op2: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    ASSG {
        op1: String,
        op2: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    IF {
        condition: Box<ASTNode<'a>>,
        then_stmt: Box<ASTNode<'a>>,
        else_stmt: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    WHILE {
        condition: Box<ASTNode<'a>>,
        body: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    RETURN {
        expr: Box<ASTNode<'a>>,
        code: Quad<'a>,
        place: Entry<'a>,
    },
    NULL,
    DUMMY(std::marker::PhantomData<&'a ()>), // do not use
}

impl<'a> ASTNode<'a> {
    pub fn new_FUNC_DEFN(name: String, params: Vec<String>, body: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::FUNC_DEFN {
            name,
            params,
            body: Box::new(body),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_FUNC_CALL(name: String, args: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::FUNC_CALL {
            name,
            args: Box::new(args),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_STMT_LIST(head: ASTNode<'a>, next: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::STMT_LIST {
            head: Box::new(head),
            next: Box::new(next),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_EXPR_LIST(head: ASTNode<'a>, next: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::EXPR_LIST {
            head: Box::new(head),
            next: Box::new(next),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_INTCONST(val: i32) -> ASTNode<'a> {
        ASTNode::INTCONST {
            val,
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_ID(name: String) -> ASTNode<'a> {
        ASTNode::ID {
            name,
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_BOOL(op: String, op1: ASTNode<'a>, op2: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::BOOL {
            op,
            op1: Box::new(op1),
            op2: Box::new(op2),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_ARITH(op: String, op1: ASTNode<'a>, op2: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::ARITH {
            op,
            op1: Box::new(op1),
            op2: Box::new(op2),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_ASSG(op1: String, op2: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::ASSG {
            op1,
            op2: Box::new(op2),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_IF(
        condition: ASTNode<'a>,
        then_stmt: ASTNode<'a>,
        else_stmt: ASTNode<'a>,
    ) -> ASTNode<'a> {
        ASTNode::IF {
            condition: Box::new(condition),
            then_stmt: Box::new(then_stmt),
            else_stmt: Box::new(else_stmt),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_WHILE(condition: ASTNode<'a>, body: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::WHILE {
            condition: Box::new(condition),
            body: Box::new(body),
            code: Quad::NULL,
            place: Entry::NULL,
        }
    }
    pub fn new_RETURN(expr: ASTNode<'a>) -> ASTNode<'a> {
        ASTNode::RETURN {
            expr: Box::new(expr),
            code: Quad::NULL,
            place: Entry::NULL,
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
            _ => {}
        }
    }

    pub fn code(&mut self) -> Quad<'a> {
        match self {
            ASTNode::FUNC_DEFN { code, .. }
            | ASTNode::FUNC_CALL { code, .. }
            | ASTNode::STMT_LIST { code, .. }
            | ASTNode::IF { code, .. }
            | ASTNode::ASSG { code, .. }
            | ASTNode::WHILE { code, .. }
            | ASTNode::RETURN { code, .. }
            | ASTNode::EXPR_LIST { code, .. }
            | ASTNode::ID { code, .. }
            | ASTNode::INTCONST { code, .. }
            | ASTNode::ARITH { code, .. }
            | ASTNode::BOOL { code, .. } => code.clone(),
            _ => Quad::NULL,
        }
    }
    pub fn set_code(&mut self, newcode: Quad<'a>) {
        match self {
            ASTNode::FUNC_DEFN { code, .. }
            | ASTNode::FUNC_CALL { code, .. }
            | ASTNode::STMT_LIST { code, .. }
            | ASTNode::IF { code, .. }
            | ASTNode::ASSG { code, .. }
            | ASTNode::WHILE { code, .. }
            | ASTNode::RETURN { code, .. }
            | ASTNode::EXPR_LIST { code, .. }
            | ASTNode::ID { code, .. }
            | ASTNode::INTCONST { code, .. }
            | ASTNode::ARITH { code, .. }
            | ASTNode::BOOL { code, .. } => *code = newcode,
            _ => {}
        }
    }
    pub fn place(&mut self) -> Entry<'a> {
        match self {
            ASTNode::FUNC_DEFN { place, .. }
            | ASTNode::FUNC_CALL { place, .. }
            | ASTNode::STMT_LIST { place, .. }
            | ASTNode::IF { place, .. }
            | ASTNode::ASSG { place, .. }
            | ASTNode::WHILE { place, .. }
            | ASTNode::RETURN { place, .. }
            | ASTNode::EXPR_LIST { place, .. }
            | ASTNode::ID { place, .. }
            | ASTNode::INTCONST { place, .. }
            | ASTNode::ARITH { place, .. }
            | ASTNode::BOOL { place, .. } => place.clone(),
            _ => Entry::NULL,
        }
    }
    pub fn set_place(&mut self, newplace: Entry<'a>) {
        match self {
            ASTNode::FUNC_DEFN { place, .. }
            | ASTNode::FUNC_CALL { place, .. }
            | ASTNode::STMT_LIST { place, .. }
            | ASTNode::IF { place, .. }
            | ASTNode::ASSG { place, .. }
            | ASTNode::WHILE { place, .. }
            | ASTNode::RETURN { place, .. }
            | ASTNode::EXPR_LIST { place, .. }
            | ASTNode::ID { place, .. }
            | ASTNode::INTCONST { place, .. }
            | ASTNode::ARITH { place, .. }
            | ASTNode::BOOL { place, .. } => *place = newplace,
            _ => {}
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
