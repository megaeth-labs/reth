//! This module is used to support the display of opcode statistics metrics.
use revm::revm_opcode::*;
use revm_utils::metrics::types::OpcodeRecord;

pub(crate) const OPCODE_NUMBER: usize = 256;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct OpcodeInfo {
    /// opcode category
    pub(crate) category: &'static str,
    /// gas fee
    pub(crate) gas: u64,
    /// opcode cost a fixed gas fee?
    pub(crate) static_gas: bool,
}

pub(crate) const MERGE_MAP: [Option<(u8, OpcodeInfo)>; OPCODE_NUMBER] = [
    Some((STOP, OpcodeInfo { category: "stop", gas: 0, static_gas: true })), //0x00
    Some((ADD, OpcodeInfo { category: "arithmetic", gas: 3, static_gas: true })), //0x01
    Some((MUL, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x02
    Some((SUB, OpcodeInfo { category: "arithmetic", gas: 3, static_gas: true })), //0x03
    Some((DIV, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x04
    Some((SDIV, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x05
    Some((MOD, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x06
    Some((SMOD, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x07
    Some((ADDMOD, OpcodeInfo { category: "arithmetic", gas: 8, static_gas: true })), //0x08
    Some((MULMOD, OpcodeInfo { category: "arithmetic", gas: 8, static_gas: true })), //0x09
    Some((EXP, OpcodeInfo { category: "arithmetic", gas: 10, static_gas: false })), //0x0a
    Some((SIGNEXTEND, OpcodeInfo { category: "arithmetic", gas: 5, static_gas: true })), //0x0b
    None,                                                                    //0x0c
    None,                                                                    //0x0d
    None,                                                                    //0x0e
    None,                                                                    //0x0f
    Some((LT, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x10
    Some((GT, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x11
    Some((SLT, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x12
    Some((SGT, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x13
    Some((EQ, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x14
    Some((ISZERO, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x15
    Some((AND, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x16
    Some((OR, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x17
    Some((XOR, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x18
    Some((NOT, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x19
    Some((BYTE, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x1a
    Some((SHL, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x1b
    Some((SHR, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x1c
    Some((SAR, OpcodeInfo { category: "bitwise", gas: 3, static_gas: true })), //0x1d
    None,                                                                    //0x1e
    None,                                                                    //0x1f
    Some((KECCAK256, OpcodeInfo { category: "system", gas: 30, static_gas: false })), //0x20
    None,                                                                    //0x21
    None,                                                                    //0x22
    None,                                                                    //0x23
    None,                                                                    //0x24
    None,                                                                    //0x25
    None,                                                                    //0x26
    None,                                                                    //0x27
    None,                                                                    //0x28
    None,                                                                    //0x29
    None,                                                                    //0x2a
    None,                                                                    //0x2b
    None,                                                                    //0x2c
    None,                                                                    //0x2d
    None,                                                                    //0x2e
    None,                                                                    //0x2f
    Some((ADDRESS, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x30
    Some((BALANCE, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x31
    Some((ORIGIN, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x32
    Some((CALLER, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x33
    Some((CALLVALUE, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x34
    Some((CALLDATALOAD, OpcodeInfo { category: "system", gas: 3, static_gas: true })), //0x35
    Some((CALLDATASIZE, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x36
    Some((CALLDATACOPY, OpcodeInfo { category: "system", gas: 3, static_gas: false })), //0x37
    Some((CODESIZE, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x38
    Some((CODECOPY, OpcodeInfo { category: "system", gas: 3, static_gas: false })), //0x39
    Some((GASPRICE, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x3a
    Some((EXTCODESIZE, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x3b
    Some((EXTCODECOPY, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x3c
    Some((RETURNDATASIZE, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x3d
    Some((RETURNDATACOPY, OpcodeInfo { category: "system", gas: 3, static_gas: false })), //0x3e
    Some((EXTCODEHASH, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x3f
    Some((BLOCKHASH, OpcodeInfo { category: "host", gas: 20, static_gas: true })), //0x40
    Some((COINBASE, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x41
    Some((TIMESTAMP, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x42
    Some((NUMBER, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x43
    Some((DIFFICULTY, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x44
    Some((GASLIMIT, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x45
    Some((CHAINID, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x46
    Some((SELFBALANCE, OpcodeInfo { category: "host", gas: 5, static_gas: true })), //0x47
    Some((BASEFEE, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x48
    Some((BLOBHASH, OpcodeInfo { category: "host_env", gas: 3, static_gas: true })), //0x49
    Some((BLOBBASEFEE, OpcodeInfo { category: "host_env", gas: 2, static_gas: true })), //0x4a
    None,                                                                    //0x4b
    None,                                                                    //0x4c
    None,                                                                    //0x4d
    None,                                                                    //0x4e
    None,                                                                    //0x4f
    Some((POP, OpcodeInfo { category: "stack::pop", gas: 2, static_gas: true })), //0x50
    Some((MLOAD, OpcodeInfo { category: "memory", gas: 3, static_gas: true })), //0x51
    Some((MSTORE, OpcodeInfo { category: "memory", gas: 3, static_gas: true })), //0x52
    Some((MSTORE8, OpcodeInfo { category: "memory", gas: 3, static_gas: true })), //0x53
    Some((SLOAD, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x54
    Some((SSTORE, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0x55
    Some((JUMP, OpcodeInfo { category: "control", gas: 8, static_gas: true })), //0x56
    Some((JUMPI, OpcodeInfo { category: "control", gas: 10, static_gas: true })), //0x57
    Some((PC, OpcodeInfo { category: "control", gas: 2, static_gas: true })), //0x58
    Some((MSIZE, OpcodeInfo { category: "memory", gas: 2, static_gas: true })), //0x59
    Some((GAS, OpcodeInfo { category: "system", gas: 2, static_gas: true })), //0x5a
    Some((JUMPDEST, OpcodeInfo { category: "control", gas: 1, static_gas: true })), //0x5b
    Some((TLOAD, OpcodeInfo { category: "host", gas: 100, static_gas: true })), //0x5c
    Some((TSTORE, OpcodeInfo { category: "host", gas: 100, static_gas: true })), //0x5d
    Some((MCOPY, OpcodeInfo { category: "memory", gas: 100, static_gas: false })), //0x5e
    Some((PUSH0, OpcodeInfo { category: "stack::push", gas: 2, static_gas: true })), //0x5f
    Some((PUSH1, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x60
    Some((PUSH2, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x61
    Some((PUSH3, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x62
    Some((PUSH4, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x63
    Some((PUSH5, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x64
    Some((PUSH6, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x65
    Some((PUSH7, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x66
    Some((PUSH8, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x67
    Some((PUSH9, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x68
    Some((PUSH10, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x69
    Some((PUSH11, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6a
    Some((PUSH12, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6b
    Some((PUSH13, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6c
    Some((PUSH14, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6d
    Some((PUSH15, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6e
    Some((PUSH16, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x6f
    Some((PUSH17, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x70
    Some((PUSH18, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x71
    Some((PUSH19, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x72
    Some((PUSH20, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x73
    Some((PUSH21, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x74
    Some((PUSH22, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x75
    Some((PUSH23, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x76
    Some((PUSH24, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x77
    Some((PUSH25, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x78
    Some((PUSH26, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x79
    Some((PUSH27, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7a
    Some((PUSH28, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7b
    Some((PUSH29, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7c
    Some((PUSH30, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7d
    Some((PUSH31, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7e
    Some((PUSH32, OpcodeInfo { category: "stack::push", gas: 3, static_gas: true })), //0x7f
    Some((DUP1, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x80
    Some((DUP2, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x81
    Some((DUP3, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x82
    Some((DUP4, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x83
    Some((DUP5, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x84
    Some((DUP6, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x85
    Some((DUP7, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x86
    Some((DUP8, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x87
    Some((DUP9, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x88
    Some((DUP10, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x89
    Some((DUP11, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8a
    Some((DUP12, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8b
    Some((DUP13, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8c
    Some((DUP14, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8d
    Some((DUP15, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8e
    Some((DUP16, OpcodeInfo { category: "stack::dup", gas: 3, static_gas: true })), //0x8f
    Some((SWAP1, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x90
    Some((SWAP2, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x91
    Some((SWAP3, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x92
    Some((SWAP4, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x93
    Some((SWAP5, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x94
    Some((SWAP6, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x95
    Some((SWAP7, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x96
    Some((SWAP8, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x97
    Some((SWAP9, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x98
    Some((SWAP10, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x99
    Some((SWAP11, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9a
    Some((SWAP12, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9b
    Some((SWAP13, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9c
    Some((SWAP14, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9d
    Some((SWAP15, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9e
    Some((SWAP16, OpcodeInfo { category: "stack::swap", gas: 3, static_gas: true })), //0x9f
    Some((LOG0, OpcodeInfo { category: "host", gas: 375, static_gas: false })), //0xa0
    Some((LOG1, OpcodeInfo { category: "host", gas: 750, static_gas: false })), //0xa1
    Some((LOG2, OpcodeInfo { category: "host", gas: 1125, static_gas: false })), //0xa2
    Some((LOG3, OpcodeInfo { category: "host", gas: 1500, static_gas: false })), //0xa3
    Some((LOG4, OpcodeInfo { category: "host", gas: 1875, static_gas: false })), //0xa4
    None,                                                                    //0xa5
    None,                                                                    //0xa6
    None,                                                                    //0xa7
    None,                                                                    //0xa8
    None,                                                                    //0xa9
    None,                                                                    //0xaa
    None,                                                                    //0xab
    None,                                                                    //0xac
    None,                                                                    //0xad
    None,                                                                    //0xae
    None,                                                                    //0xaf
    None,                                                                    //0xb0
    None,                                                                    //0xb1
    None,                                                                    //0xb2
    None,                                                                    //0xb3
    None,                                                                    //0xb4
    None,                                                                    //0xb5
    None,                                                                    //0xb6
    None,                                                                    //0xb7
    None,                                                                    //0xb8
    None,                                                                    //0xb9
    None,                                                                    //0xba
    None,                                                                    //0xbb
    None,                                                                    //0xbc
    None,                                                                    //0xbd
    None,                                                                    //0xbe
    None,                                                                    //0xbf
    None,                                                                    //0xc0
    None,                                                                    //0xc1
    None,                                                                    //0xc2
    None,                                                                    //0xc3
    None,                                                                    //0xc4
    None,                                                                    //0xc5
    None,                                                                    //0xc6
    None,                                                                    //0xc7
    None,                                                                    //0xc8
    None,                                                                    //0xc9
    None,                                                                    //0xca
    None,                                                                    //0xcb
    None,                                                                    //0xcc
    None,                                                                    //0xcd
    None,                                                                    //0xce
    None,                                                                    //0xcf
    None,                                                                    //0xd0
    None,                                                                    //0xd1
    None,                                                                    //0xd2
    None,                                                                    //0xd3
    None,                                                                    //0xd4
    None,                                                                    //0xd5
    None,                                                                    //0xd6
    None,                                                                    //0xd7
    None,                                                                    //0xd8
    None,                                                                    //0xd9
    None,                                                                    //0xda
    None,                                                                    //0xdb
    None,                                                                    //0xdc
    None,                                                                    //0xdd
    None,                                                                    //0xde
    None,                                                                    //0xdf
    None,                                                                    //0xe0
    None,                                                                    //0xe1
    None,                                                                    //0xe2
    None,                                                                    //0xe3
    None,                                                                    //0xe4
    None,                                                                    //0xe5
    None,                                                                    //0xe6
    None,                                                                    //0xe7
    None,                                                                    //0xe8
    None,                                                                    //0xe9
    None,                                                                    //0xea
    None,                                                                    //0xeb
    None,                                                                    //0xec
    None,                                                                    //0xed
    None,                                                                    //0xee
    None,                                                                    //0xef
    Some((CREATE, OpcodeInfo { category: "host", gas: 32000, static_gas: false })), //0xf0
    Some((CALL, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0xf1
    Some((CALLCODE, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0xf2
    Some((RETURN, OpcodeInfo { category: "control", gas: 0, static_gas: true })), //0xf3
    Some((DELEGATECALL, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0xf4
    Some((CREATE2, OpcodeInfo { category: "host", gas: 32000, static_gas: false })), //0xf5
    None,                                                                    //0xf6
    None,                                                                    //0xf7
    None,                                                                    //0xf8
    None,                                                                    //0xf9
    Some((STATICCALL, OpcodeInfo { category: "host", gas: 100, static_gas: false })), //0xfa
    None,                                                                    //0xfb
    None,                                                                    //0xfc
    Some((REVERT, OpcodeInfo { category: "control", gas: 0, static_gas: true })), //0xfd
    Some((INVALID, OpcodeInfo { category: "invalid", gas: 0, static_gas: true })), //0xfe
    Some((SELFDESTRUCT, OpcodeInfo { category: "host", gas: 5000, static_gas: false })), //0xff
];

use super::commons::*;
use revm_utils::time_utils::convert_cycles_to_ns_f64;
use std::collections::BTreeMap;
const MGAS_TO_GAS: u64 = 1_000_000u64;

const COL_WIDTH: usize = 15;
#[derive(Default, Debug)]
struct OpcodeMergeRecord {
    count: u64,
    count_pct: f64,
    time: u64,
    time_pct: f64,
    avg_cost: f64,
}

#[derive(Default, Debug)]
struct OpcodeStat {
    count: u64,
    count_pct: f64,
    time: u64,
    time_pct: f64,
    avg_cost: f64,
    mgas: f64,
    mgas_pct: f64,
    static_gas: Option<u64>,
    dyn_gas: Option<f64>,
    cat: Option<&'static str>,
}

impl OpcodeStat {
    fn print(&self, opcode: &str) {
        let static_gas = match self.static_gas {
            None => "NAN".to_string(),
            Some(s) => s.to_string(),
        };

        let dyn_gas = match self.dyn_gas {
            None => "".to_string(),
            Some(gas) => {
                if gas > 0.01 {
                    format!("{:.2}", gas).to_string()
                } else {
                    "".to_string()
                }
            }
        };

        println!(
            "{: <COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$.3}{:>COL_WIDTH$.2}{:>COL_WIDTH$.3} \
            {:>COL_WIDTH$.1}{:>COL_WIDTH$.2}{:>COL_WIDTH$.2}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}",
            opcode,
            self.count,
            self.count_pct * 100.0,
            cycles_as_secs(self.time),
            self.time_pct * 100.0,
            self.avg_cost,
            self.mgas,
            self.mgas_pct * 100.0,
            static_gas,
            dyn_gas,
            self.cat.unwrap_or("NAN"),
        );
    }
}

#[derive(Debug)]
struct OpcodeStats {
    overall: OpcodeStat,
    opcode: [Option<OpcodeStat>; OPCODE_NUMBER],
    merge_records: BTreeMap<&'static str, OpcodeMergeRecord>,
}

const ARRAY_REPEAT_VALUE: std::option::Option<OpcodeStat> = None;
impl Default for OpcodeStats {
    fn default() -> Self {
        Self {
            overall: OpcodeStat::default(),
            opcode: [ARRAY_REPEAT_VALUE; OPCODE_NUMBER],
            merge_records: BTreeMap::new(),
        }
    }
}

impl OpcodeStats {
    fn print_opcode_title(&self) {
        println!("===========================================================================Metric of instruction======================================================================");
        println!(
            "{: <COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$} \
            {:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}",
            "Opcode",
            "Count",
            "Count (%)",
            "Time (s)",
            "Time (%)",
            "Cost (ns)",
            "Total Mgas",
            "Gas (%)",
            "Static gas",
            "Dyn. gas",
            "Category"
        );
    }

    fn print_opcodes(&self) {
        println!();
        self.print_opcode_title();
        self.overall.print("overall");
        for i in 0..OPCODE_NUMBER {
            let name = OpCode::new(i as u8);
            if name.is_none() {
                continue
            }
            self.opcode[i]
                .as_ref()
                .expect("opcode record should not empty")
                .print(name.unwrap().as_str());
        }
        println!();
    }

    fn print_category(&self) {
        println!("\n");
        println!("==========================================================================================");
        println!(
            "{:<COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$}",
            "Opcode Cat.", "Count", "Count (%)", "Time (s)", "Time (%)", "Cost (ns)",
        );

        for (k, v) in self.merge_records.iter() {
            if *k == "" {
                continue
            }
            println!(
                "{:<COL_WIDTH$}{:>COL_WIDTH$}{:>COL_WIDTH$.2}{:>COL_WIDTH$.1}{:>COL_WIDTH$.3}{:>COL_WIDTH$.3}",
                *k,
                v.count,
                v.count_pct * 100.0,
                cycles_as_secs(v.time),
                v.time_pct * 100.0,
                v.avg_cost,
            );
        }
    }
}

trait SupportPrint {
    fn stats(&self) -> OpcodeStats;
    fn print_addition_count(&self);
    fn print_sload_percentile(&self);
}

// Return (total_gas, static_gas, dyn_gas).
fn caculate_gas(opcode: u8, count: u64, total_gas: i128) -> (f64, u64, f64) {
    let (base_gas, is_static) = match MERGE_MAP[opcode as usize] {
        Some(opcode_info) => (opcode_info.1.gas, opcode_info.1.static_gas),
        None => return (0.0, 0, 0.0),
    };

    let total_static_gas = base_gas.checked_mul(count).unwrap_or(0);
    if is_static {
        return (total_static_gas as f64, base_gas, 0.0)
    }

    let dyn_gas = if total_gas > total_static_gas as i128 {
        (total_gas - total_static_gas as i128) as f64 / count as f64
    } else {
        0.0
    };

    (total_gas as f64, base_gas, dyn_gas)
}

fn category_name(opcode: u8) -> Option<&'static str> {
    Some(MERGE_MAP[opcode as usize]?.1.category)
}

impl SupportPrint for OpcodeRecord {
    fn stats(&self) -> OpcodeStats {
        let mut opcode_stats = OpcodeStats::default();
        // induction
        for (i, v) in self.opcode_record.iter().enumerate() {
            // opcode
            let op = i as u8;
            let mut opcode_stat = OpcodeStat::default();
            opcode_stat.count = v.0;
            opcode_stat.time = v.1;
            opcode_stat.avg_cost = convert_cycles_to_ns_f64(v.1) / v.0 as f64;
            let (op_total, op_static, op_dyn) = caculate_gas(op, v.0, v.2);
            opcode_stat.mgas = op_total / MGAS_TO_GAS as f64;
            opcode_stat.static_gas = Some(op_static);
            opcode_stat.dyn_gas = Some(op_dyn);

            let cat = match category_name(i as u8) {
                Some(name) => name,
                None => "",
            };
            opcode_stat.cat = Some(cat);
            opcode_stats.opcode[i] = Some(opcode_stat);

            // overall
            opcode_stats.overall.count =
                opcode_stats.overall.count.checked_add(v.0).expect("overflow");
            opcode_stats.overall.time =
                opcode_stats.overall.time.checked_add(v.1).expect("overflow");
            opcode_stats.overall.mgas += opcode_stats.opcode[i].as_ref().expect("empty").mgas;

            // merge
            opcode_stats
                .merge_records
                .entry(cat)
                .and_modify(|r| {
                    r.count += v.0;
                    r.time += v.1;
                })
                .or_insert(OpcodeMergeRecord {
                    count: v.0,
                    count_pct: 0.0,
                    time: v.1,
                    time_pct: 0.0,
                    avg_cost: 0.0,
                });
        }

        // calculate opcode pct
        for (i, _v) in self.opcode_record.iter().enumerate() {
            opcode_stats.opcode[i].as_mut().expect("empty").count_pct =
                opcode_stats.opcode[i].as_mut().expect("empty").count as f64 /
                    opcode_stats.overall.count as f64;
            opcode_stats.opcode[i].as_mut().expect("empty").time_pct =
                opcode_stats.opcode[i].as_mut().expect("empty").time as f64 /
                    opcode_stats.overall.time as f64;
            opcode_stats.opcode[i].as_mut().expect("empty").mgas_pct =
                opcode_stats.opcode[i].as_mut().expect("empty").mgas as f64 /
                    opcode_stats.overall.mgas as f64;
        }
        opcode_stats.overall.count_pct = 1.0;
        opcode_stats.overall.time_pct = 1.0;
        opcode_stats.overall.mgas_pct = 1.0;
        opcode_stats.overall.avg_cost =
            convert_cycles_to_ns_f64(opcode_stats.overall.time) / opcode_stats.overall.count as f64;

        // calculate merge opcode pct
        for (_, value) in opcode_stats.merge_records.iter_mut() {
            value.count_pct = value.count as f64 / opcode_stats.overall.count as f64;
            value.time_pct = value.time as f64 / opcode_stats.overall.time as f64;
            value.avg_cost = convert_cycles_to_ns_f64(value.time) / value.count as f64;
        }

        opcode_stats
    }

    fn print_addition_count(&self) {
        println!();
        println!("call additional rdtsc count: {}", self.additional_count[0]);
        println!("call_code additional rdtsc count: {}", self.additional_count[1]);
        println!("delegate_call additional rdtsc count: {}", self.additional_count[2]);
        println!("static_call additional rdtsc count: {}", self.additional_count[3]);
        println!();
    }

    fn print_sload_percentile(&self) {
        println!("=============sload time percentile=============");
        self.sload_percentile.print_content();
    }
}

impl Print for OpcodeRecord {
    fn print(&self, _block_number: u64) {
        let opcode_stats = self.stats();
        opcode_stats.print_opcodes();
        opcode_stats.print_category();
        self.print_addition_count();
        self.print_sload_percentile();
    }
}
