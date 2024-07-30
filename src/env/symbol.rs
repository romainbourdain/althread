use super::{datatype::DataType, value::Value};

#[derive(Debug)]
pub struct Symbol {
    pub datatype: DataType,
    pub mutable: bool,
    pub value: Value,
}

impl Symbol {
    pub fn new(
        mutable: bool,
        datatype: Option<DataType>,
        value: Option<Value>,
    ) -> Result<Self, String> {
        let (datatype, value) = match (datatype, value) {
            (Some(datatype), Some(value)) => (datatype, value),
            (Some(datatype), None) => {
                let value = Value::from_datatype(&datatype);
                (datatype, value)
            }
            (None, Some(value)) => {
                let datatype = DataType::from_value(&value);
                (datatype, value)
            }
            (None, None) => (DataType::Void, Value::Null),
        };
        let value_type = DataType::from_value(&value);

        if datatype != value_type {
            return Err(format!(
                "Wrong type for variable: expected {:?}, found {:?}",
                datatype, value_type
            ));
        }

        Ok(Self {
            datatype,
            mutable,
            value,
        })
    }

    pub fn update(&mut self, value: Value) -> Result<(), String> {
        if !self.mutable {
            return Err(format!("Variable is not mutable"));
        }

        let value_datatype = DataType::from_value(&value);
        if self.datatype != value_datatype {
            return Err(format!(
                "Wrong type for variable: expected {}, found {}",
                self.datatype, value_datatype
            ));
        }

        self.value = value;
        Ok(())
    }
}
