use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct NewTableRequest {
    pub table_name: String,
}
