use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{Cursor, Read};
use std::{fs, vec};

const WORD_BYTE: u8 = 0b00000001;
const BYTE_BYTE: u8 = 0b00000000;

lazy_static! {

    // This is when the register is a byte register (not a word register.)
    static ref BYTE_MAP_REG: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AL");
        m.insert(0b00001000, "CL");
        m.insert(0b00010000, "DL");
        m.insert(0b00011000, "BL");
        m.insert(0b00100000, "AH");
        m.insert(0b00101000, "CH");
        m.insert(0b00110000, "DH");
        m.insert(0b00111000, "BH");
        m.insert(0b00000001, "CL");
        m.insert(0b00000010, "DL");
        m.insert(0b00000011, "BL");
        m.insert(0b00000100, "AH");
        m.insert(0b00000101, "CH");
        m.insert(0b00000110, "DH");
        m.insert(0b00000111, "BH");

        m
    };

    // This is when the register is a word register (not a byte register)
    static ref WORD_MAP_REG: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AX");
        m.insert(0b00001000, "CX");
        m.insert(0b00010000, "DX");
        m.insert(0b00011000, "BX");
        m.insert(0b00100000, "SP");
        m.insert(0b00101000, "BP");
        m.insert(0b00110000, "SI");
        m.insert(0b00111000, "DI");
        m.insert(0b00000001, "CX");
        m.insert(0b00000010, "DX");
        m.insert(0b00000011, "BX");
        m.insert(0b00000100, "SP");
        m.insert(0b00000101, "BP");
        m.insert(0b00000110, "SI");
        m.insert(0b00000111, "DI");

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
    format!("MOV {dest},{src}")
}

fn write_file(content: Vec<String>) -> std::io::Result<()> {
    let mut buffer = File::create("result.asm").expect("File not created");
    for w in &content {
        match write!(buffer, "{}", w) {
            Ok(()) => {}
            Err(_) => {}
        }
    }
    Ok(())
}

/// Running the code in dev mode:
/// `cargo run -- <file_name>`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];

    let mut vector: Vec<String> = Vec::new();

    let bin_instructions = read_bin(file_name)?;

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

        vector.push(format!("{}\n", decoded_instruction));
    }

    write_file(vector).expect("Error in writing file.");

    Ok(())
}
