#[derive(Debug)]
enum Operateur {
    Plus,
    Moins,
    Division,
    Multiplication,
}

impl Operateur {
    fn run(&self, x: f32, y: f32) -> Result<f32, String> {
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
            _ => Err(String::from("Operateur inconnu"))
        }
    }
}

#[derive(Debug)]
enum Expr {
    Number(f32),
    Token(Operateur),
}


#[derive(Debug)]
struct Operation {
    operation: Vec<Expr>
}

impl Operation {
    pub fn string_to_operation(op: &'static str) -> Operation {
        fn vector_to_operation(op_vector: Vec<&'static str>, index: i8) -> Vec<Expr> {
            let t = match index {
                index if index % 2 == 0 => Expr::Number(op_vector[index as usize].parse().unwrap()),
                index if index % 2 != 0 => {
                    let operateur = Operateur::operateur_by_string(op_vector[index as usize]);
                    match operateur {
                        Ok(e) => Expr::Token(e),
                        Err(e) => panic!("{}", e)
                    }
                }
                _ => panic!("Error avec l'index")
            };
            let mut v = vec![t];
            let index = index + 1;
            if op_vector.len() > index as usize {
                v.append(&mut vector_to_operation(op_vector, index));
            }
            v
        }
        let operation: Vec<&str> = op.split(' ').collect();
        if operation.len() % 2 == 0 {
            panic!("Operation incorrect");
        }
        Operation { operation: vector_to_operation(operation, 0) }
    }

    fn evaluate_operation(self) -> f32 {
        fn evaluate_prio(mut vector_op: Vec<Expr>, index: i8) -> f32 {
            let expr = &vector_op[index as usize];
            match expr {
                Expr::Number(_) => {
                    let index = index + 1;
                    if vector_op.len() > index as usize {
                        evaluate_prio(vector_op, index)
                    } else {
                        evaluate_not_prio(vector_op, 0)
                    }
                }
                Expr::Token(operateur) => {
                    match operateur {
                        Operateur::Moins | Operateur::Plus => {
                            let index = index + 1;
                            evaluate_prio(vector_op, index)
                        }
                        Operateur::Multiplication | Operateur::Division => {
                            let x = match vector_op[(index-1) as usize] {
                                Expr::Number(e) => e,
                                _ => panic!("Ce n'est pas un nombre")
                            };
                            let y = match vector_op[(index+1) as usize] {
                                Expr::Number(e) => e,
                                _ => panic!("Ce n'est pas un nombre")
                            };
                            let r = operateur.run(x, y);
                            match r {
                                Err(e) => panic!("{}", e),
                                Ok(i) => {
                                    // On supprime l'opérateur
                                    vector_op.remove(index as usize);
                                    // Comme le nombre suivant l'opérateur à pris sa place on le supprime aussi
                                    vector_op.remove(index as usize);
                                    let index = index - 1;
                                    // On remplace le nombre se trouvant devant l'opérateur par le résultat
                                    vector_op[index as usize] = Expr::Number(i);
                                    if vector_op.len() < index as usize {
                                        evaluate_not_prio(vector_op, 0)
                                    } else {
                                        evaluate_prio(vector_op, index)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        fn evaluate_not_prio(mut vector_op: Vec<Expr>, index: i8) -> f32 {
            let expr = &vector_op[index as usize];
            match expr {
                Expr::Number(_) => {
                    let index = index + 1;
                    evaluate_not_prio(vector_op, index)
                }
                Expr::Token(operateur) => {
                    match operateur {
                        Operateur::Multiplication | Operateur::Division => panic!("Les opérateurs prioritaires sont a faire avant"),
                        Operateur::Moins | Operateur::Plus => {
                            let x = match vector_op[(index-1) as usize] {
                                Expr::Number(e) => e,
                                _ => panic!("Ce n'est pas un nombre")
                            };
                            let y = match vector_op[(index+1) as usize] {
                                Expr::Number(e) => e,
                                _ => panic!("Ce n'est pas un nombre")
                            };
                            let r = operateur.run(x, y);
                            match r {
                                Err(e) => panic!("{}", e),
                                Ok(i) => {
                                    // On supprime l'opérateur
                                    vector_op.remove(index as usize);
                                    // Comme le nombre suivant l'opérateur à pris sa place on le supprime aussi
                                    vector_op.remove(index as usize);
                                    let index = index - 1;
                                    // On remplace le nombre se trouvant devant l'opérateur par le résultat
                                    vector_op[index as usize] = Expr::Number(i);
                                    if vector_op.len() == 1 {
                                        i
                                    } else {
                                        evaluate_not_prio(vector_op, index)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let op = self.operation;
        evaluate_prio(op, 0)
    }
}

fn main() {
    println!("Hello, world!");
    let t = "8 + 25 * 2 / 50 + 10";
    let r = Operation::string_to_operation(t);
    println!("{} = {:?}", t, r.operation);
    println!("{} = {}", t, r.evaluate_operation());
}