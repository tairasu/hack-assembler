fn main() {

    let file = get_file();
    let mut output: String = String::new();

    //check if A instruction
    let lines = file.lines();
    for line in lines {
        if line.starts_with("@") {
            let line = line.trim_start_matches("@");
            let line = line.parse::<i32>().unwrap();
            let binary = format!("0{:015b}", line);

            output.push_str(&binary);
            output.push('\n');
        } else {
            let mut binary:String = String::from("111");
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
                _ => println!("Error")
            }
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
    //write to file with the .hack extension
    std::fs::write("/Users/marco/VSCode/rust/hack-assembler/6/add/Add.hack", output).unwrap();

}


fn get_file() -> String {

        //READ FILE AND REMOVE COMMENTS

        let path = "/Users/marco/VSCode/rust/hack-assembler/6/add/Add.asm";
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
        output
}

