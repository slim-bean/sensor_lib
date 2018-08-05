
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

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
    pub name: String,
    pub value: String,
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
