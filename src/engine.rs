use crate::Args;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};

pub enum JsonValue {
    Object { kvp: Vec<(String, JsonValue)> },
    Array { elements: Vec<JsonValue> },
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

struct JsonParser {
    data: Vec<u8>,
    idx: usize,
}

pub fn engine_main(args: Args) -> Result<()> {
    let json_name = format!("{}.json", args.file_name);
    let bin_name = format!("{}.bin", args.file_name);
    let start = std::time::Instant::now();
    let results_file = File::open(bin_name)?;
    let mut buf_reader = std::io::BufReader::new(results_file);
    let mut expected_results = Vec::with_capacity(args.count as usize);
    let mut buffer = [0u8; 8];
    loop {
        if buf_reader.read_exact(&mut buffer).is_err() {
            break;
        }
        let val = f64::from_le_bytes(buffer);
        expected_results.push(val);
    }
    println!("read res time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let mut file = File::open(json_name)?;
    let mut data = Vec::with_capacity(1024 * 1024 * 128);
    file.read_to_end(&mut data)?;
    println!("read json time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let mut parser = JsonParser { data, idx: 0 };
    let val = parser.parse_data()?;
    println!("parse to IL time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let results = parse_pairs_from_json(val)?;
    println!("parse to pairs time: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    calculate_haversine_for_pairs(&results, &expected_results);
    println!("calc time: {:?}", start.elapsed());
    Ok(())
}

const WS_RGX: [u8; 4] = [b' ', b'\n', b'\r', b'\t'];

impl JsonParser {
    pub(crate) fn parse_data(&mut self) -> Result<JsonValue> {
        self.ignore_whitespace();
        match self.data[self.idx] {
            b'{' => {
                self.idx += 1;
                let mut key_val_pairs = Vec::new();
                loop {
                    if self.data[self.idx] == b',' {
                        self.idx += 1;
                    }
                    if self.data[self.idx] == b'}' {
                        self.idx += 1;
                        break;
                    }
                    self.ignore_whitespace();
                    let key = self.read_str()?;
                    self.ignore_whitespace();
                    if self.data[self.idx] != b':' {
                        Err(Error::new(ErrorKind::InvalidData, "unexpected token"))?;
                    }
                    self.idx += 1;
                    let val = self.parse_data()?;
                    self.ignore_whitespace();
                    key_val_pairs.push((key, val));
                }
                Ok(JsonValue::Object { kvp: key_val_pairs })
            }
            b'[' => {
                self.idx += 1;
                let mut elements = Vec::new();
                loop {
                    self.ignore_whitespace();
                    if self.data[self.idx] == b',' {
                        self.idx += 1;
                    }
                    if self.data[self.idx] == b']' {
                        self.idx += 1;
                        break;
                    }
                    self.ignore_whitespace();
                    let val = self.parse_data()?;
                    self.ignore_whitespace();
                    elements.push(val);
                }
                Ok(JsonValue::Array { elements })
            }
            b'"' => Ok(JsonValue::String(self.read_str()?)),
            b'-' | b'0'..=b'9' => {
                let start = self.idx;
                let mut end = start + 1;
                if self.data[end] == b'-' {
                    end += 1;
                }
                while self.data[end] >= b'0' && self.data[end] <= b'9' || self.data[end] == b'.' {
                    end += 1;
                }

                self.idx = end;
                let m = self.data[start..end].to_vec();
                let x = String::from_utf8(m).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                Ok(JsonValue::Number(
                    x.parse()
                        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
                ))
            }
            b't' => {
                self.idx += 4;
                Ok(JsonValue::Boolean(true))
            }
            b'f' => {
                self.idx += 5;
                Ok(JsonValue::Boolean(false))
            }
            b'n' => {
                self.idx += 4;
                Ok(JsonValue::Null)
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "unexpected.")),
        }
    }

    fn ignore_whitespace(&mut self) -> () {
        while WS_RGX.contains(&self.data[self.idx]) {
            self.idx += 1;
        }
    }

    fn read_str(&mut self) -> Result<String> {
        let start = self.idx + 1;
        let mut end = start;
        while self.data[end] != b'"' && self.data[end - 1] != b'\\' {
            end += 1;
        }
        self.idx = end + 1;
        String::from_utf8(self.data[start..end].to_vec())
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}

fn parse_pairs_from_json(val: JsonValue) -> Result<Vec<Pairs>> {
    match val {
        JsonValue::Object { kvp: key_val_pairs } => {
            let i = 0;
            if key_val_pairs[i].0.eq("pairs") {
                if let JsonValue::Array { elements } = &key_val_pairs[i].1 {
                    let mut results: Vec<Pairs> = Vec::new();
                    for element in elements {
                        results.push(construct(element)?);
                    }
                    return Ok(results);
                }
            }
            Err(Error::new(ErrorKind::InvalidData, "No key pairs"))
        }
        _ => Err(Error::new(ErrorKind::InvalidData, "unexpected.")),
    }
}

fn calculate_haversine_for_pairs(results: &Vec<Pairs>, x: &Vec<f64>) {
    let len = results.len();
    if x.len() != len {
        println!("lengths not equal");
        return;
    }
    let mut sum = 0f64;
    for i in 0..len {
        let pair = &results[i];
        let h = haversine(pair.x0, pair.x1, pair.y0, pair.y1);
        sum += h;
        if h != x[i] {
            println!("{}: {} != {}", i, h, x[i]);
        }
    }
    println!("count: {}", len);
    println!("avg: {}", sum / len as f64);
}

fn construct(val: &JsonValue) -> Result<Pairs> {
    match val {
        JsonValue::Object { kvp: key_val_pairs } => Ok(Pairs::new(key_val_pairs)?),
        _ => Err(Error::new(ErrorKind::InvalidData, "unexpected.")),
    }
}

struct Pairs {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

impl Pairs {
    fn new(kvp: &Vec<(String, JsonValue)>) -> Result<Self> {
        let x0 = kvp.read_double("x0")?;
        let x1 = kvp.read_double("x1")?;
        let y0 = kvp.read_double("y0")?;
        let y1 = kvp.read_double("y1")?;
        Ok(Pairs { x0, x1, y0, y1 })
    }
}

trait MapThing {
    fn read_double(&self, key: &str) -> Result<f64>;
}

impl MapThing for Vec<(String, JsonValue)> {
    fn read_double(&self, key: &str) -> Result<f64> {
        for pair in self {
            if key.eq(pair.0.as_str()) {
                if let JsonValue::Number(f) = pair.1 {
                    return Ok(f.clone());
                }
            }
        }
        Err(Error::new(ErrorKind::NotFound, "no key"))
    }
}

fn haversine(x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let d_lat = f64::to_degrees(y1 - y0);
    let d_lon = f64::to_degrees(x1 - x0);
    let lat1 = f64::to_degrees(y0);
    let lat2 = f64::to_degrees(y1);

    let a = ((d_lat / 2.0f64).sin().powi(2))
        + lat1.cos() * lat2.cos() * ((d_lon / 2.0f64).sin().powi(2));
    let c = 2.0f64 * (a.sqrt().asin());
    6372.8f64 * c
}
