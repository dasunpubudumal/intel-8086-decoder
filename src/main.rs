use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

lazy_static! {
    static ref BYTE_MAP: HashMap<u8, &'static str> = {
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
    static ref WORD_MAP: HashMap<u8, &'static str> = {
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
}

/// Struct to hold the Instruction
struct Instruction {
    /// Operation: 6-bits
    operation: u8,
    /// D: 1-bit
    direction: u8,
    /// W: 1-bit.
    /// If W=1, we know that we are working with 2-byte registers (i.e., word-lengthed in 8086),
    /// while if W=0, we know that we are working with 1-byte registers.
    word: u8,
    /// MOD: 2-bits
    mode: u8,
    /// REG: 3-bits
    reg1: u8,
    /// REG: 3-bits
    reg2: u8,
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

    // These are 16-bit values.
    let operation = bin_instruction & 0b1111110000000000;
    let direction = bin_instruction & 0b0000001000000000;
    let word = bin_instruction & 0b0000000100000000;
    // These are actually 8 bit values.
    let mode = bin_instruction & 0b0000000011000000;
    let reg1 = bin_instruction & 0b0000000000111000;
    let reg2 = bin_instruction & 0b0000000000000111;

    // operation, direction and word are 16 bit values because they are part of the first byte.
    // So, we need to shift the bits in order to make it one byte.
    // For example, if operation is 0b1000100000000000, shifting it 8 would be equal to 0b10001000.
    // If direction is 0b0000001000000000, then shifting it to 8 would be 0b00000010.
    // If word is 0b0000000100000000, then shifting it to 8 would be 0b00000001.
    //
    // If the REG value is 010, the u8 value of it would be 0b00010000.
    let instruction = Instruction {
        operation: ((operation >> 8) as u8),
        direction: ((direction >> 8) as u8),
        word: ((word >> 8) as u8),
        mode: (mode as u8),
        reg1: (reg1 as u8),
        reg2: (reg2 as u8),
    };

    Ok(())
}
