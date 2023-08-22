use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct NewTableRequest {
    pub table_name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DropTableRequest {
    pub table_name: String,
}
