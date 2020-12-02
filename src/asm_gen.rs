/*





*/
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;

use crate::ast::ASTNode;
use crate::symbol_table::Entry;
use crate::symbol_table::SymbolTable;

pub fn generate<'a>(node: &mut ASTNode, symbols: &'a mut SymbolTable<'a>) {
    let mut list = Quad::generate(node, symbols);
    println!("\n# ------------------------------------------\n");
    println!(".text\n.align 2\n");

    while list != Quad::NULL {
        match list {
            Quad::ARITH {
                op,
                src1,
                src2,
                dest,
                next,
            } => {
                match op.as_str() {
                    "UMINUS" => {}
                    "+" => {}
                    "-" => {}
                    "*" => {}
                    "/" => {}
                    _ => std::process::exit(1),
                }

                list = *next;
            }
            Quad::BOOL {
                op,
                src1,
                src2,
                dest,
                next,
            } => {
                match op.as_str() {
                    "!=" => {}
                    "==" => {}
                    ">" => {}
                    ">=" => {}
                    "<" => {}
                    "<=" => {}
                    _ => std::process::exit(1),
                }

                list = *next;
            }
            Quad::ASSG {
                inv,
                mut src1,
                mut dest,
                next,
            } => {
                if inv {
                } else {
                    // three address code
                    println!("# {} = {}", dest.string(), src1.string());

                    // MIPS code
                    match src1 {
                        Operand::INTCONST(v) => println!("li $t1, {}", v),
                        Operand::ENTRY(_) => println!("lw $t1, {}", src1.string()),
                    }
                    println!("sw $t1, {}", dest.string());

                    /*
                    if (quad->dest.val.entry->param_num == -10)
                        printf("sw $t1, _%s\n", quad->dest.val.entry->id);
                    else
                        printf("sw $t1, %d($fp)\n", quad->dest.val.entry->frame_offset);
                    printf("\n");
                    */
                }
                list = *next;
            }
            Quad::GOTO { dest, next } => {
                list = *next;
            }
            Quad::LABEL { tag, next } => {
                list = *next;
            }
            Quad::ENTER { name, next } => {
                list = *next;
            }
            Quad::LEAVE { name, next } => {
                list = *next;
            }
            Quad::PARAM { name, next } => {
                list = *next;
            }
            Quad::CALL { name, num, next } => {
                list = *next;
            }
            Quad::RETURN { src1, next } => {
                list = *next;
            }
            Quad::RETRIEVE { next } => {
                list = *next;
            }
            Quad::NULL => std::process::exit(1),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand<'a> {
    ENTRY(Entry<'a>),
    SYM(SymbolTable<'a>),
    INTCONST(i32),
}

impl<'a> Operand<'a> {
    fn string(&mut self) -> String {
        match self {
            Operand::ENTRY(e) => e.string(),
            Operand::SYM(e) => e.string(),
            Operand::INTCONST(val) => String::from(format!("{}", val)),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Quad<'a> {
    ARITH {
        op: String,
        src1: Operand<'a>,
        src2: Operand<'a>,
        dest: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    ASSG {
        inv: bool,
        src1: Operand<'a>,
        dest: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    BOOL {
        op: String,
        src1: Operand<'a>,
        src2: Operand<'a>,
        dest: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    GOTO {
        dest: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    LABEL {
        tag: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    ENTER {
        name: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    LEAVE {
        name: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    PARAM {
        name: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    CALL {
        name: Operand<'a>,
        num: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    RETURN {
        src1: Operand<'a>,
        next: Box<Quad<'a>>,
    },
    RETRIEVE {
        next: Box<Quad<'a>>,
    },
    NULL,
}

lazy_static! {
    static ref tmp_num: Mutex<i32> = Mutex::new(0);
    pub static ref label_num: Mutex<i32> = Mutex::new(0);
}

impl<'a> Quad<'a> {
    fn new_ARITH(op: String, src1: Operand<'a>, src2: Operand<'a>, dest: Operand<'a>) -> Quad<'a> {
        Quad::ARITH {
            op,
            src1,
            src2,
            dest,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_BOOL(op: String, src1: Operand<'a>, src2: Operand<'a>, dest: Operand<'a>) -> Quad<'a> {
        Quad::BOOL {
            op,
            src1,
            src2,
            dest,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_ASSG(inv: bool, src1: Operand<'a>, dest: Operand<'a>) -> Quad<'a> {
        Quad::ASSG {
            inv,
            src1,
            dest,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_GOTO(dest: Operand<'a>) -> Quad<'a> {
        Quad::GOTO {
            dest,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_LABEL() -> Quad<'a> {
        Quad::LABEL {
            tag: Operand::INTCONST(FA_label(1)),
            next: Box::new(Quad::NULL),
        }
    }
    fn new_ENTER(name: Operand<'a>) -> Quad<'a> {
        Quad::ENTER {
            name,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_LEAVE(name: Operand<'a>) -> Quad<'a> {
        Quad::LEAVE {
            name,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_PARAM(name: Operand<'a>) -> Quad<'a> {
        Quad::PARAM {
            name,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_CALL(name: Operand<'a>, num: Operand<'a>) -> Quad<'a> {
        Quad::CALL {
            name,
            num,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_RETURN(src1: Operand<'a>) -> Quad<'a> {
        Quad::RETURN {
            src1,
            next: Box::new(Quad::NULL),
        }
    }
    fn new_RETRIEVE() -> Quad<'a> {
        Quad::RETRIEVE {
            next: Box::new(Quad::NULL),
        }
    }

    fn append(&mut self, other: &mut Quad<'a>) {
        match self {
            Quad::ARITH { next, .. }
            | Quad::ASSG { next, .. }
            | Quad::BOOL { next, .. }
            | Quad::GOTO { next, .. }
            | Quad::LABEL { next, .. }
            | Quad::ENTER { next, .. }
            | Quad::LEAVE { next, .. }
            | Quad::PARAM { next, .. }
            | Quad::CALL { next, .. }
            | Quad::RETURN { next, .. }
            | Quad::RETRIEVE { next, .. } => {
                if **next == Quad::NULL {
                    *next = Box::new(other.clone());
                } else {
                    (*next).append(other);
                }
            }
            _ => {}
        }
    }

    fn newtmp(symbols: &'a mut SymbolTable<'a>) -> Option<&'a mut Entry<'a>> {
        let mut name = format!("_tmp{}", FA_tmp(1));
        symbols.add_body_var(&mut name);
        return symbols.get_body_var(&mut name);
    }
    /* this is the three address code generator */
    fn generate(node: &mut ASTNode<'a>, symbols: &'a mut SymbolTable<'a>) -> Quad<'a> {
        genCode_FUNC_DEFN(node, symbols);
        return node.code();
    }
}

fn genCode_FUNC_DEFN<'a>(node: &mut ASTNode<'a>, symbols: &'a mut SymbolTable<'a>) {
    match node {
        ASTNode::FUNC_DEFN {
            name,
            params,
            body,
            code,
            place,
        } => {
            let func = symbols.get_function(name, &mut (params.len() as u32));
            let func_end = Quad::new_LABEL();
            genCode_STMT_LIST(&mut **body, func_end, symbols);
            if **body != ASTNode::NULL {
                *code = body.code();
                (*code).append(&mut func_end);
                (*code).append(&mut Quad::new_LEAVE(Operand::SYM(*func.unwrap())));
            } else {
                *code = func_end;
                (*code).append(&mut Quad::new_LEAVE(Operand::SYM(*func.unwrap())));
            }
        }
        _ => std::process::exit(1),
    }
}

fn genCode_STMT_LIST<'a>(
    node: &mut ASTNode,
    end_label: Quad<'a>,
    symbols: &'a mut SymbolTable<'a>,
) {
}

fn FA_label(off: i32) -> i32 {
    let mut out = label_num.lock().unwrap();
    *out = *out + off;
    return *out - off;
}

fn FA_tmp(off: i32) -> i32 {
    let mut out = tmp_num.lock().unwrap();
    *out = *out + off;
    return *out - off;
}
