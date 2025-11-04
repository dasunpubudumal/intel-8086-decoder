use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

lazy_static! {
    static ref BYTE_MAP_REG1: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AL");
        m.insert(0b00001000, "CL");
        m.insert(0b00010000, "DL");
        m.insert(0b00011000, "BL");
        m.insert(0b00100000, "AH");
        m.insert(0b00101000, "CH");
        m.insert(0b00110000, "DH");
        m.insert(0b00111000, "BH");

        m
    };
    static ref WORD_MAP_REG1: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AX");
        m.insert(0b00001000, "CX");
        m.insert(0b00010000, "DX");
        m.insert(0b00011000, "BX");
        m.insert(0b00100000, "SP");
        m.insert(0b00101000, "BP");
        m.insert(0b00110000, "SI");
        m.insert(0b00111000, "DI");

        m
    };
    static ref BYTE_MAP_REG2: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AL");
        m.insert(0b00000001, "CL");
        m.insert(0b00000010, "DL");
        m.insert(0b00000011, "BL");
        m.insert(0b00000100, "AH");
        m.insert(0b00000101, "CH");
        m.insert(0b00110110, "DH");
        m.insert(0b00110111, "BH");

        m
    };
    static ref WORD_MAP_REG2: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0b00000000, "AX");
        m.insert(0b00000001, "CX");
        m.insert(0b00000010, "DX");
        m.insert(0b00000011, "BX");
        m.insert(0b00000100, "SP");
        m.insert(0b00100101, "BP");
        m.insert(0b00000110, "SI");
        m.insert(0b00000111, "DI");

        m
    };
}

/// Reads a bin file and returns the bit value
fn read_bin(filename: &str) -> std::io::Result<u16> {
    let data = fs::read(filename)?;
    let mut cursor = Cursor::new(&data);

    //  2 8-bit words.
    let mut word = [0u8; 2];
    // Read the data exactly into 2 x 8-bit words
    // This is because read_exact functions only apply to u8 values.
    if cursor.read_exact(&mut word).is_err() {}
    // The big-endian format is used here.
    // big-endian and little-endian source: https://en.wikipedia.org/wiki/Endianness
    // When we convert a byte array to a fixed-sized integer,
    // the value of the integer depends on how have represented the array in memory.
    // The value may differ between big and little endian representations.
    // So, when converting, we need to explicitly say which endianness we need to use.
    //
    // For example, if the byte array is of type [u8] [0x89, 0xd9] (where 0x89 and 0xd9 are both 8
    // bit digits as each value in hexa are represented with 4 bits; i.e., 0 -> 0000, 1 -> 0001,
    // etc.), how both 0x89 and 0xd9 are stored in memory is important when converting to
    // a binary representation.
    let value = u16::from_be_bytes(word);

    Ok(value)
}

fn main() -> std::io::Result<()> {
    let file_name = "listing_0037_single_register_mov";
    let bin_instruction = read_bin(file_name)?;

    println!("Instruction {:16b}", bin_instruction);

    // These are 16-bit values.
    let operation = bin_instruction & 0b1111110000000000;
    let direction = bin_instruction & 0b0000001000000000;
    let word = bin_instruction & 0b0000000100000000;
    // These are actually 8 bit values.
    let mode = bin_instruction & 0b0000000011000000;
    let reg1 = bin_instruction & 0b0000000000111000;
    let reg2 = bin_instruction & 0b0000000000000111;

    let _ = (operation >> 8) as u8;
    let _ = (direction >> 8) as u8;
    let word_u8 = (word >> 8) as u8;
    let _ = mode as u8;
    let reg1_u8 = reg1 as u8;
    let reg2_u8 = reg2 as u8;

    // operation, direction and word are 16 bit values because they are part of the first byte.
    // So, we need to shift the bits in order to make it one byte.
    // For example, if operation is 0b1000100000000000, shifting it 8 would be equal to 0b10001000.
    // If direction is 0b0000001000000000, then shifting it to 8 would be 0b00000010.
    // If word is 0b0000000100000000, then shifting it to 8 would be 0b00000001.
    //
    // If the REG value is 010, the u8 value of it would be 0b00010000.

    let mut src = String::new();
    let mut dest = String::new();

    println!("REG1 -> {:08b}, REG2 -> {:08b}", reg1_u8, reg2_u8);

    if word_u8 == 0b00000001 {
        // Use WORD_MAP to find what the registers are.
        let val1 = WORD_MAP_REG1[&reg1_u8];
        let val2 = WORD_MAP_REG2[&reg2_u8];

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
    } else if word_u8 == 0b00000000 {
        // Use BYTE_MAP to find what the registers are.
        let val1 = BYTE_MAP_REG1[&reg1_u8];
        let val2 = BYTE_MAP_REG2[&reg2_u8];

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
    println!("MOV {dest},{src}");

    Ok(())
}
