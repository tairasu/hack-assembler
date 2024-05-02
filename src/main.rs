use std::collections::HashMap;

fn main() {

    let file = get_file();
    let mut output: String = String::new();
    let mut custom_symbols: HashMap<String, i16> = HashMap::new();
    let mut jump_symbols: HashMap<String, i16> = HashMap::new();

    //get all symbols in brackets and add them to the custom symbols hashmap
    let lines = file.lines();
    for line in lines {
        if line.starts_with("(") {
            let line = line.trim_start_matches("(").trim_end_matches(")");
            custom_symbols.insert(line.to_string(), 0);
        }
    }



    //PREDEFINED SYMBOLS
    let predefined_symbols = [
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

    //check if A instruction
    let lines = file.lines();
    for line in lines {
        if line.starts_with("@") {
            let line = line.trim_start_matches("@");

            if !line.parse::<f64>().is_err() {
                let line = line.parse::<f64>().unwrap();
                //USING NUMBERS
                let binary = format!("0{:015b}", line as i16);
                output.push_str(&binary);
                output.push('\n');
            } else {
                //ADDING CUSTOM SYMBOLS
                if !predefined_symbols.iter().any(|(symbol, _)| symbol.to_string() == format!("{}", &line)) && line.is_ascii() {
                    //count how many custom symbols there are
                    let count = custom_symbols.len() as i16;
                    custom_symbols.insert(line.to_string(), 16+count);
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

            //comp
            match line.split("=").nth(1) {
                Some("0") => binary.push_str("0101010"),
                Some("1") => binary.push_str("0111111"),
                Some("-1") => binary.push_str("0111010"),
                Some("D") => binary.push_str("0001100"),
                Some("A") => binary.push_str("0110000"),
                Some("M") => binary.push_str("1110000"),
                Some("!D") => binary.push_str("0001101"),
                Some("!A") => binary.push_str("0110001"),
                Some("!M") => binary.push_str("1110001"),
                Some("-D") => binary.push_str("0001111"),
                Some("-A") => binary.push_str("0110011"),
                Some("-M") => binary.push_str("1110011"),
                Some("D+1") => binary.push_str("0011111"),
                Some("A+1") => binary.push_str("0110111"),
                Some("M+1") => binary.push_str("1110111"),
                Some("D-1") => binary.push_str("0001110"),
                Some("A-1") => binary.push_str("0110010"),
                Some("M-1") => binary.push_str("1110010"),
                Some("D+A") => binary.push_str("0000010"),
                Some("D+M") => binary.push_str("1000010"),
                Some("D-A") => binary.push_str("0010011"),
                Some("D-M") => binary.push_str("1010011"),
                Some("A-D") => binary.push_str("0000111"),
                Some("M-D") => binary.push_str("1000111"),
                Some("D&A") => binary.push_str("0000000"),
                Some("D&M") => binary.push_str("1000000"),
                Some("D|A") => binary.push_str("0010101"),
                Some("D|M") => binary.push_str("1010101"),
                _ => println!("BRERROR")
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

    println!("{:?}", predefined_symbols);
    println!("{:?}", custom_symbols);

    //validate output that each line is 16 bits
    // let lines = output.lines();
    // for line in lines {
    //     if line.len() != 16 {
    //         panic!("Error: Syntax error in input file. Each line should be 16 bits long.");
    //     }
    // }

    //write to file with the .hack extension
    std::fs::write("/Users/marco/VSCode/nand2tetris/hack-assembler/6/max/Max.hack", output).unwrap();

}


fn get_file() -> String {

        //READ FILE AND REMOVE COMMENTS

        let path = "/Users/marco/VSCode/nand2tetris/hack-assembler/6/max/Max.asm";
        let file = std::fs::read_to_string(path).unwrap();
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
        println!("{}", output);
        output
}

