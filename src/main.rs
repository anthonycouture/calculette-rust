enum Operateur {
    Plus,
    Moins,
    Division,
}

impl Operateur {

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
        Operateur::read_result(Operateur::run(self, x, y))
    }
}

struct Operation {
    x: f32,
    operation: Option<(Operateur, Box<Operation>)>,
}

impl Operation {
    fn read_op(&self) -> f32 {
        match self {
            Self { x: a, operation: None } => *a,
            Self { x: a, operation: Some((op, ref t)) } => op.run_read_result(*a, t.read_op())
        }
    }
}


fn main() {
    println!("Hello, world!");
    let add = Operation { x: 1_f32, operation: Some((Operateur::Plus, Box::new(Operation { x: 1_f32, operation: None }))) };
    let minus = Operation {x:3_f32,operation:Some((Operateur::Moins, Box::new(Operation { x: 2_f32, operation: None})))};
    let div_zero = Operation {x:3_f32,operation:Some((Operateur::Division, Box::new(Operation { x: 0_f32, operation: None})))};
    let div = Operation {x:3_f32,operation:Some((Operateur::Division, Box::new(Operation { x: 2_f32, operation: None})))};
    let div_and_add = Operation {x:1_f32,operation:Some((Operateur::Plus, Box::new(Operation { x: 5_f32, operation: Some((Operateur::Division, Box::new(Operation { x: 2_f32, operation: None }))) })))};
    println!("{}", &add.read_op());
    println!("{}", &minus.read_op());
    println!("{}", &div.read_op());
    println!("{}", &div_and_add.read_op());
    // Decommente ligne ci-dessous le programme plante à cause d'une divion par 0
    //println!("{}", &div_zero.read_op());

}