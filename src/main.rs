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

enum Operation {
    Number(f32),
    NumberExprNumber(f32, Operateur, f32),
    NumberExprNode(f32, Operateur, Box<Operation>),
    NodeExprNode(Box<Operation>, Operateur, Box<Operation>)
}

impl Operation {

    fn to_string(&self) -> String {
        match self {
            Self::Number(x ) => x.to_string(),
            Self::NumberExprNumber(x, op, y) => {
                String::from(x.to_string() + op.to_string() + &*y.to_string())
            },
            Self::NumberExprNode(x, op, ref node) => {
                String::from(x.to_string()+op.to_string()+&*node.to_string())
            },
            Self::NodeExprNode(ref node1, op, ref node2) => {
                String::from(node1.to_string()+op.to_string()+&*node2.to_string())
            },
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}",Operation::Number(1.00).to_string());
    println!("{}",Operation::NumberExprNumber(1.00, Operateur::Plus, 2.00).to_string());
    println!("{}",Operation::NumberExprNode(1.00, Operateur::Plus, Box::from(Operation::NumberExprNumber(1.00, Operateur::Moins, 2.00))).to_string());
    println!("{}",Operation::NodeExprNode(Box::from(Operation::NumberExprNumber(2.00, Operateur::Multiplication, 2.00)), Operateur::Plus, Box::from(Operation::NumberExprNumber(1.00, Operateur::Division, 2.00))).to_string());
}