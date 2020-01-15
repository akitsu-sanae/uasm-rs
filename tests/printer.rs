extern crate uasm_rs;
use uasm_rs::*;

#[test]
fn plain() {
    let program = Program {
        data: Data {
            exports: vec![],
            decls: vec![],
        },
        code: Code {
            exports: vec![],
            insts: vec![],
        },
    };
    assert_eq!(
        program.to_uasm(),
        r#".data_start
    .export 

.data_end

.code_start
    .export 

.code_end
"#
        .to_string()
    )
}

#[test]
fn tiny() {
    let program = Program {
        data: Data {
            exports: vec![VarName::new("Target")],
            decls: vec![VarDecl {
                name: VarName::new("Target"),
                type_: TypeName::new("%UnityEngineTransform"),
                init: Literal::This,
            }],
        },
        code: Code {
            exports: vec![VarName::new("_update")],
            insts: vec![
                Inst::Label(VarName::new("_update")),
                Inst::Jump(Addr::Immediate(0xffffff)),
            ],
        },
    };
    assert_eq!(
        program.to_uasm(),
        r#".data_start
    .export Target

    Target: %UnityEngineTransform, this
.data_end

.code_start
    .export _update

    _update:
        JUMP, 0xFFFFFF
.code_end
"#
        .to_string()
    )
}
