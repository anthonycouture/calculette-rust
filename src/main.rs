use std::borrow::Borrow;

enum Operateur {
    Plus,
    Moins,
    Division,
    Multiplication,
}

impl Operateur {
    fn run(&self, x: f32, y: f32) -> Result<f32, &'static str> {
        match self {
            Self::Plus => Ok(x + y),
            Self::Moins => Ok(x - y),
            Self::Division => match y {
                y if y == 0.00 => Err("Division par 0"),
                _ => Ok(x / y)
            },
            Self::Multiplication => Ok(x * y)
        }
    }

    fn read_result(result: Result<f32, &'static str>) -> f32 {
        match result {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        }
    }

    fn run_read_result(&self, x: f32, y: f32) -> f32 {
        Operateur::read_result(Operateur::run(self, x, y))
    }

    pub fn operateur_by_string(operateur: &'static str) -> Self {
        match operateur {
            "+" => Self::Plus,
            "-" => Self::Moins,
            "*" => Self::Multiplication,
            "/" => Self::Division,
            _ => panic!("{}", "Operateur inconnu")
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Self::Plus => "+",
            Self::Moins => "-",
            Self::Division => "/",
            Self::Multiplication => "*"
        }
    }
}

enum Expr {
    Number(f32),
    Token(Operateur),
}

enum Operation {
    Number(f32),
    NumberOperateurNumber(f32, Operateur, f32),
    NumberOperateurNode(f32, Operateur, Box<Operation>),
    NodeOperateurNode(Box<Operation>, Operateur, Box<Operation>),
}

impl Operation {
    pub fn string_to_operation(op: &'static str) -> Operation {
        fn verif_operation(mut op_vec: Vec<&'static str>, is_operateur: bool) -> Vec<Expr> {
            let i = op_vec[0];
            op_vec.remove(0);
            let mut vec_expr: Vec<Expr> = Vec::new();
            match is_operateur {
                false => {
                    vec_expr.push(Expr::Number(i.parse::<f32>().unwrap()));
                }
                true => {
                    vec_expr.push(Expr::Token(Operateur::operateur_by_string(i)));
                }
            }
            if op_vec.len() > 0 {
                vec_expr.append(&mut verif_operation(op_vec, !is_operateur));
            }
            vec_expr
        }
        fn is_token_faible(op_vec: Vec<&Expr>, index: i8) -> i8 {
            if op_vec.len() > 0 {
                let i = op_vec[0];
                let mut op_vec = op_vec.clone();
                op_vec.remove(0);
                match i {
                    Expr::Token(e) => {
                        match e {
                            Operateur::Moins | Operateur::Plus => index,
                            _ => {
                                let index = index + 1;
                                is_token_faible(op_vec, index)
                            }
                        }
                    }
                    _ => {
                        let index = index + 1;
                        is_token_faible(op_vec, index)
                    }
                }
            } else {
                -1
            }
        }
        let mut op_vec: Vec<&'static str> = op.split(' ').collect();
        if op_vec.len() % 2 == 0 {
            panic!("Operation incorrecte");
        }
        let expr_vec = verif_operation(op_vec.clone(), false);
        Operation::Number(0.00)
    }

    fn to_string(&self) -> String {
        match self {
            Self::Number(x) => x.to_string(),
            Self::NumberOperateurNumber(x, op, y) => {
                String::from(x.to_string() + op.to_string() + &*y.to_string())
            }
            Self::NumberOperateurNode(x, op, ref node) => {
                String::from(x.to_string() + op.to_string() + &*node.to_string())
            }
            Self::NodeOperateurNode(ref node1, op, ref node2) => {
                String::from(node1.to_string() + op.to_string() + &*node2.to_string())
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", Operation::Number(1.00).to_string());
    println!("{}", Operation::NumberOperateurNumber(1.00, Operateur::Plus, 2.00).to_string());
    println!("{}", Operation::NumberOperateurNode(1.00, Operateur::Plus, Box::from(Operation::NumberOperateurNumber(1.00, Operateur::Moins, 2.00))).to_string());
    println!("{}", Operation::NodeOperateurNode(Box::from(Operation::NumberOperateurNumber(2.00, Operateur::Multiplication, 2.00)), Operateur::Plus, Box::from(Operation::NumberOperateurNumber(1.00, Operateur::Division, 2.00))).to_string());
    Operation::string_to_operation("1 * 1 * 1 + 1");
}