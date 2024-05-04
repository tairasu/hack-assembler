use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    // CLI ARGUMENTS
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./rust-assembler <filepath>");
        std::process::exit(1);
    }
    // GET FILE
    let file_content = match get_file(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    };
    let output = parse(&file_content);

    // Handle potential error during file writing
    if let Err(e) = std::fs::write(&args[1].replace(".asm", ".hack"), &output) {
        eprintln!("Failed to write output file: {}", e);
        std::process::exit(1);
    }
}

fn get_file(filepath: &str) -> io::Result<String> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut output = String::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(comment_start) = line.find("//") {
            // If there's a comment in the middle of the line, only keep the part before it
            let before_comment = line[..comment_start].trim();
            if !before_comment.is_empty() {
                output.push_str(before_comment);
                output.push('\n');
            }
        } else {
            // No comment found, push the whole line
            output.push_str(line);
            output.push('\n');
        }
    }
    Ok(output)
}

fn parse(file: &str) -> String {
    let mut output: String = String::new();
    let mut custom_symbols: HashMap<&str, i16> = HashMap::new();

    //PREDEFINED SYMBOLS
    let predefined_symbols: [(&str, i16); 23] = [
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
            custom_symbols.insert(line, line_number);
        } else {
            line_number += 1;
        }
    }
    //remove all L instructions and save in lines
    let clean_lines = file.lines().filter(|line| !line.starts_with("(")).collect::<Vec<&str>>();
    //SECOND PASS
    //check if A instruction
    for line in clean_lines {
        if line.starts_with("@") {
            let line = line.trim_start_matches('@');

            if let Ok(number) = line.parse::<i16>() {
                // Using numbers directly
                let binary = format!("0{:015b}", number);
                output.push_str(&binary);
                output.push('\n');
            } else {
                // Check if the line is a predefined symbol or needs to be added as a custom symbol
                if !predefined_symbols.iter().any(|(symbol, _)| *symbol == line) && !custom_symbols.contains_key(line) {
                    let count = custom_symbols.len() as i16;
                    custom_symbols.insert(line, 16 + count); // Inserting a &str directly
                }
                // Using predefined symbols
                if let Some(&value) = predefined_symbols.iter().find(|(symbol, _)| *symbol == line).map(|(_, value)| value) {
                    let binary = format!("0{:015b}", value);
                    output.push_str(&binary);
                    output.push('\n');
                } 
                // Using custom symbols
                else if let Some(&value) = custom_symbols.get(line) {
                    let binary = format!("0{:015b}", value);
                    output.push_str(&binary);
                    output.push('\n');
                }
            }
        } else {
        //C instruction
            output.push_str("111");
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
            "0" => output.push_str("0101010"),
            "1" => output.push_str("0111111"),
            "-1" => output.push_str("0111010"),
            "D" => output.push_str("0001100"),
            "A" => output.push_str("0110000"),
            "M" => output.push_str("1110000"),
            "!D" => output.push_str("0001101"),
            "!A" => output.push_str("0110001"),
            "!M" => output.push_str("1110001"),
            "-D" => output.push_str("0001111"),
            "-A" => output.push_str("0110011"),
            "-M" => output.push_str("1110011"),
            "D+1" => output.push_str("0011111"),
            "A+1" => output.push_str("0110111"),
            "M+1" => output.push_str("1110111"),
            "D-1" => output.push_str("0001110"),
            "A-1" => output.push_str("0110010"),
            "M-1" => output.push_str("1110010"),
            "D+A" => output.push_str("0000010"),
            "D+M" => output.push_str("1000010"),
            "D-A" => output.push_str("0010011"),
            "D-M" => output.push_str("1010011"),
            "A-D" => output.push_str("0000111"),
            "M-D" => output.push_str("1000111"),
            "D&A" => output.push_str("0000000"),
            "D&M" => output.push_str("1000000"),
            "D|A" => output.push_str("0010101"),
            "D|M" => output.push_str("1010101"),
            _ => (),
        }
            //dest
            match line.split("=").nth(0) {
                Some("M") => output.push_str("001"),
                Some("D") => output.push_str("010"),
                Some("MD") => output.push_str("011"),
                Some("A") => output.push_str("100"),
                Some("AM") => output.push_str("101"),
                Some("AD") => output.push_str("110"),
                Some("AMD") => output.push_str("111"),
                _ => output.push_str("000")
            }
            //jump
            match line.split(";").nth(1) {
                Some("JGT") => output.push_str("001"),
                Some("JEQ") => output.push_str("010"),
                Some("JGE") => output.push_str("011"),
                Some("JLT") => output.push_str("100"),
                Some("JNE") => output.push_str("101"),
                Some("JLE") => output.push_str("110"),
                Some("JMP") => output.push_str("111"),
                _ => output.push_str("000")
            }
            output.push('\n');
        }
    }
    //trim last \n
    output.pop();
    output
}