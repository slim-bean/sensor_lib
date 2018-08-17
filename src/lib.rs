
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

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
    TwoByteFloat,
    SingleByteInt,
    TwoByteInt,
    FourByteInt,
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
    pub id: u16,
    pub name: String,
    pub format: SensorFormat,
    pub destination_queues: Vec<String>,
    pub metric_name: String,
    pub metric_type: MetricType,
    pub metric_labels: HashMap<String, String>,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorValue {
    pub id: u16,
    pub timestamp: u64,
    pub value: String,
}


pub fn load_from_file(yaml_file: &str) -> HashMap<u16, SensorDefinition> {
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
