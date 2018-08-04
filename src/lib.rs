
#[macro_use]
extern crate serde_derive;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SensorFormat {
    Bool,
    TwoByteFloat,
    SingleByteInt,
    TwoByteInt,
    FourByteInt,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorDefinition {
    pub id: u16,
    pub name: String,
    pub format: SensorFormat,
    pub destination_queues: Vec<String>
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorValue {
    pub id: u16,
    pub name: String,
    pub value: String,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
