pub enum Stmt {
    FuncDef {name: String, args: Vec<Stmt>, body: Vec<Stmt>},
    FuncVoid {name: String, args: Vec<Stmt>},
    FuncCall {name: String, args: Vec<Stmt>, body: Vec<Stmt>},
    Return {value: Vec<Stmt>},
    Print{value: Vec<Stmt>},
    Input{variable: String},

    Addition{a: Vec<Stmt>, b: Vec<Stmt>},
    Subtraction {a: Vec<Stmt>, b: Vec<Stmt>},
    Multiplication {a: Vec<Stmt>, b: Vec<Stmt>},
    Division {a: Vec<Stmt>, b: Vec<Stmt>},
    Exponentiation {a: Vec<Stmt>, b: Vec<Stmt>},
    DivisionWithRemainder {a: Vec<Stmt>, b: Vec<Stmt>},
    RemainderOfDivision {a: Vec<Stmt>, b: Vec<Stmt>},

    Variable{name: String},
    Integer{value: i64},
    Float{value: f64},
    String{value: String},
    Boolean{value: bool}
}