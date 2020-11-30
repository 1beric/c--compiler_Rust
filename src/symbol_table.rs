#[derive(PartialEq, Debug)]
pub enum Entry<'a> {
    BODY_VAR {
        id: String,
        var_num: u32,
        frame_offset: i32,
        next: Option<Box<Entry<'a>>>,
    },
    PARAM_VAR {
        id: String,
        param_num: u32,
        frame_offset: i32,
        next: Option<Box<Entry<'a>>>,
    },
    GLOBAL_VAR {
        id: String,
        next: Option<Box<Entry<'a>>>,
    },
    NULL(std::marker::PhantomData<&'a ()>), // do not use
}

impl<'a> Entry<'a> {
    fn new_global(id: &mut String) -> Entry<'a> {
        Entry::GLOBAL_VAR {
            id: id.clone(),
            next: None,
        }
    }

    fn new_param(id: &mut String, param_num: u32) -> Entry<'a> {
        Entry::PARAM_VAR {
            id: id.clone(),
            param_num,
            frame_offset: 0,
            next: None,
        }
    }

    fn new_body_var(id: &mut String, var_num: u32) -> Entry<'a> {
        Entry::BODY_VAR {
            id: id.clone(),
            var_num,
            frame_offset: 0,
            next: None,
        }
    }

    fn add_global(&mut self, id: &mut String) {
        match self {
            Entry::GLOBAL_VAR {
                next: ref mut g, ..
            } => match g {
                Some(ref mut n) => n.add_global(id),
                None => *g = Some(Box::new(Entry::new_global(id))),
            },
            _ => std::process::exit(1),
        }
    }

    fn add_param(&mut self, id: &mut String) {
        self.priv_add_param(id, 1);
    }

    fn priv_add_param(&mut self, id: &mut String, param_num: u32) {
        match self {
            Entry::PARAM_VAR {
                next: ref mut g, ..
            } => match g {
                Some(ref mut n) => n.priv_add_param(id, param_num + 1),
                None => *g = Some(Box::new(Entry::new_param(id, param_num))),
            },
            _ => std::process::exit(1),
        }
    }

    fn add_body_var(&mut self, id: &mut String) {
        self.priv_add_body_var(id, 1);
    }

    fn priv_add_body_var(&mut self, id: &mut String, var_num: u32) {
        match self {
            Entry::BODY_VAR {
                next: ref mut g, ..
            } => match g {
                Some(ref mut n) => n.priv_add_body_var(id, var_num + 1),
                None => *g = Some(Box::new(Entry::new_body_var(id, var_num))),
            },
            _ => std::process::exit(1),
        }
    }

    fn var_def(&mut self, id: &mut String) -> bool {
        match self {
            Entry::BODY_VAR {
                next: ref mut g,
                id: var_id,
                ..
            } => {
                if *id == *var_id {
                    return true;
                }
                match g {
                    Some(ref mut n) => n.var_def(id),
                    None => return false,
                }
            }
            Entry::PARAM_VAR {
                next: ref mut g,
                id: var_id,
                ..
            } => {
                if *id == *var_id {
                    return true;
                }
                match g {
                    Some(ref mut n) => n.var_def(id),
                    None => return false,
                }
            }
            Entry::GLOBAL_VAR {
                next: ref mut g,
                id: var_id,
                ..
            } => {
                if *id == *var_id {
                    return true;
                }
                match g {
                    Some(ref mut n) => n.var_def(id),
                    None => return false,
                }
            }
            _ => std::process::exit(1),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SymbolTable<'a> {
    GLOBAL {
        globals: Option<Box<Entry<'a>>>,
        functions: Option<Box<SymbolTable<'a>>>,
    },
    FUNCTION {
        ntemps: u32,
        nparams: u32,
        name: String,
        params: Option<Box<Entry<'a>>>,
        body_vars: Option<Box<Entry<'a>>>,
        next: Option<Box<SymbolTable<'a>>>,
    },
}

impl<'a> SymbolTable<'a> {
    pub fn init_global() -> SymbolTable<'a> {
        SymbolTable::GLOBAL {
            globals: None,
            functions: None,
        }
    }
    pub fn add_function(&mut self, name: &mut String, nparams: &mut u32) {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.add_function(name, nparams),
                None => {
                    *f = Some(Box::new(SymbolTable::FUNCTION {
                        ntemps: 0,
                        nparams: *nparams,
                        name: name.clone(),
                        params: None,
                        body_vars: None,
                        next: None,
                    }));
                }
            },
            SymbolTable::FUNCTION {
                next: ref mut f, ..
            } => match f {
                Some(ref mut b) => return b.add_function(name, nparams),
                None => {
                    *f = Some(Box::new(SymbolTable::FUNCTION {
                        ntemps: 0,
                        nparams: *nparams,
                        name: name.clone(),
                        params: None,
                        body_vars: None,
                        next: None,
                    }));
                }
            },
        }
    }
    pub fn add_global(&mut self, id: &mut String) {
        match self {
            SymbolTable::GLOBAL {
                globals: ref mut g, ..
            } => match g {
                Some(ref mut b) => return b.add_global(id),
                None => *g = Some(Box::new(Entry::new_global(id))),
            },
            _ => std::process::exit(1),
        }
    }
    pub fn add_param(&mut self, id: &mut String) {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.add_param(id),
                None => {
                    eprintln!("cannot add param when no function has been defined.");
                    std::process::exit(1);
                }
            },
            SymbolTable::FUNCTION {
                next: ref mut f,
                params: ref mut p,
                ..
            } => match f {
                Some(ref mut b) => return b.add_param(id),
                None => match p {
                    Some(ref mut b) => b.add_param(id),
                    None => *p = Some(Box::new(Entry::new_param(id, 0))),
                },
            },
        }
    }
    pub fn add_body_var(&mut self, id: &mut String) {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.add_body_var(id),
                None => {
                    eprintln!("cannot add body_var when no function has been defined.");
                    std::process::exit(1);
                }
            },
            SymbolTable::FUNCTION {
                next: ref mut f,
                params: ref mut p,
                ..
            } => match f {
                Some(ref mut b) => return b.add_body_var(id),
                None => match p {
                    Some(ref mut b) => b.add_body_var(id),
                    None => *p = Some(Box::new(Entry::new_body_var(id, 0))),
                },
            },
        }
    }

    pub fn get_fn_body_bytes(&mut self, id: &mut String) -> u32 {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.get_fn_body_bytes(id),
                None => {
                    eprintln!("cannot get fn bytes when no function has been defined.");
                    std::process::exit(1);
                }
            },
            SymbolTable::FUNCTION {
                next: ref mut f,
                name,
                nparams: np,
                ..
            } => {
                if *id == *name {
                    return *np * 4;
                }
                match f {
                    Some(ref mut b) => return b.get_fn_body_bytes(id),
                    None => {
                        eprintln!("cannot get fn bytes when {} has not been defined.", id);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    pub fn body_var_param_def(&mut self, id: &mut String) -> bool {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.body_var_param_def(id),
                None => {
                    eprintln!("cannot get body_var or param when no function has been defined.");
                    std::process::exit(1);
                }
            },
            SymbolTable::FUNCTION {
                next: ref mut f,
                params: p,
                body_vars: b,
                ..
            } => match f {
                Some(ref mut b) => return b.body_var_param_def(id),
                None => {
                    match p {
                        Some(ref mut p1) => {
                            if p1.var_def(id) {
                                return true;
                            }
                        }
                        None => {}
                    }
                    match b {
                        Some(ref mut b1) => {
                            if b1.var_def(id) {
                                return true;
                            }
                        }
                        None => {}
                    }
                    return false;
                }
            },
        }
    }
    pub fn global_var_def(&mut self, id: &mut String) -> bool {
        match self {
            SymbolTable::GLOBAL {
                globals: ref mut g, ..
            } => match g {
                Some(ref mut b) => return b.var_def(id),
                None => return false,
            },
            _ => {
                eprintln!("cannot get global from a function table.");
                std::process::exit(1);
            }
        }
    }
    pub fn function_def(&mut self, id: &mut String, nparams: &mut u32) -> bool {
        match self {
            SymbolTable::GLOBAL {
                functions: ref mut f,
                ..
            } => match f {
                Some(ref mut b) => return b.function_def(id, nparams),
                None => return false,
            },
            SymbolTable::FUNCTION {
                next: ref mut f,
                name: n,
                nparams: p,
                ..
            } => {
                if *id == *n && *p == *nparams {
                    return true;
                }
                match f {
                    Some(ref mut b) => {
                        return b.function_def(id, nparams);
                    }
                    None => return false,
                }
            }
        }
    }
}
