use std::rc::Rc;

use loc::Loc;

pub trait Node {
    fn loc(&self) -> &Loc;
}

macro_rules! node {
    (struct $nam:ident { $($field:ident : $ty:ty ,)* }) => {
        #[derive(Debug)]
        pub struct $nam {
            pub loc: Loc,
            $(pub $field : $ty ,)*
        }

        impl Node for $nam {
            fn loc(&self) -> &Loc {
                &self.loc
            }
        }
    };

    (enum $nam:ident { $($case:ident { $($field:ident : $ty:ty ,)* } ,)* }) => {
        #[derive(Debug)]
        pub enum $nam {
            $($case {
                loc: Loc,
                $($field : $ty ,)*
            } ,)*
        }

        impl Node for $nam {
            fn loc(&self) -> &Loc {
                match *self {
                    $($nam::$case { ref loc, .. } => loc,)*
                }
            }
        }
    }
}

node!(struct SourceFile {
    package: PackageClause,
});

node!(struct PackageClause {
    name: Id,
});

node!(struct ExpressionList {
    exprs: Vec<Rc<Expression>>,
});

node!(struct Block {
    statements: Vec<Rc<Statement>>,
});

node!(struct ConstDecl {
    specs: Vec<Rc<ConstSpec>>,
});

node!(struct Id {
    ident: String,
});

node!(struct ConstSpec {
    idents: Vec<Id>,
    exprs: ExpressionList,
});

node!(struct TypeDecl {});

node!(struct VarDecl {});

node!(enum Declaration {
    Const { decl: ConstDecl, },
    Type { decl: TypeDecl, },
    Var { decl: VarDecl, },
});

node!(struct FunctionDecl {});

node!(struct MethodDecl {});

node!(enum TopLevelDecl {
    Decl { decl: Declaration, },
    Func { decl: FunctionDecl, },
    Method { decl: MethodDecl, },
});

node!(struct LabeledStatement {
    label: Id,
    statement: Rc<Statement>,
});

node!(struct IncStmt {
    lhs: Rc<Expression>,
});

node!(struct DecStmt {
    lhs: Rc<Expression>,
});

#[derive(Debug)]
pub enum AssignOp {
    Add,
    Mul,
}

node!(enum SimpleStmt {
    Expr {
        expr: Rc<Expression>,
    },
    Send {
        chan: Rc<Expression>,
        value: Rc<Expression>,
    },
    Inc {
        lhs: Rc<Expression>,
    },
    Dec {
        lhs: Rc<Expression>,
    },
    Assign {
        lhs: ExpressionList,
        op: Option<AssignOp>,
        rhs: ExpressionList,
    },
    ShortVarDecl {
        lhs: ExpressionList,
        op: Option<AssignOp>,
        rhs: ExpressionList,
    },
});

node!(struct IfStmt {
    stmt: Option<SimpleStmt>,
    cond: Rc<Expression>,
    then: Block,
    // else_: Option<Rc<ElseStmt>>,
});

node!(enum Else {
    If { if_: IfStmt, },
    Else { else_: Block, },
});

node!(enum Statement {
    Decl { decl: Declaration, },
    Labeled {
        label: Id,
        stmt: Rc<Statement>,
    },
    Simple { simple: SimpleStmt, },
    If { if_: IfStmt, },
});

node!(enum Expression {
    //
});
