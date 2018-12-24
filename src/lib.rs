
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str;

//#[macro_use]
//extern crate lazy_static;
//

//
//lazy_static! {
//    pub static ref SENSORS_MAP: HashMap<u16, SensorDefinition> = {
//        let mut map = HashMap::new();
//
//        let mut met_labels = HashMap::new();
//        met_labels.insert(String::from("location"), String::from("ws1"));
//        map.insert(1u16, SensorDefinition{
//            id: 1,
//            name: String::from("ws_1_batt"),
//            format: SensorFormat::Bool,
//            destination_queues: vec![String::from("/ws/1/batt")],
//            metric_name: String::from("ws_1_batt"),
//            metric_type: MetricType::Gauge,
//            metric_labels: met_labels,
//        });
//
//
//        map
//    };
//}



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SensorFormat {
    Bool,
    TwoByteFloat,   //Limited in range from -128.99 to 127.99
    SingleByteInt,  //Limited in range from 0 to 255
    TwoByteInt,     //Limited in range from 0 to 65535
    FourByteInt,    //Limited in range from 0 to 4294967295
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    Gauge,
    Counter,
    Histogram,
    Summary,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorDefinition {
    pub id: i16,
    pub name: String,
    pub format: SensorFormat,
    pub destination_queues: Vec<String>,
    pub metric_name: String,
    pub metric_type: MetricType,
    pub metric_labels: HashMap<String, String>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorValue {
    pub id: i16,
    pub timestamp: u64,
    pub value: String,
}

//Most of the i32's are actually 16bit unsigned but postgres doesn't have an unsigned so we are forced to use i32
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LightValue {
    pub timestamp: u64,
    pub location: i16,
    pub uv_raw: i32,
    pub uv_index: f32,
    pub vis_raw: i32,
    pub ir_raw: i32,
    pub lux: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TempHumidityValue {
    pub timestamp: u64,
    pub location: i16,
    pub temp: f32,
    pub humidity: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WindSpeedDirValue{
    pub timestamp: u64,
    pub location: i16,
    pub speed: i16,
    pub dir: i16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AirParticulateValue{
    pub timestamp: u64,
    pub location: i16,
    pub pm2_5: i16,
    pub pm10: i16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ElectricValue {
    pub timestamp: u64,
    pub location: i16,
    pub total_kwh: f64,
    pub total_reactive: f64,
    pub total_reverse: f64,
    pub volts_l1: f32,
    pub volts_l2: f32,
    pub amps_l1: f32,
    pub amps_l2: f32,
    pub watts_l1: i16,
    pub watts_l2: i16,
    pub watts_total: i16,
    pub pf_l1: f32,
    pub pf_l2: f32,
    pub reactive_l1: i16,
    pub reactive_l2: i16,
    pub reactive_total: i16,
    pub frequency: f32,
}


#[derive(Debug)]
pub struct ElectricValueParseError {
    element: String,
    message: String,
    data: String,
}

impl std::fmt::Display for ElectricValueParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed parsing element '{}', with error '{}', and data '{}'",  self.element, self.message, self.data)
    }
}

impl ElectricValue {
    pub fn new(timestamp: u64, location: i16, payload: &[u8;255]) -> Result<ElectricValue, ElectricValueParseError> {

        let kwh_scale: u8 = str::from_utf8(&payload[230..231])
            .map_err(|err| ElectricValueParseError{
                element: String::from("kwh_scale"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("kwh_scale"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let total_kwh = str::from_utf8(&payload[16..24])
            .map_err(|err| ElectricValueParseError{
                element: String::from("total_kwh"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("total_kwh"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f64| {
                if kwh_scale == 0 {
                    Ok(val_as_flt)
                } else if kwh_scale == 1 {
                    Ok(val_as_flt/10.0)
                } else if kwh_scale == 2 {
                    Ok(val_as_flt/100.0)
                } else {
                    Err(ElectricValueParseError{
                        element: String::from("total_kwh"),
                        message: format!("Unexpected kwh_scale: {}", kwh_scale),
                        data: String::new(),
                    })
                }
            })?;

        let total_reactive = str::from_utf8(&payload[24..32])
            .map_err(|err| ElectricValueParseError{
                element: String::from("total_reactive"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("total_reactive"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f64| {
                if kwh_scale == 0 {
                    Ok(val_as_flt)
                } else if kwh_scale == 1 {
                    Ok(val_as_flt/10.0)
                } else if kwh_scale == 2 {
                    Ok(val_as_flt/100.0)
                } else {
                    Err(ElectricValueParseError{
                        element: String::from("total_reactive"),
                        message: format!("Unexpected kwh_scale: {}", kwh_scale),
                        data: String::new(),
                    })
                }
            })?;

        let total_reverse = str::from_utf8(&payload[32..40])
            .map_err(|err| ElectricValueParseError{
                element: String::from("total_reverse"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("total_reverse"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f64| {
                if kwh_scale == 0 {
                    Ok(val_as_flt)
                } else if kwh_scale == 1 {
                    Ok(val_as_flt/10.0)
                } else if kwh_scale == 2 {
                    Ok(val_as_flt/100.0)
                } else {
                    Err(ElectricValueParseError{
                        element: String::from("total_reverse"),
                        message: format!("Unexpected kwh_scale: {}", kwh_scale),
                        data: String::new(),
                    })
                }
            })?;

        let volts_l1 = str::from_utf8(&payload[104..108])
            .map_err(|err| ElectricValueParseError{
                element: String::from("volts_l1"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("volts_l1"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| Ok(val_as_flt/10.0))?;

        let volts_l2 = str::from_utf8(&payload[108..112])
            .map_err(|err| ElectricValueParseError{
                element: String::from("volts_l2"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("volts_l2"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| Ok(val_as_flt/10.0))?;

        let amps_l1 = str::from_utf8(&payload[116..121])
            .map_err(|err| ElectricValueParseError{
                element: String::from("amps_l1"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("amps_l1"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| Ok(val_as_flt/10.0))?;

        let amps_l2 = str::from_utf8(&payload[121..126])
            .map_err(|err| ElectricValueParseError{
                element: String::from("amps_l2"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("amps_l2"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| Ok(val_as_flt/10.0))?;

        let watts_l1 = str::from_utf8(&payload[131..138])
            .map_err(|err| ElectricValueParseError{
                element: String::from("watts_l1"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("watts_l1"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let watts_l2 = str::from_utf8(&payload[138..145])
            .map_err(|err| ElectricValueParseError{
                element: String::from("watts_l2"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("watts_l2"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let watts_total = str::from_utf8(&payload[152..159])
            .map_err(|err| ElectricValueParseError{
                element: String::from("watts_total"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("watts_total"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let pf_l1_dir = str::from_utf8(&payload[159..160])
            .map_err(|err| ElectricValueParseError{
                element: String::from("pf_l1_dir"),
                message: format!("{}", err),
                data: String::new(),
            })?;

        let pf_l1 = str::from_utf8(&payload[160..163])
            .map_err(|err| ElectricValueParseError{
                element: String::from("pf_l1"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.trim().parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("pf_l1"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| {
                let ret_val = val_as_flt/100.0;
                if pf_l1_dir == "L" {
                    Ok(ret_val)
                } else if pf_l1_dir == "C" {
                    Ok(1.0 + (1.0-ret_val))
                } else {
                    Ok(ret_val)
                }
            })?;

        let pf_l2_dir = str::from_utf8(&payload[163..164])
            .map_err(|err| ElectricValueParseError{
                element: String::from("pf_l2_dir"),
                message: format!("{}", err),
                data: String::new(),
            })?;

        let pf_l2 = str::from_utf8(&payload[164..167])
            .map_err(|err| ElectricValueParseError{
                element: String::from("pf_l2"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.trim().parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("pf_l2"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| {
                let ret_val = val_as_flt/100.0;
                if pf_l2_dir == "L" {
                    Ok(ret_val)
                } else if pf_l2_dir == "C" {
                    Ok(1.0 + (1.0-ret_val))
                } else {
                    Ok(ret_val)
                }
            })?;

        let reactive_l1 = str::from_utf8(&payload[171..178])
            .map_err(|err| ElectricValueParseError{
                element: String::from("reactive_l1"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("reactive_l1"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let reactive_l2 = str::from_utf8(&payload[178..185])
            .map_err(|err| ElectricValueParseError{
                element: String::from("reactive_l2"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("reactive_l2"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let reactive_total = str::from_utf8(&payload[192..199])
            .map_err(|err| ElectricValueParseError{
                element: String::from("reactive_total"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("reactive_total"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))?;

        let frequency = str::from_utf8(&payload[199..203])
            .map_err(|err| ElectricValueParseError{
                element: String::from("frequency"),
                message: format!("{}", err),
                data: String::new(),
            })
            .and_then( |val_as_str| val_as_str.parse().map_err(|err|{
                ElectricValueParseError{
                    element: String::from("frequency"),
                    message: format!("{}", err),
                    data: String::from(val_as_str),
                }
            }))
            .and_then(|val_as_flt: f32| Ok(val_as_flt/100.0))?;


        Ok(ElectricValue{
            timestamp,
            location,
            total_kwh,
            total_reactive,
            total_reverse,
            volts_l1,
            volts_l2,
            amps_l1,
            amps_l2,
            watts_l1,
            watts_l2,
            watts_total,
            pf_l1,
            pf_l2,
            reactive_l1,
            reactive_l2,
            reactive_total,
            frequency,
        })
    }
}

pub fn load_from_file(yaml_file: &str) -> HashMap<i16, SensorDefinition> {
    let file = File::open(yaml_file).unwrap();
    let mut sensors_map = HashMap::new();
    let sensors_vec = serde_yaml::from_reader::<BufReader<File>, Vec<SensorDefinition>>(BufReader::new(file)).unwrap();
    for sensor_def in sensors_vec {
        sensors_map.insert(sensor_def.id.clone(), sensor_def);
    }
    sensors_map
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use SensorDefinition;

    extern crate serde_yaml;

    #[test]
    fn validate_yaml() {
        let file = File::open("sensors/sensors.yml").unwrap();
        //TODO load this into a Map rather than a Vec
        let sensors_vec = serde_yaml::from_reader::<BufReader<File>, Vec<SensorDefinition>>(BufReader::new(file)).unwrap();
        println!("sensors: {:?}", sensors_vec);
    }
}
