#[derive(Debug)]
pub struct Program {
    pub data: Data,
    pub code: Code,
}

impl Program {
    pub fn to_uasm(&self) -> String {
        format!("{}\n{}", self.data.to_uasm(), self.code.to_uasm())
    }
}

#[derive(Debug)]
pub struct VarName(pub String);

impl VarName {
    pub fn new(name: &str) -> Self {
        VarName(name.to_string())
    }
}

#[derive(Debug)]
pub struct TypeName(pub String);

impl TypeName {
    pub fn new(name: &str) -> Self {
        TypeName(name.to_string())
    }
}

#[derive(Debug)]
pub struct Data {
    pub exports: Vec<VarName>,
    pub decls: Vec<VarDecl>,
}

impl Data {
    pub fn to_uasm(&self) -> String {
        format!(
            r#".data_start
{}
{}
.data_end
"#,
            string_of_exports(&self.exports),
            self.decls.iter().fold("".to_string(), |uasm, decl| {
                format!("{}\n{}", uasm, decl.to_uasm())
            })
        )
    }
}

fn string_of_exports(exports: &Vec<VarName>) -> String {
    format!(
        "    .export {}",
        if exports.is_empty() {
            "".to_string()
        } else if exports.len() == 1 {
            exports[0].0.clone()
        } else {
            exports.iter().fold("".to_string(), |uasm, export| {
                format!("{}, {}", uasm, export.0)
            })
        }
    )
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: VarName,
    pub type_: TypeName,
    pub init: Literal,
}

impl VarDecl {
    pub fn to_uasm(&self) -> String {
        format!(
            "    {}: {}, {}",
            self.name.0,
            self.type_.0,
            self.init.to_uasm()
        )
    }
}

#[derive(Debug)]
pub enum Literal {
    Null,
    This,
}

impl Literal {
    pub fn to_uasm(&self) -> String {
        match self {
            Literal::Null => "null".to_string(),
            Literal::This => "this".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Var(VarName),
}

#[derive(Debug)]
pub struct Code {
    pub exports: Vec<VarName>,
    pub insts: Vec<Inst>,
}

impl Code {
    pub fn to_uasm(&self) -> String {
        format!(
            r#".code_start
{}
{}
.code_end
"#,
            string_of_exports(&self.exports),
            self.insts.iter().fold("".to_string(), |uasm, inst| {
                format!("{}\n{}", uasm, inst.to_uasm())
            })
        )
    }
}

#[derive(Debug)]
pub enum Addr {
    Var(VarName),
    Immediate(i32),
}

impl Addr {
    pub fn to_uasm(&self) -> String {
        use Addr::*;
        match self {
            Var(name) => name.0.clone(),
            Immediate(n) => format!("0x{:X}", n),
        }
    }
}

#[derive(Debug)]
pub enum Inst {
    Nop,
    Push(Addr),
    Pop,
    JumpIfFalse(Addr),
    Jump(Addr),
    Extern(String),
    JumpIndirect(Addr),
    Copy,
    Label(VarName),
}

impl Inst {
    pub fn to_uasm(&self) -> String {
        use Inst::*;
        match self {
            Nop => "        NOP".to_string(),
            Push(addr) => format!("        PUSH, {}", addr.to_uasm()),
            Pop => "        POP".to_string(),
            JumpIfFalse(addr) => format!("        JUMP_IF_FALSE, {}", addr.to_uasm()),
            Jump(addr) => format!("        JUMP, {}", addr.to_uasm()),
            Extern(fun_name) => format!("        EXTERN, {}", fun_name),
            JumpIndirect(addr) => format!("        JUMP_INDIRECT, {}", addr.to_uasm()),
            Copy => "        COPY".to_string(),
            Label(label_name) => format!("    {}:", label_name.0),
        }
    }
}
