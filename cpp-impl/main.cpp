#include <bitset>
#include <cstdint>
#include <format>
#include <fstream>
#include <ios>
#include <iostream>
#include <map>
#include <string>
#include <vector>

typedef std::map<uint8_t, std::string> Table;

const Table BYTE_MAP = {
    {0b00000000, "al"}, {0b00001000, "cl"}, {0b00010000, "dl"},
    {0b00011000, "bl"}, {0b00100000, "ah"}, {0b00101000, "ch"},
    {0b00110000, "dh"}, {0b00111000, "bh"}, {0b00000001, "cl"},
    {0b00000010, "dl"}, {0b00000011, "bl"}, {0b00000100, "ah"},
    {0b00000101, "ch"}, {0b00000110, "dh"}, {0b00000111, "bh"},

};
const Table WORD_MAP = {
    {0b00000000, "ax"}, {0b00001000, "cx"}, {0b00010000, "dx"},
    {0b00011000, "bx"}, {0b00100000, "sp"}, {0b00101000, "bp"},
    {0b00110000, "si"}, {0b00111000, "di"}, {0b00000001, "cx"},
    {0b00000010, "dx"}, {0b00000011, "bx"}, {0b00000100, "sp"},
    {0b00000101, "bp"}, {0b00000110, "si"}, {0b00000111, "di"},
};

const int8_t WORD_BYTE = 0b00000001;
const int8_t BYTE_BYTE = 0b00000000;

// Short type is 2 bytes
// https://cplusplus.com/doc/tutorial/files/
std::vector<int16_t> read_file(const std::string &file_name) {
  std::ifstream infile(file_name, std::ios::binary);
  if (!infile) {
    throw std::runtime_error("Failed to open file: " + file_name);
  }

  // find file size
  infile.seekg(0, std::ios::end);
  std::streamsize size = infile.tellg();
  infile.seekg(0, std::ios::beg);

  // size must be multiple of 2 bytes (16 bits)
  if (size % sizeof(int16_t) != 0) {
    throw std::runtime_error("File size is not a multiple of int16_t");
  }

  std::vector<int16_t> data(size / sizeof(int16_t));
  infile.read(reinterpret_cast<char *>(data.data()), size);

  return data;
}

void assign_src_dst(std::string *src, std::string *dst, int8_t reg1,
                    int8_t reg2, int8_t direction, Table map) {
  std::string val1 = WORD_MAP.at(reg1);
  std::string val2 = WORD_MAP.at(reg2);
  if (direction == 0b00000010) {
    *src = val2;
    *dst = val1;
  } else {
    *src = val1;
    *dst = val2;
  }
}

std::string decode(int16_t instruction) {
  int8_t operation = ((instruction & 0b1111110000000000) >> 8);
  int8_t direction = ((instruction & 0b0000001000000000) >> 8);
  int8_t word = ((instruction & 0b0000000100000000) >> 8);
  int8_t mode = ((instruction & 0b0000000011000000) >> 8);
  int8_t reg1 = instruction & 0b0000000000111000;
  int8_t reg2 = instruction & 0b0000000000000111;

  std::string src = "";
  std::string dst = "";

  if (word == WORD_BYTE) {
    assign_src_dst(&src, &dst, reg1, reg2, direction, WORD_MAP);
  } else if (word == BYTE_BYTE) {
    assign_src_dst(&src, &dst, reg1, reg2, direction, BYTE_MAP);
  } else {
    // Unknown value!
  }
  return std::format("mov {}, {}", dst, src);
}

int main() {
  std::ifstream infile("listing_0037_single_register_mov", std::ios::binary);
  if (!infile) {
    std::cerr << "Failed to open file\n";
    return 1;
  }

  while (true) {
    uint8_t lo, hi;

    // Read two bytes (big-endian)
    if (!infile.read(reinterpret_cast<char *>(&hi), sizeof(uint8_t)))
      break;
    if (!infile.read(reinterpret_cast<char *>(&lo), sizeof(uint8_t)))
      break;

    // Recombine into a 16-bit 8086 instruction
    uint16_t instr =
        static_cast<uint16_t>(lo) | (static_cast<uint16_t>(hi) << 8);

    std::string decoded = decode(instr);

    std::cout << std::format("Decoded instruction: {}\n", decoded);

    // Print binary string (16 bits)
    std::cout << std::bitset<16>(instr).to_string() << "\n";
  }
  return 0;
}
