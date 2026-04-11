#[derive(Debug, Clone)]
pub enum Stmt {
    FuncDef {name: String, args: Vec<String>, body: Vec<Stmt>},
    FuncVoid {name: String, args: Vec<Stmt>},
    FuncCall {name: String, args: Vec<Stmt>},
    Return {value: Box<Stmt>},
    Print{value: Vec<Stmt>},
    Input{variable: String},

    Addition{a: Box<Stmt>, b: Box<Stmt>},
    Subtraction {a: Box<Stmt>, b: Box<Stmt>},
    Multiplication {a: Box<Stmt>, b: Box<Stmt>},
    Division {a: Box<Stmt>, b: Box<Stmt>},
    Exponentiation {a: Box<Stmt>, b: Box<Stmt>},
    DivisionWithRemainder {a: Box<Stmt>, b: Box<Stmt>},
    RemainderOfDivision {a: Box<Stmt>, b: Box<Stmt>},

    Variable{name: String},
    Integer{value: i64},
    Float{value: f64},
    String{value: String},
    Boolean{value: bool}
}