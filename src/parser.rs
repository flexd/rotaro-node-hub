use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum OpCode {
    Handshake = 101,
    Racetime = 102,
    Timesync = 103,
    GetChannel = 104,
    SetChannel = 105,
    RunTimer = 106,
    Calibrate = 107,
    PingPong = 108,
    SetName = 109,
    GetName = 110,
    SetRSSI = 111,
    GetRSSI = 112,
    Invalid = 0,
}
impl From<u8> for OpCode {
    fn from(opcode: u8) -> OpCode {
        match opcode {
            101 => OpCode::Handshake,
            102 => OpCode::Racetime,
            103 => OpCode::Timesync,
            104 => OpCode::GetChannel,
            105 => OpCode::SetChannel,
            106 => OpCode::RunTimer,
            107 => OpCode::Calibrate,
            108 => OpCode::PingPong,
            109 => OpCode::SetName,
            110 => OpCode::GetName,
            111 => OpCode::SetRSSI,
            112 => OpCode::GetRSSI,
            _ => OpCode::Invalid,
        }
    }
}
impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> u8 {
        match opcode {
            OpCode::Handshake => 101,
            OpCode::Racetime => 102,
            OpCode::Timesync => 103,
            OpCode::GetChannel => 104,
            OpCode::SetChannel => 105,
            OpCode::RunTimer => 106,
            OpCode::Calibrate => 107,
            OpCode::PingPong => 108,
            OpCode::SetName => 109,
            OpCode::GetName => 110,
            OpCode::SetRSSI => 111,
            OpCode::GetRSSI => 112,
            OpCode::Invalid => 0,
        }
    }
}
pub fn parse_operation(input: &[u8]) -> OpCode {
    OpCode::from(input[0])
}
#[derive(Serialize,Deserialize)]
pub enum Operations {
        Handshake(Handshake) ,
}
#[derive(Serialize,Deserialize, Debug)]
pub struct Handshake {
    pub id: u8,
    pub version: u8,
    pub type: u8,
    pub mac: [u8; 12],
    pub name: [u8; 100],
    pub channel: u8,
    pub calib_threshold: u16,
    pub enter_offset: u16,
    pub exit_offset: u16,
}
#[derive(Serialize,Deserialize, Debug)]
pub struct Timesync {
    pub id: u8,
    pub bounce_count: u8,
    pub hub_time: u32,
}
