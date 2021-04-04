enum Operations {
    Plus,
    Moins,
    Division
}

impl Operations{
    fn run(self, x: f32, y:f32) -> Result<f32, &'static str>{
        match self {
            Self::Plus => Ok(x+y),
            Self::Moins => Ok(x-y),

            Self::Division => match y {
                0_f32 => Err("Division par 0"),
                _ => Ok(x/y)
            }
        }
    }

    fn read_result(result : Result<f32, &'static str>){
        match result {
            Ok(v) => println!("result: {:?}", v),
            Err(e) => println!("error: {:?}", e),
        }
    }

    pub fn run_read_result(self, x: f32, y:f32){
        Operations::read_result(Operations::run(self, x, y));
    }
}

struct Op {
    x : f32,
    y: f32,
    operation : Operations
}


fn main() {
    println!("Hello, world!");
    let add = Op {x:1_f32,y:2_f32,operation:Operations::Plus};
    let minus = Op {x:3_f32,y:2_f32,operation:Operations::Moins};
    let div_zeo = Op {x:3_f32,y:0_f32,operation:Operations::Division};
    let div = Op {x:3_f32,y:2_f32,operation:Operations::Division};
    add.operation.run_read_result(add.x, add.y);
    minus.operation.run_read_result(minus.x, minus.y);
    div_zeo.operation.run_read_result(div_zeo.x, div_zeo.y);
    div.operation.run_read_result(div.x, div.y);
}