use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{Cursor, Read};

const WORD_BYTE: u8 = 0b00000001;
const BYTE_BYTE: u8 = 0b00000000;

lazy_static! {

    // This is when the register is a byte register (not a word register.)
    static ref BYTE_MAP_REG: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "al");
        m.insert(0b00001000, "cl");
        m.insert(0b00010000, "dl");
        m.insert(0b00011000, "bl");
        m.insert(0b00100000, "ah");
        m.insert(0b00101000, "ch");
        m.insert(0b00110000, "dh");
        m.insert(0b00111000, "bh");
        m.insert(0b00000001, "cl");
        m.insert(0b00000010, "dl");
        m.insert(0b00000011, "bl");
        m.insert(0b00000100, "ah");
        m.insert(0b00000101, "ch");
        m.insert(0b00000110, "dh");
        m.insert(0b00000111, "bh");

        m
    };

    // This is when the register is a word register (not a byte register)
    static ref WORD_MAP_REG: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "ax");
        m.insert(0b00001000, "cx");
        m.insert(0b00010000, "dx");
        m.insert(0b00011000, "bx");
        m.insert(0b00100000, "sp");
        m.insert(0b00101000, "bp");
        m.insert(0b00110000, "si");
        m.insert(0b00111000, "di");
        m.insert(0b00000001, "cx");
        m.insert(0b00000010, "dx");
        m.insert(0b00000011, "bx");
        m.insert(0b00000100, "sp");
        m.insert(0b00000101, "bp");
        m.insert(0b00000110, "si");
        m.insert(0b00000111, "di");

        m
    };
}

/// Reads a bin file and returns the u16 values in a vector.
fn read_bin(filename: &str) -> std::io::Result<Vec<u16>> {
    let data = fs::read(filename)?;
    let mut cursor = Cursor::new(&data);
    let mut instructions: Vec<u16> = Vec::new();

    loop {
        let mut word = [0u8; 2];
        if cursor.read_exact(&mut word).is_err() {
            break;
        }

        instructions.push(u16::from_be_bytes(word));
    }

    Ok(instructions)
}

fn decode(operation: u8, direction: u8, word: u8, mode: u8, reg1: u8, reg2: u8) -> String {
    // operation, direction and word are 16 bit values because they are part of the first byte.
    // So, we need to shift the bits in order to make it one byte.
    // For example, if operation is 0b1000100000000000, shifting it 8 would be equal to 0b10001000.
    // If direction is 0b0000001000000000, then shifting it to 8 would be 0b00000010.
    // If word is 0b0000000100000000, then shifting it to 8 would be 0b00000001.
    //
    // If the REG value is 010, the u8 value of it would be 0b00010000.

    let mut src = String::new();
    let mut dest = String::new();

    if word == WORD_BYTE {
        // Use WORD_MAP to find what the registers are.
        let val1 = WORD_MAP_REG[&reg1];
        let val2 = WORD_MAP_REG[&reg2];

        if direction == 0b00000010 {
            // D = 1
            // Then reg1 is the destination.
            src = String::from(val2);
            dest = String::from(val1);
        } else {
            // D = 0
            // Then reg2 is the destination.
            src = String::from(val1);
            dest = String::from(val2);
        }
    } else if word == BYTE_BYTE {
        // Use BYTE_MAP to find what the registers are.
        let val1 = BYTE_MAP_REG[&reg1];
        let val2 = BYTE_MAP_REG[&reg2];

        if direction == 0b00000010 {
            // D = 1
            // Then reg1 is the destination.
            src = String::from(val2);
            dest = String::from(val1);
        } else {
            // D = 0
            // Then reg2 is the destination.
            src = String::from(val1);
            dest = String::from(val2);
        }
    } else {
        // Unknown value!
    }
    format!("mov {dest},{src}")
}

fn write_file_from_string(content: &str) -> std::io::Result<()> {
    let mut buffer = File::create("result.asm").expect("File not created.");
    match write!(buffer, "{}", content) {
        Ok(()) => return Ok(()),
        Err(_) => {}
    }
    Ok(())
}

/// Running the code in dev mode:
/// `cargo run -- <file_name>`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];

    let mut output_string = String::from("bits 16;\n\n");

    let bin_instructions = read_bin(&file_name)?;

    for bin_instruction in bin_instructions {
        // These are 16-bit values.
        let operation = ((bin_instruction & 0b1111110000000000) >> 8) as u8;
        let direction = ((bin_instruction & 0b0000001000000000) >> 8) as u8;
        let word = ((bin_instruction & 0b0000000100000000) >> 8) as u8;
        // These are actually 8 bit values (the first byte is 0).
        let mode = ((bin_instruction & 0b0000000011000000) >> 8) as u8;
        let reg1 = (bin_instruction & 0b0000000000111000) as u8;
        let reg2 = (bin_instruction & 0b0000000000000111) as u8;

        let decoded_instruction = decode(operation, direction, word, mode, reg1, reg2);

        output_string.push_str(format!("{decoded_instruction}\n").as_str());
    }

    write_file_from_string(output_string.as_str()).expect("Error in writing file.");

    Ok(())
}
