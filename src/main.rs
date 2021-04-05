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
    pub fn new(nombre:f32, operation_suite: Option<(Operateur, Operation)>) -> Self{
        match operation_suite {
            None => Self{ x: nombre, operation : None},
            Some((operateur, suite)) => Self{x: nombre, operation: Some((operateur, Box::new(suite)))}
        }
    }

    fn calcule_operation(&self) -> f32 {
        match self {
            Self { x: a, operation: None } => *a,
            Self { x: a, operation: Some((op, ref t)) } => op.run_read_result(*a, t.calcule_operation())
        }
    }
}


fn main() {
    println!("Hello, world!");
    let addition = Operation::new(1_f32, Some((Operateur::Plus, Operation { x: 1_f32, operation: None })));
    let soustraction = Operation::new(3_f32,Some((Operateur::Moins, Operation { x: 2_f32, operation: None })));
    let division = Operation::new(3_f32, Some((Operateur::Division,Operation { x: 2_f32, operation: None })));
    let multiplication = Operation::new(3_f32, Some((Operateur::Multiplication, Operation { x: 2_f32, operation: None })));
    let division_and_addition = Operation::new(1_f32, Some((Operateur::Plus, Operation::new(5_f32, Some((Operateur::Division, Operation { x: 2_f32, operation: None })))))) ;
    println!("Résultat de 1+1 = {}", &addition.calcule_operation());
    println!("Résultat de 3-2 = {}", &soustraction.calcule_operation());
    println!("Résultat de 3/2 = {}", &division.calcule_operation());
    println!("Résultat de 3*2 = {}", &multiplication.calcule_operation());
    println!("Résultat de 1+5/2 = {}", &division_and_addition.calcule_operation());
    // Decommente les lignes ci-dessous le programme plante à cause d'une divion par 0
    /*let division_zero = Operation::new(3_f32, Some((Operateur::Division, Operation { x: 0_f32, operation: None })));
    println!("Résultat de la divison 3/0 = {}", &division_zero.calcule_operation());*/
}