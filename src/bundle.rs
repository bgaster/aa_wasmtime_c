use anyhow::{Result, anyhow};
use serde::{Deserialize};

//use crate::utils::*;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    VInt(i32),
    VFloat(f32),
    VString(String),
    VPair((u8,u8)),
    VVU8(Vec<u8>),
}

impl From<Value> for i32 {
    fn from(v: Value) -> Self {
        match v {
            Value::VFloat(f) => f as i32,
            Value::VInt(i) => i,
            _ => 0,
        }
    } 
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::VFloat(f) => f.to_string(),
            Self::VInt(i) => i.to_string(),
            Self::VString(s) => s.clone(),
            Self::VPair((x,y)) => {
                "[".to_string() + &x.to_string() + "," + &y.to_string() + "]"
            }
            Self::VVU8(v) => {
                let mut s = "[".to_string();
                for (i, u) in v.iter().enumerate() {
                    s = s + &((*u) as i32).to_string();
                    if i + 1 != v.len() {
                        s.push(',')
                    }
                }
                s.push(']');
                s
            },
        }
    }
}

//pub type Index = u32;


#[derive(Deserialize, Debug, Clone)]
pub struct GUIBundle {
    pub url: String,
    pub name: String,
    pub params: Vec<Vec<Value>>,
    pub width: i32,
    pub height: i32, 
}

#[derive(Deserialize, Debug, Clone)]
pub struct Info {
    pub name: String,
    pub vendor: String,
    pub presets: u32,
    pub parameters: u32,
    pub inputs: i32,
    pub outputs: i32,
    pub midi_inputs: u32,
    pub midi_outputs: u32,
    pub id: u32,
    pub version: u32,
    pub category: String,
    pub initial_delay: u32,
    pub preset_chunks: bool,
    pub f64_precision: bool,
    pub silent_when_stopped: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bundle {
    pub wasm_url: Vec<String>,
    pub gui: GUIBundle,
    pub gui_description: Option<String>,
    pub info: Info,
}

impl Bundle {
    pub fn from_json(data: &str) -> Result<Self> {
        let bundle : serde_json::Result<Bundle> = serde_json::from_str(data);
        //println!("{:?}", bundle);
        bundle.map_or(Err(anyhow!("")), |b| Ok(b))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub json_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Modules {
    pub default: String,
    pub modules: Vec<Module>,
}

impl Modules {
    pub fn _from_json(data: &str) -> Result<Self> {
        let modules : serde_json::Result<Modules> = serde_json::from_str(data);
        modules.map_or(Err(anyhow!("")), |b| Ok(b))
    }
}