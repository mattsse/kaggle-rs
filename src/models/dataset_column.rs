use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DatasetColumn {
    /// The order that the column comes in, 0-based. (The first column is 0,
    /// second is 1, etc.)
    order: Option<f32>,
    /// The column name
    name: Option<String>,
    /// The type of all of the fields in the column. Please see the data types on https://github.com/Kaggle/kaggle-api/wiki/Dataset-Metadata
    #[serde(rename = "type")]
    field_type: Option<String>,
    /// Used to store the original type of the column, which will be converted
    /// to Kaggle's types. For example, an `originalType` of `\"integer\"` would
    /// convert to a `type` of `\"numeric\"`
    #[serde(rename = "originalType")]
    original_type: Option<String>,
    /// The description of the column
    #[serde(rename = "description")]
    description: Option<String>,
}

impl DatasetColumn {
    pub fn set_order(&mut self, order: f32) {
        self.order = Some(order);
    }

    pub fn with_order(mut self, order: f32) -> DatasetColumn {
        self.order = Some(order);
        self
    }

    pub fn order(&self) -> Option<&f32> {
        self.order.as_ref()
    }

    pub fn reset_order(&mut self) {
        self.order = None;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_name(mut self, name: String) -> DatasetColumn {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = None;
    }

    pub fn set_type(&mut self, _type: String) {
        self.field_type = Some(_type);
    }

    pub fn with_type(mut self, _type: String) -> DatasetColumn {
        self.field_type = Some(_type);
        self
    }

    pub fn get_type(&self) -> Option<&String> {
        self.field_type.as_ref()
    }

    pub fn reset_type(&mut self) {
        self.field_type = None;
    }

    pub fn set_original_type(&mut self, original_type: String) {
        self.original_type = Some(original_type);
    }

    pub fn with_original_type(mut self, original_type: String) -> DatasetColumn {
        self.original_type = Some(original_type);
        self
    }

    pub fn original_type(&self) -> Option<&String> {
        self.original_type.as_ref()
    }

    pub fn reset_original_type(&mut self) {
        self.original_type = None;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn with_description(mut self, description: String) -> DatasetColumn {
        self.description = Some(description);
        self
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn reset_description(&mut self) {
        self.description = None;
    }
}
