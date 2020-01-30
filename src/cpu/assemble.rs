use super::address_mode::AddressMode;
use super::opcode;
use crate::math;
use regex::Regex;
use std::collections::HashMap;
use std::error;
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
    pub fn to_numeric(&self, symbols: &dyn SymbolTable) -> Result<Numeric, Error> {
        match self {
            Self::Reference(s) => match symbols.get(s) {
                Some(n) => Ok(n),
                None => Err(Error::SymbolNotFound(s.to_string())),
            },
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
enum Error {
    InvalidStatement(String),
    InvalidNumeric(String),
    InvalidMnemonic(String),
    InvalidOperand(String),
    InvalidOpval(String),
    SymbolNotFound(String),
    NoValidAddressMode(opcode::Type, String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidStatement(src) => write!(f, "invalid statement: {}", src),
            Self::InvalidNumeric(src) => write!(f, "invalid numeric: {}", src),
            Self::InvalidMnemonic(src) => write!(f, "invalid mnemonic: {}", src),
            Self::InvalidOperand(src) => write!(f, "invalid operand: {}", src),
            Self::InvalidOpval(src) => write!(f, "invalid opval: {}", src),
            Self::SymbolNotFound(src) => write!(f, "symbol not found: {}", src),
            Self::NoValidAddressMode(opcode_type, operand_str) => write!(
                f,
                "no valid address mode for opcode type {:?} and operand {}",
                opcode_type, operand_str
            ),
        }
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

fn parse_statement<'a>(src: &'a str) -> Result<Statement<'a>, Error> {
    if let Some(caps) = LABEL_REGEX.captures(src) {
        return Ok(Statement::Label(caps.name("ident").unwrap().as_str()));
    }

    if let Some(caps) = DEFINITION_REGEX.captures(src) {
        return Ok(Statement::Definition(
            caps.name("ident").unwrap().as_str(),
            parse_numeric(caps.name("value").unwrap().as_str())?,
        ));
    }

    if let Some(caps) = INSTRUCTION_REGEX.captures(src) {
        let mut operand = Operand::None;
        if let Some(operand_src) = caps.name("operand") {
            operand = parse_operand(operand_src.as_str())?;
        }

        return Ok(Statement::Instruction(
            parse_mnemonic(caps.name("mnemonic").unwrap().as_str())?,
            operand,
        ));
    }

    Err(Error::InvalidStatement(src.to_string()))
}

fn parse_numeric(src: &str) -> Result<Numeric, Error> {
    if let Some(caps) = NUMERIC_REGEX.captures(src) {
        let digits = caps.name("digits").unwrap().as_str();

        if digits.len() == 2 {
            match u8::from_str_radix(digits, 16) {
                Ok(n) => return Ok(Numeric::Byte(n)),
                _ => return Err(Error::InvalidNumeric(src.to_string())),
            }
        }

        if digits.len() == 4 {
            match u16::from_str_radix(digits, 16) {
                Ok(n) => return Ok(Numeric::Word(n)),
                _ => return Err(Error::InvalidNumeric(src.to_string())),
            }
        }
    }

    Err(Error::InvalidNumeric(src.to_string()))
}

fn parse_mnemonic(src: &str) -> Result<opcode::Type, Error> {
    match opcode::Type::from_mnemonic(src) {
        Some(t) => Ok(t),
        None => Err(Error::InvalidMnemonic(src.to_string())),
    }
}

fn parse_operand<'a>(src: &'a str) -> Result<Operand<'a>, Error> {
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

    Err(Error::InvalidOperand(src.to_string()))
}

fn parse_opval<'a>(src: &'a str) -> Result<Opval<'a>, Error> {
    if let Some(caps) = IDENTIFIER_REGEX.captures(src) {
        return Ok(Opval::Reference(caps.name("value").unwrap().as_str()));
    }

    if let Ok(numeric) = parse_numeric(src) {
        return Ok(Opval::Literal(numeric));
    }

    Err(Error::InvalidOpval(src.to_string()))
}

fn infer_address_mode(
    opcode_type: opcode::Type,
    operand: &Operand,
    symbols: &dyn SymbolTable,
) -> Result<AddressMode, Error> {
    match operand {
        Operand::None => {
            if opcode_type.compatible_with(AddressMode::Implicit) {
                return Ok(AddressMode::Implicit);
            }
            if opcode_type.compatible_with(AddressMode::Accumulator) {
                return Ok(AddressMode::Accumulator);
            }
        }
        Operand::Immediate(_) => {
            if opcode_type.compatible_with(AddressMode::Immediate) {
                return Ok(AddressMode::Immediate);
            }
        }
        Operand::IndexX(opval) => match opval.to_numeric(symbols)? {
            Numeric::Byte(_) => {
                if opcode_type.compatible_with(AddressMode::ZeroPageX) {
                    return Ok(AddressMode::ZeroPageX);
                }
            }
            Numeric::Word(_) => {
                if opcode_type.compatible_with(AddressMode::AbsoluteX) {
                    return Ok(AddressMode::AbsoluteX);
                }
            }
        },
        Operand::IndexY(opval) => match opval.to_numeric(symbols)? {
            Numeric::Byte(_) => {
                if opcode_type.compatible_with(AddressMode::ZeroPageY) {
                    return Ok(AddressMode::ZeroPageY);
                }
            }
            Numeric::Word(_) => {
                if opcode_type.compatible_with(AddressMode::AbsoluteY) {
                    return Ok(AddressMode::AbsoluteY);
                }
            }
        },
        Operand::Indirect(_) => {
            if opcode_type.compatible_with(AddressMode::Indirect) {
                return Ok(AddressMode::Indirect);
            }
        }
        Operand::IndirectX(_) => {
            if opcode_type.compatible_with(AddressMode::IndirectX) {
                return Ok(AddressMode::IndirectX);
            }
        }
        Operand::IndirectY(_) => {
            if opcode_type.compatible_with(AddressMode::IndirectY) {
                return Ok(AddressMode::IndirectY);
            }
        }
        Operand::Direct(opval) => {
            // branches
            if opcode_type.compatible_with(AddressMode::Relative) {
                return Ok(AddressMode::Relative);
            }

            // jumps
            if opcode_type.is_jump() {
                return Ok(AddressMode::Absolute);
            }

            // literals and references
            match opval.to_numeric(symbols)? {
                Numeric::Byte(_) => return Ok(AddressMode::ZeroPage),
                Numeric::Word(_) => return Ok(AddressMode::Absolute),
            }
        }
    }

    Err(Error::NoValidAddressMode(
        opcode_type,
        format!("{:?}", operand),
    ))
}

// TODO: return an actual error message.
pub fn assemble(src: &str, base_reloc_addr: u16) -> Vec<u8> {
    // collect statements
    let mut statements = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let line = line.trim();

        // strip comments
        let line = match line.chars().position(|c| c == ';') {
            Some(pos) => line[0..pos].trim(),
            None => line,
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
            infer_address_mode(opcode_type, &operand, &def_symbols).unwrap()
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

                                        Some(Numeric::Byte((delta as i8).to_le_bytes()[0]))
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

        let operand_bytes = match num_opval {
            Some(n) => n.to_bytes(),
            None => Vec::new(),
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
