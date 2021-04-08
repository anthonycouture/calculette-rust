#[derive(Debug)]
pub enum Operateur {
    Plus,
    Moins,
    Division,
    Multiplication,
}

impl Operateur {
    pub fn run(&self, x: f32, y: f32) -> Result<f32, String> {
        match self {
            Self::Plus => Ok(x + y),
            Self::Moins => Ok(x - y),
            Self::Division => match y {
                y if y == 0.00 => Err(String::from("Division par 0")),
                _ => Ok(x / y)
            },
            Self::Multiplication => Ok(x * y)
        }
    }

    pub fn operateur_by_string(operateur: &'static str) -> Result<Self, String> {
        match operateur {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Moins),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            _ => Err({
                let mut error = String::from("Operateur ");
                error.push_str(operateur);
                error.push_str(" inconnu");
                error
            })
        }
    }
}
