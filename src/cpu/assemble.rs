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

impl Opval<'_> {
    pub fn to_numeric<'a>(
        &self,
        definitions: &HashMap<&'a str, Numeric>,
    ) -> Result<Numeric, ParseError> {
        match self {
            Self::Reference(s) => {
                if let Some(n) = definitions.get(s) {
                    Ok(*n)
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
    static ref LABEL_REGEX: Regex = Regex::new(r"^(?P<identifier>[_a-zA-Z]\w*):$").unwrap();
    static ref DEFINITION_REGEX: Regex = Regex::new(
        r"(?x)
        ^
        define
        \s+
        (?P<identifier>[a-zA-Z]\w*)
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
            [a-eA-E0-9][a-eA-E0-9]
            ([a-eA-E0-9][a-eA-E0-9])?
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
        return Ok(Statement::Label(caps.name("identifier").unwrap().as_str()));
    }

    if let Some(caps) = DEFINITION_REGEX.captures(line) {
        return Ok(Statement::Definition(
            caps.name("identifier").unwrap().as_str(),
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

fn parse_numeric<'a>(src: &'a str) -> Result<Numeric, Box<dyn Error>> {
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

fn parse_mnemonic<'a>(src: &'a str) -> Result<opcode::Type, Box<dyn Error>> {
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

fn parse_operand<'a>(src: &'a str) -> Result<Operand, Box<dyn Error>> {
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

fn parse_opval<'a>(src: &'a str) -> Result<Opval, Box<dyn Error>> {
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
pub fn assemble(src: &str) -> Vec<u8> {
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

    // setup tables for labels and definitions
    let mut labels: HashMap<&str, usize> = HashMap::new();
    let mut definitions: HashMap<&str, Numeric> = HashMap::new();
    let mut instructions = Vec::new();
    for s in statements.iter() {
        match s {
            Statement::Label(identifier) => {
                let addr = instructions.len();
                labels.insert(identifier, addr);
                definitions.insert(identifier, Numeric::Word(addr as u16));
            }
            Statement::Definition(identifier, numeric) => {
                definitions.insert(identifier, *numeric);
            }
            Statement::Instruction(opcode_type, operand) => {
                instructions.push((opcode_type, operand))
            }
        }
    }

    // resolve definition references and assign address modes
    // TODO: reduce code length and clean up error handling to track line numbers
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
                Operand::IndexX(opval) => match opval.to_numeric(&definitions).unwrap() {
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
                Operand::IndexY(opval) => match opval.to_numeric(&definitions).unwrap() {
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
                    if opcode_type.compatible_with(AddressMode::Relative) {
                        return AddressMode::Relative;
                    }

                    match opval.to_numeric(&definitions).unwrap() {
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

    println!("{:?}", address_modes);

    // generate instruction addresses
    let instruction_addrs = address_modes
        .iter()
        .fold((Vec::new(), 0), |(mut accum, next), addr_mode| {
            accum.push(next);
            return (accum, next + 1 + addr_mode.operand_size());
        })
        .0;

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
                                match labels.get(s) {
                                    Some(&other_ins) => {
                                        // The displacement operand is calculated
                                        // relative to the next instruction.
                                        let base_adr = instruction_addrs[i] + 2;
                                        let other_addr = instruction_addrs[other_ins];

                                        let delta = (base_adr as i64) - (other_addr as i64);
                                        if delta < -128 || 127 < delta {
                                            panic!("label {} is too far for relative address", s);
                                        }

                                        Some(Numeric::Byte(math::encode_i8_as_u8(delta as i8)))
                                    }
                                    None => panic!("can't find label {}", s),
                                }
                            }
                            Opval::Literal(n) => Some(*n),
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
                Operand::Immediate(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::IndexX(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::IndexY(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::Indirect(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::IndirectX(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::IndirectY(opval) => Some(opval.to_numeric(&definitions).unwrap()),
                Operand::Direct(opval) => Some(opval.to_numeric(&definitions).unwrap()),
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

    println!("{:?}", code);

    return code;
}

#[test]
fn test() {
    let example = "; abc
label1:
dex
rol
rol $01
lda #$0b ; hi

define foobar $01
define barfoo $0101

label2:
adc $01
adc $0101
adc foobar
adc barfoo
adc $01,x
adc $0101,x
adc foobar,x
adc barfoo,x
ldx $01,y
ldx $0101,y
ldx foobar,y
ldx barfoo,y
adc #$01
adc #foobar
jmp ($0101)
jmp (label2)
jmp (barfoo)
adc ($01,x)
adc (foobar,x)
adc ($01),y
adc (foobar),y
beq $01
beq label1
label3:
; yo
  ; yo
label4:

";
    assemble(example);
}
