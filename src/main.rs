use std::collections::HashMap;

fn main() {

    //CLI ARGUMENTS
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./rust-assembler <filepath>");
        std::process::exit(1);
    }

    //GET FILE
    let file = get_file(&args[1]);
    let output = parse(&file);
    std::fs::write(&args[1].replace(".asm", ".hack"), output).unwrap();

}


fn get_file(filepath: &str) -> String {

        //READ FILE AND REMOVE COMMENTS

        let file = std::fs::read_to_string(filepath).unwrap();
        let lines = file.lines();
        let mut output = String::new();
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("//") {
                continue;
            }
            let line = line.split("//").next().unwrap().trim();
            output.push_str(line);
            output.push('\n');
        }
        output
}

fn parse(file: &str) -> String {
    
    let mut output: String = String::new();
    let mut custom_symbols: HashMap<String, i16> = HashMap::new();

    //PREDEFINED SYMBOLS
    let predefined_symbols: [(&str, i32); 23] = [
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
        ("R0", 0),
        ("R1", 1),
        ("R2", 2),
        ("R3", 3),
        ("R4", 4),
        ("R5", 5),
        ("R6", 6),
        ("R7", 7),
        ("R8", 8),
        ("R9", 9),
        ("R10", 10),
        ("R11", 11),
        ("R12", 12),
        ("R13", 13),
        ("R14", 14),
        ("R15", 15),
        ("SCREEN", 16384),
        ("KBD", 24576),
    ];
    
    //FIRST PASS
    //check if L instruction
    let lines = file.lines();
    let mut line_number: i16 = 0;
    for line in lines {
        if line.starts_with("(") {
            let line = line.trim_start_matches("(").trim_end_matches(")");
            custom_symbols.insert(line.to_string(), line_number);
        } else {
            line_number += 1;
        }
    }

    //remove all L instructions and save in lines
    let lines = file.lines();
    let clean_lines = lines.filter(|line| !line.starts_with("(")).collect::<Vec<&str>>();

    //SECOND PASS
    //check if A instruction
    for line in clean_lines {
        if line.starts_with("@") {
            let line = line.trim_start_matches("@");

            if !line.parse::<i16>().is_err() {
                //USING NUMBERS
                let line = line.parse::<i16>().unwrap();
                let binary = format!("0{:015b}", line as i16);
                output.push_str(&binary);
                output.push('\n');
            } else {
                //ADDING CUSTOM SYMBOLS
                if !predefined_symbols.iter().any(|(symbol, _)| symbol == &line) && !custom_symbols.contains_key(line) {
                    let count = custom_symbols.len() as i16;
                    custom_symbols.insert(line.to_string(), 16 + count);
                }

                //USING PREDEFINED SYMBOLS
                if predefined_symbols.iter().any(|(symbol, _)| symbol.to_string() == format!("{}", &line)) {
                    let predefined_symbol = predefined_symbols.iter().find(|(symbol, _)| symbol.to_string() == format!("{}", &line)).unwrap();
                    let binary = format!("0{:015b}", predefined_symbol.1);
                    output.push_str(&binary);
                    output.push('\n');
                }
                else if custom_symbols.iter().any(|(symbol, _)| symbol.to_string() == format!("{}", &line)) {
                //USING CUSTOM SYMBOLS
                    let custom_symbol = custom_symbols.iter().find(|(symbol, _)| symbol.to_string() == format!("{}", &line)).unwrap();
                    let binary = format!("0{:015b}", custom_symbol.1);
                    output.push_str(&binary);
                    output.push('\n');
                }
            }
            
        } else {

            //C instruction
            let mut binary:String = String::from("111");

        // Determine the `comp` part of the instruction
        let comp = if line.contains('=') {
            // extract the part after '='
            line.split('=').nth(1).unwrap_or("")
        } else if line.contains(';') {
            // extract the part before ';'
            line.split(';').nth(0).unwrap_or("")
        } else {
            // Default case
            ""
        };

        // Use a single `match` statement for the `comp` part
        match comp {
            "0" => binary.push_str("0101010"),
            "1" => binary.push_str("0111111"),
            "-1" => binary.push_str("0111010"),
            "D" => binary.push_str("0001100"),
            "A" => binary.push_str("0110000"),
            "M" => binary.push_str("1110000"),
            "!D" => binary.push_str("0001101"),
            "!A" => binary.push_str("0110001"),
            "!M" => binary.push_str("1110001"),
            "-D" => binary.push_str("0001111"),
            "-A" => binary.push_str("0110011"),
            "-M" => binary.push_str("1110011"),
            "D+1" => binary.push_str("0011111"),
            "A+1" => binary.push_str("0110111"),
            "M+1" => binary.push_str("1110111"),
            "D-1" => binary.push_str("0001110"),
            "A-1" => binary.push_str("0110010"),
            "M-1" => binary.push_str("1110010"),
            "D+A" => binary.push_str("0000010"),
            "D+M" => binary.push_str("1000010"),
            "D-A" => binary.push_str("0010011"),
            "D-M" => binary.push_str("1010011"),
            "A-D" => binary.push_str("0000111"),
            "M-D" => binary.push_str("1000111"),
            "D&A" => binary.push_str("0000000"),
            "D&M" => binary.push_str("1000000"),
            "D|A" => binary.push_str("0010101"),
            "D|M" => binary.push_str("1010101"),
            _ => (),
        }

            //dest
            match line.split("=").nth(0) {
                None => binary.push_str("000"),
                Some("M") => binary.push_str("001"),
                Some("D") => binary.push_str("010"),
                Some("MD") => binary.push_str("011"),
                Some("A") => binary.push_str("100"),
                Some("AM") => binary.push_str("101"),
                Some("AD") => binary.push_str("110"),
                Some("AMD") => binary.push_str("111"),
                _ => binary.push_str("000")
            }

            //jump

            match line.split(";").nth(1) {
                None => binary.push_str("000"),
                Some("JGT") => binary.push_str("001"),
                Some("JEQ") => binary.push_str("010"),
                Some("JGE") => binary.push_str("011"),
                Some("JLT") => binary.push_str("100"),
                Some("JNE") => binary.push_str("101"),
                Some("JLE") => binary.push_str("110"),
                Some("JMP") => binary.push_str("111"),
                _ => binary.push_str("000")
            }


            output.push_str(&binary);
            output.push('\n');
        }
    }
    //trim last \n
    output.pop();
    println!("{}", output);
    output
}