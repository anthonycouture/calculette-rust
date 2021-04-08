mod operation;
fn main() {
    println!("Hello, world!");
    let list_operation = vec!["3 * 4 / 2 + 3", "2 / 5 * 7 + 6.8",
                              "6 * 6 * 8 - 7 / 2 + 8 / 9 - 4", "0 / 1 - 7 + 78 / 7 / 7",
                              "7 - 8 * 1 / 1 + 1", "1", "1 * 1", "1 + 1"];
    let list_response = vec![9.00, 9.6, 281.3889, -5.408163265306, 0.00, 1.00, 1.00, 2.00];
    let mut i = 0;
    for t in list_operation {
        let r = operation::Operation::string_to_operation(t);
        //println!("{} = {:?}", t, r.operation);
        let response = r.evaluate_operation();
        println!("{} = {} {}", t, response, response== list_response[i]);
        i = i +1;
    }
}