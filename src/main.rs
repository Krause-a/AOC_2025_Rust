fn main() {
    // 1. Read in the inputs
    // Inputs look like
    // 5 1 0 --debug
    // 2 2 --debug
    // 2 1 1
    // 4 2
    let mut args = std::env::args();
    let _called_exec = args.next();
    let day : usize = args.next().expect("Missing DAY argument").parse().expect("DAY argument must be a number");
    let part : usize = args.next().expect("Missing PART argument").parse().expect("PART argument must be a number");
    let mut debug_enabled = false;
    let mut use_test_set = true;
    if let Some(third_arg) = args.next() {
        if third_arg == "--debug" {
            debug_enabled = true;
        }
        else if third_arg == "0" {
            use_test_set = false;
        }
        else {
            eprintln!("Unused third argument!");
        }
    }
    if let Some(fourth_arg) = args.next() {
        if fourth_arg == "--debug" {
            debug_enabled = true;
        }
        else {
            eprintln!("Unused fourth argument!");
        }
    }

    // 2. Select the correct data file
    let mut data_file_name = format!("{day:02}_{part}");
    if use_test_set {
        data_file_name += "_test";
    }
    std::fs::

    // Call the correct day
    println!("Hello, world!");


    // Ohh! Idea!
    // Return an object that has multiple different generator functions
    // It wraps the file handle for the correct data and exposes multiple different ways to access
    // the data. Stuff like
    // fn get_lines()
    // fn get_chars()
    // fn get_string()
    //
    // This can help optimize certain ways of processing the data!
}
