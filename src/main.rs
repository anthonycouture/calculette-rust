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
    let addition = Operation { x: 1_f32, operation: Some((Operateur::Plus, Box::new(Operation { x: 1_f32, operation: None }))) };
    let soustraction = Operation { x: 3_f32, operation: Some((Operateur::Moins, Box::new(Operation { x: 2_f32, operation: None }))) };
    let division = Operation { x: 3_f32, operation: Some((Operateur::Division, Box::new(Operation { x: 2_f32, operation: None }))) };
    let multiplication = Operation { x: 3_f32, operation: Some((Operateur::Multiplication, Box::new(Operation { x: 2_f32, operation: None }))) };
    let division_and_addition = Operation { x: 1_f32, operation: Some((Operateur::Plus, Box::new(Operation { x: 5_f32, operation: Some((Operateur::Division, Box::new(Operation { x: 2_f32, operation: None }))) }))) };
    println!("Résultat de 1+1 = {}", &addition.read_op());
    println!("Résultat de 3-2 = {}", &soustraction.read_op());
    println!("Résultat de 3/2 = {}", &division.read_op());
    println!("Résultat de 3*2 = {}", &multiplication.read_op());
    println!("Résultat de 1+5/2 = {}", &division_and_addition.read_op());
    // Decommente les lignes ci-dessous le programme plante à cause d'une divion par 0
    /*let division_zero = Operation { x: 3_f32, operation: Some((Operateur::Division, Box::new(Operation { x: 0_f32, operation: None }))) };
    println!("Résultat de la divison 3/0 = {}", &division_zero.read_op());*/
}