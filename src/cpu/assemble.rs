use super::address_mode::AddressMode;
use super::opcode;
use crate::math;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum Statement<'a> {
    Label(&'a str),
    Definition(&'a str, Numeric),
    Instruction(opcode::Type, Operand<'a>),
}

#[derive(Debug)]
enum Operand<'a> {
    None,                 // address modes: implicit, accumulator
    Immediate(Opval<'a>), // address modes: immediate
    IndexX(Opval<'a>),    // address modes: zero page x, absolute x
    IndexY(Opval<'a>),    // address modes: zero page y, absolute y
    Indirect(Opval<'a>),  // address modes: indirect
    IndirectX(Opval<'a>), // address modes: indirect x
    IndirectY(Opval<'a>), // address modes: indirect y
    Direct(Opval<'a>),    // address modes: absolute, relative, zero page
}

#[derive(Debug)]
enum Opval<'a> {
    Reference(&'a str),
    Literal(Numeric),
}

impl<'a> Opval<'a> {
    pub fn to_numeric(&self, symbols: &dyn SymbolTable) -> Result<Numeric, ParseError> {
        match self {
            Self::Reference(s) => {
                if let Some(n) = symbols.get(s) {
                    Ok(n)
                } else {
                    Err(ParseError {
                        msg: format!("no definition found for \"{}\"", s),
                        src: String::from(""),
                    })
                }
            }
            Self::Literal(n) => Ok(*n),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Numeric {
    Byte(u8),
    Word(u16),
}

impl Numeric {
    fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::Byte(n) => vec![n],
            Self::Word(n) => math::u16_to_bytes_le(n).iter().cloned().collect(),
        }
    }

    fn to_u16(self) -> Option<u16> {
        match self {
            Self::Byte(_) => None,
            Self::Word(n) => Some(n),
        }
    }
}

trait SymbolTable {
    fn get(&self, symbol: &str) -> Option<Numeric>;
}

struct MapSymbolTable<'a>(HashMap<&'a str, Numeric>);

impl<'a> SymbolTable for MapSymbolTable<'a> {
    fn get(&self, symbol: &str) -> Option<Numeric> {
        self.0.get(symbol).map(|n| *n)
    }
}

struct CompositeSymbolTable<'a>(Vec<&'a dyn SymbolTable>);

impl<'a> SymbolTable for CompositeSymbolTable<'a> {
    fn get(&self, symbol: &str) -> Option<Numeric> {
        for m in self.0.iter() {
            let n = m.get(symbol);
            match n {
                Some(_) => return n,
                None => (),
            }
        }

        None
    }
}

#[derive(Debug)]
struct ParseError {
    msg: String,
    src: String,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.msg, self.src)
    }
}

lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new(r"^(?P<ident>[_a-zA-Z]\w*):$").unwrap();
    static ref DEFINITION_REGEX: Regex = Regex::new(
        r"(?x)
        ^
        define
        \s+
        (?P<ident>[a-zA-Z]\w*)
        \s+
        (?P<value>\S+)
        $
    "
    )
    .unwrap();
    static ref INSTRUCTION_REGEX: Regex = Regex::new(
        r"(?x)
        ^
        (?P<mnemonic>[a-zA-Z]{3})
        (
            \s+
            (?P<operand>\S*)
        )?
        $
    "
    )
    .unwrap();
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^(?P<value>[_a-zA-Z]\w*)$").unwrap();
    static ref NUMERIC_REGEX: Regex = Regex::new(
        r"(?x)
        ^
        \$
        (?P<digits>
            [a-fA-F0-9][a-fA-F0-9]
            ([a-fA-F0-9][a-fA-F0-9])?
        )
        $
    "
    )
    .unwrap();
    static ref IMMEDIATE_REGEX: Regex = Regex::new(r"^#(?P<opval>\S+)$").unwrap();
    static ref INDEX_X_REGEX: Regex = Regex::new(r"^(?P<opval>\$?\w+),[xX]$").unwrap();
    static ref INDEX_Y_REGEX: Regex = Regex::new(r"^(?P<opval>\$?\w+),[yY]$").unwrap();
    static ref INDIRECT_REGEX: Regex = Regex::new(r"^\((?P<opval>\$?\w+)\)$").unwrap();
    static ref INDIRECT_X_REGEX: Regex = Regex::new(r"^\((?P<opval>\$?\w+),[xX]\)$").unwrap();
    static ref INDIRECT_Y_REGEX: Regex = Regex::new(r"^\((?P<opval>\$?\w+)\),[yY]$").unwrap();
}

fn parse_statement<'a>(line: &'a str) -> Result<Statement<'a>, Box<dyn Error>> {
    if let Some(caps) = LABEL_REGEX.captures(line) {
        return Ok(Statement::Label(caps.name("ident").unwrap().as_str()));
    }

    if let Some(caps) = DEFINITION_REGEX.captures(line) {
        return Ok(Statement::Definition(
            caps.name("ident").unwrap().as_str(),
            parse_numeric(caps.name("value").unwrap().as_str())?,
        ));
    }

    if let Some(caps) = INSTRUCTION_REGEX.captures(line) {
        let mut operand = Operand::None;
        if let Some(operand_src) = caps.name("operand") {
            operand = parse_operand(operand_src.as_str())?;
        }

        return Ok(Statement::Instruction(
            parse_mnemonic(caps.name("mnemonic").unwrap().as_str())?,
            operand,
        ));
    }

    Err(Box::new(ParseError {
        msg: String::from("invalid statement"),
        src: line.to_string(),
    }))
}

fn parse_numeric(src: &str) -> Result<Numeric, Box<dyn Error>> {
    if let Some(caps) = NUMERIC_REGEX.captures(src) {
        let digits = caps.name("digits").unwrap().as_str();
        if digits.len() == 2 {
            return Ok(Numeric::Byte(u8::from_str_radix(digits, 16)?));
        }

        if digits.len() == 4 {
            let hi = u8::from_str_radix(&digits[0..2], 16)?;
            let lo = u8::from_str_radix(&digits[2..4], 16)?;
            return Ok(Numeric::Word(math::bytes_to_u16_le([lo, hi])));
        }
    }

    Err(Box::new(ParseError {
        msg: String::from("invalid literal"),
        src: src.to_string(),
    }))
}

fn parse_mnemonic(src: &str) -> Result<opcode::Type, Box<dyn Error>> {
    match src.to_ascii_uppercase().as_str() {
        "ADC" => Ok(opcode::Type::Adc),
        "AND" => Ok(opcode::Type::And),
        "ASL" => Ok(opcode::Type::Asl),
        "BCC" => Ok(opcode::Type::Bcc),
        "BCS" => Ok(opcode::Type::Bcs),
        "BEQ" => Ok(opcode::Type::Beq),
        "BIT" => Ok(opcode::Type::Bit),
        "BMI" => Ok(opcode::Type::Bmi),
        "BNE" => Ok(opcode::Type::Bne),
        "BPL" => Ok(opcode::Type::Bpl),
        "BRK" => Ok(opcode::Type::Brk),
        "BVC" => Ok(opcode::Type::Bvc),
        "BVS" => Ok(opcode::Type::Bvs),
        "CLC" => Ok(opcode::Type::Clc),
        "CLD" => Ok(opcode::Type::Cld),
        "CLI" => Ok(opcode::Type::Cli),
        "CLV" => Ok(opcode::Type::Clv),
        "CMP" => Ok(opcode::Type::Cmp),
        "CPX" => Ok(opcode::Type::Cpx),
        "CPY" => Ok(opcode::Type::Cpy),
        "DEC" => Ok(opcode::Type::Dec),
        "DEX" => Ok(opcode::Type::Dex),
        "DEY" => Ok(opcode::Type::Dey),
        "EOR" => Ok(opcode::Type::Eor),
        "INC" => Ok(opcode::Type::Inc),
        "INX" => Ok(opcode::Type::Inx),
        "INY" => Ok(opcode::Type::Iny),
        "JMP" => Ok(opcode::Type::Jmp),
        "JSR" => Ok(opcode::Type::Jsr),
        "LDA" => Ok(opcode::Type::Lda),
        "LDX" => Ok(opcode::Type::Ldx),
        "LDY" => Ok(opcode::Type::Ldy),
        "LSR" => Ok(opcode::Type::Lsr),
        "NOP" => Ok(opcode::Type::Nop),
        "ORA" => Ok(opcode::Type::Ora),
        "PHA" => Ok(opcode::Type::Pha),
        "PHP" => Ok(opcode::Type::Php),
        "PLA" => Ok(opcode::Type::Pla),
        "PLP" => Ok(opcode::Type::Plp),
        "ROL" => Ok(opcode::Type::Rol),
        "ROR" => Ok(opcode::Type::Ror),
        "RTI" => Ok(opcode::Type::Rti),
        "RTS" => Ok(opcode::Type::Rts),
        "SBC" => Ok(opcode::Type::Sbc),
        "SEC" => Ok(opcode::Type::Sec),
        "SED" => Ok(opcode::Type::Sed),
        "SEI" => Ok(opcode::Type::Sei),
        "STA" => Ok(opcode::Type::Sta),
        "STX" => Ok(opcode::Type::Stx),
        "STY" => Ok(opcode::Type::Sty),
        "TAX" => Ok(opcode::Type::Tax),
        "TAY" => Ok(opcode::Type::Tay),
        "TSX" => Ok(opcode::Type::Tsx),
        "TXA" => Ok(opcode::Type::Txa),
        "TXS" => Ok(opcode::Type::Txs),
        "TYA" => Ok(opcode::Type::Tya),
        _ => Err(Box::new(ParseError {
            msg: String::from("invalid opcode"),
            src: src.to_string(),
        })),
    }
}

fn parse_operand<'a>(src: &'a str) -> Result<Operand<'a>, Box<dyn Error>> {
    if let Some(caps) = IMMEDIATE_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::Immediate(opval));
    }

    if let Some(caps) = INDEX_X_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::IndexX(opval));
    }

    if let Some(caps) = INDEX_Y_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::IndexY(opval));
    }

    if let Some(caps) = INDIRECT_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::Indirect(opval));
    }

    if let Some(caps) = INDIRECT_X_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::IndirectX(opval));
    }

    if let Some(caps) = INDIRECT_Y_REGEX.captures(src) {
        let opval = parse_opval(caps.name("opval").unwrap().as_str())?;
        return Ok(Operand::IndirectY(opval));
    }

    if let Ok(opval) = parse_opval(src) {
        return Ok(Operand::Direct(opval));
    }

    Err(Box::new(ParseError {
        msg: String::from("invalid operand"),
        src: src.to_string(),
    }))
}

fn parse_opval<'a>(src: &'a str) -> Result<Opval<'a>, Box<dyn Error>> {
    if let Some(caps) = IDENTIFIER_REGEX.captures(src) {
        return Ok(Opval::Reference(caps.name("value").unwrap().as_str()));
    }

    if let Ok(numeric) = parse_numeric(src) {
        return Ok(Opval::Literal(numeric));
    }

    Err(Box::new(ParseError {
        msg: String::from("invalid opval"),
        src: src.to_string(),
    }))
}

// TODO: return an actual error message.
pub fn assemble(src: &str, base_reloc_addr: u16) -> Vec<u8> {
    // collect statements
    let mut statements = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let line = line.trim();

        // strip comments
        let line = if let Some(pos) = line.chars().position(|c| c == ';') {
            line[0..pos].trim()
        } else {
            line
        };

        if line.len() == 0 {
            continue;
        }

        match parse_statement(line) {
            Ok(s) => statements.push(s),
            Err(e) => panic!("error on line {}: {:?}\n{}", i, e, line),
        }
    }

    // To ease calculations for jumps and branches to a label at the end of the
    // source, we'll put a synthetic NOP at the end, and avoid emitting code at
    // the end.
    statements.push(Statement::Instruction(opcode::Type::Nop, Operand::None));

    // setup tables for labels and definitions
    let mut instructions_by_label: HashMap<&str, usize> = HashMap::new();
    let mut definitions: HashMap<&str, Numeric> = HashMap::new();
    let mut instructions = Vec::new();

    for s in statements.iter() {
        match s {
            Statement::Label(label) => {
                instructions_by_label.insert(label, instructions.len());
            }
            Statement::Definition(ident, numeric) => {
                definitions.insert(ident, *numeric);
            }
            Statement::Instruction(opcode_type, operand) => {
                instructions.push((opcode_type, operand))
            }
        }
    }

    let def_symbols = MapSymbolTable(definitions);

    // Infer address modes, which helps us determine exact instruction sizes
    // and label addresses.
    let address_modes: Vec<AddressMode> = instructions
        .iter()
        .map(|(&opcode_type, operand)| {
            match operand {
                Operand::None => {
                    if opcode_type.compatible_with(AddressMode::Implicit) {
                        return AddressMode::Implicit;
                    }
                    if opcode_type.compatible_with(AddressMode::Accumulator) {
                        return AddressMode::Accumulator;
                    }
                }
                Operand::Immediate(_) => {
                    if opcode_type.compatible_with(AddressMode::Immediate) {
                        return AddressMode::Immediate;
                    }
                }
                Operand::IndexX(opval) => match opval.to_numeric(&def_symbols).unwrap() {
                    Numeric::Byte(_) => {
                        if opcode_type.compatible_with(AddressMode::ZeroPageX) {
                            return AddressMode::ZeroPageX;
                        }
                    }
                    Numeric::Word(_) => {
                        if opcode_type.compatible_with(AddressMode::AbsoluteX) {
                            return AddressMode::AbsoluteX;
                        }
                    }
                },
                Operand::IndexY(opval) => match opval.to_numeric(&def_symbols).unwrap() {
                    Numeric::Byte(_) => {
                        if opcode_type.compatible_with(AddressMode::ZeroPageY) {
                            return AddressMode::ZeroPageY;
                        }
                    }
                    Numeric::Word(_) => {
                        if opcode_type.compatible_with(AddressMode::AbsoluteY) {
                            return AddressMode::AbsoluteY;
                        }
                    }
                },
                Operand::Indirect(_) => {
                    if opcode_type.compatible_with(AddressMode::Indirect) {
                        return AddressMode::Indirect;
                    }
                }
                Operand::IndirectX(_) => {
                    if opcode_type.compatible_with(AddressMode::IndirectX) {
                        return AddressMode::IndirectX;
                    }
                }
                Operand::IndirectY(_) => {
                    if opcode_type.compatible_with(AddressMode::IndirectY) {
                        return AddressMode::IndirectY;
                    }
                }
                Operand::Direct(opval) => {
                    // branches
                    if opcode_type.compatible_with(AddressMode::Relative) {
                        return AddressMode::Relative;
                    }

                    // jumps
                    if let Opval::Reference(ident) = opval {
                        if instructions_by_label.contains_key(ident) {
                            return AddressMode::Absolute;
                        }
                    }

                    // literals and references
                    match opval.to_numeric(&def_symbols).unwrap() {
                        Numeric::Byte(_) => return AddressMode::ZeroPage,
                        Numeric::Word(_) => return AddressMode::Absolute,
                    }
                }
            }

            panic!(
                "incompatible operand {:?} for opcode type {:?}",
                operand, opcode_type
            );
        })
        .collect();

    // generate instruction addresses
    let instruction_addrs = address_modes
        .iter()
        .fold(
            (Vec::new(), base_reloc_addr),
            |(mut accum, next), addr_mode| {
                accum.push(next);
                return (accum, next + 1 + addr_mode.operand_size() as u16);
            },
        )
        .0;

    // For jumps/branches, we want additional symbol table that maps labels to
    // relocated addresses.
    let mut addrs_by_label = HashMap::new();
    for (label, ins) in instructions_by_label.iter() {
        addrs_by_label.insert(*label, Numeric::Word(instruction_addrs[*ins]));
    }

    let label_symbols = MapSymbolTable(addrs_by_label);
    let all_symbols = CompositeSymbolTable(vec![&label_symbols, &def_symbols]);

    // Remove the NOP inserted for dealing with labels that appear as the last
    // statement.
    instructions.pop();

    // code generation
    let mut code = Vec::new();
    for (i, (&opcode_type, operand)) in instructions.iter().enumerate() {
        let addr_mode = address_modes[i];

        let num_opval = match addr_mode {
            // Relative address modes treat reference operands as labels, and
            // the encoded version should be a signed delta value.
            AddressMode::Relative => {
                match operand {
                    Operand::Direct(opval) => {
                        match opval {
                            Opval::Reference(s) => {
                                match label_symbols.get(s) {
                                    Some(dest) => {
                                        // The displacement operand is calculated
                                        // relative to the next instruction.
                                        let src = instruction_addrs[i] + 2;
                                        let delta = (dest.to_u16().unwrap() as i64) - (src as i64);
                                        if delta < -128 || 127 < delta {
                                            panic!("label {} is too far for relative address", s);
                                        }

                                        Some(Numeric::Byte(math::encode_i8_as_u8(delta as i8)))
                                    }
                                    None => panic!("can't find label {}", s),
                                }
                            }
                            Opval::Literal(_) => {
                                panic!("can't use literal {:?} for relative address mode", operand)
                            }
                        }
                    }
                    _ => panic!(
                        "invalid operand {:?} for opcode type {:?}",
                        operand, opcode_type
                    ),
                }
            }
            _ => match operand {
                Operand::None => None,
                Operand::Immediate(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::IndexX(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::IndexY(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::Indirect(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::IndirectX(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::IndirectY(opval) => Some(opval.to_numeric(&def_symbols).unwrap()),
                Operand::Direct(opval) => {
                    // Jumps can use labels.
                    let symbols: &dyn SymbolTable = if opcode_type.is_jump() {
                        &all_symbols
                    } else {
                        &def_symbols
                    };
                    Some(opval.to_numeric(symbols).unwrap())
                }
            },
        };

        let operand_bytes = if let Some(n) = num_opval {
            n.to_bytes()
        } else {
            Vec::new()
        };

        if operand_bytes.len() != addr_mode.operand_size() {
            panic!(
                "invalid size for operand {:?} for opcode type {:?}",
                operand, opcode_type
            );
        }

        // Write opcode
        let oc = opcode::encode(opcode_type, addr_mode).unwrap();
        code.push(oc);

        // Write operand
        for &v in operand_bytes.iter() {
            code.push(v);
        }
    }

    return code;
}

#[test]
fn test() {
    // comment/whitespace filtering
    let asm = "

; hi

nop ;inline

nop;inline

  ; byte

";
    assert_eq!(assemble(asm, 0), vec![0xEA, 0xEA]);

    let asm = "
define addr $01FF
define byte $69
label_a: ; label at start
dex ; implicit
adc #$01 ; immediate literal
adc #byte ; immediate ref
adc $01 ; zero page literal
adc byte ; zero page ref
adc $01,x ; zero page x literal
adc byte,x ; zero page x literal
ldx $01,y ; zero page y literal
ldx byte,y ; zero page y ref
beq label_a ; branch to opening label
beq label_b ; branch to intermediate label
beq label_c ; branch to terminating label
jmp $0101 ; jump to literal
jmp addr ; jump to ref
jmp label_a ; jump to opening label
jmp label_b ; jump to intermediate label
jmp label_c ; jump to terminating label
jmp ($0101) ; indirect literal
jmp (addr) ; indirect ref
label_b: ; intermediate label
adc $0101 ; absolute literal
adc addr ; absolute ref
adc $0101,x ; absolute x literal
adc addr,x ; absolute x ref
adc $0101,y ; absolute y literal
adc addr,y ; absolute y ref
adc ($01,x) ; indirect x literal
adc (byte,x) ; indirect x ref
adc ($01),y ; indirect y literal
adc (byte),y ; indirect y ref
label_c: ; terminal label
";
    assert_eq!(
        assemble(asm, 0x600),
        vec![
            // hexdump generated via: https://skilldrick.github.io/easy6502
            0xCA, 0x69, 0x01, 0x69, 0x69, 0x65, 0x01, 0x65, 0x69, 0x75, 0x01, 0x75, 0x69, 0xB6,
            0x01, 0xB6, 0x69, 0xF0, 0xED, 0xF0, 0x17, 0xF0, 0x2F, 0x4C, 0x01, 0x01, 0x4C, 0xFF,
            0x01, 0x4C, 0x00, 0x06, 0x4C, 0x2C, 0x06, 0x4C, 0x46, 0x06, 0x6C, 0x01, 0x01, 0x6C,
            0xFF, 0x01, 0x6D, 0x01, 0x01, 0x6D, 0xFF, 0x01, 0x7D, 0x01, 0x01, 0x7D, 0xFF, 0x01,
            0x79, 0x01, 0x01, 0x79, 0xFF, 0x01, 0x61, 0x01, 0x61, 0x69, 0x71, 0x01, 0x71, 0x69
        ]
    );
}
