enum Operations {
    Plus,
    Moins,
    Division,
}

impl Operations {

    fn run(&self, x: f32, y: f32) -> Result<f32, &'static str> {
        match self {
            Self::Plus => Ok(x + y),
            Self::Moins => Ok(x - y),

            Self::Division => match y {
                y if y == 0.00 => Err("Division par 0"),
                _ => Ok(x / y)
            }
        }
    }

    fn read_result(result: Result<f32, &'static str>) -> f32 {
        match result {
            Ok(v) => v,
            Err(e) => panic!("{}",e),
        }
    }

    fn run_read_result(&self, x: f32, y: f32) -> f32 {
        Operations::read_result(Operations::run(self, x, y))
    }
}

struct Op {
    x: f32,
    operation: Option<(Operations, Box<Op>)>,
}

impl Op {
    fn read_op(&self) -> f32 {
        match self {
            Self { x: a, operation: None } => *a,
            Self { x: a, operation: Some((op, ref t)) } => op.run_read_result(*a, t.read_op())
        }
    }
}


fn main() {
    println!("Hello, world!");
    let add = Op { x: 1_f32, operation: Some((Operations::Plus, Box::new(Op { x: 1_f32, operation: None }))) };
    let minus = Op {x:3_f32,operation:Some((Operations::Moins, Box::new(Op{ x: 2_f32, operation: None})))};
    let div_zero = Op {x:3_f32,operation:Some((Operations::Division, Box::new(Op{ x: 0_f32, operation: None})))};
    let div = Op {x:3_f32,operation:Some((Operations::Division, Box::new(Op{ x: 2_f32, operation: None})))};
    println!("{}", &add.read_op());
    println!("{}", &minus.read_op());
    println!("{}", &div.read_op());
    // Decommente ligne ci-dessous le programme plante à cause d'une divion par 0
    //println!("{}", &div_zero.read_op());

}