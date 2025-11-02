use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};

lazy_static! {
    static ref BYTE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("000", "AL");
        m.insert("001", "CL");
        m.insert("010", "DL");
        m.insert("011", "BL");
        m.insert("100", "AH");
        m.insert("101", "CH");
        m.insert("110", "DH");
        m.insert("111", "BH");
        m
    };
    static ref WORD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("000", "AX");
        m.insert("001", "CX");
        m.insert("010", "DX");
        m.insert("011", "BX");
        m.insert("100", "AX");
        m.insert("101", "CX");
        m.insert("110", "DX");
        m.insert("111", "BX");
        m
    };
}

/// Struct to hold the Instruction
struct Instruction {
    /// Operation: 6-bits
    operation: &'static str,
    /// D: 1-bit
    direction: &'static str,
    /// W: 1-bit.
    /// If W=1, we know that we are working with 2-byte registers (i.e., word-lengthed in 8086),
    /// while if W=0, we know that we are working with 1-byte registers.
    word: &'static str,
    /// MOD: 2-bits
    mode: &'static str,
    /// REG: 3-bits
    reg1: &'static str,
    /// REG: 3-bits
    reg2: &'static str,
}

/// Reads a bin file and returns the bit value
/// into a string.
fn read_bin(filename: &str) -> std::io::Result<String> {
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

    Ok(format!("{:16b}", value))
}

fn main() -> std::io::Result<()> {
    let file_name = "listing_0038_many_register_mov";
    let bin_in_hexa = read_bin(file_name)?;

    println!("Binary encoded: {}", bin_in_hexa);

    Ok(())
}
