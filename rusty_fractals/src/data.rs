use crate::data_px::DataPx;
use crate::pixel_states::DomainElementState;
use bincode::{Decode, Encode, config, decode_from_std_read, encode_into_std_write};
use config::standard;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Encode, Decode, Clone)]
pub struct DataPxSer {
    pub is_alive: bool,
    pub data: DataSer,
}

#[derive(Clone, Copy, Encode, Decode)]
pub struct DataSer {
    pub origin_re: f64,
    pub origin_im: f64,
    pub value: u64,
    pub state: DomainElementState,
    pub quad: f64,
    // never color
}

pub fn init_from_data(file_name: &str) -> Vec<DataPx> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {:?}", file_name, e);
            return Vec::new();
        }
    };
    let mut reader = BufReader::new(file);

    let read_serializable: Vec<DataPxSer> = match decode_from_std_read(&mut reader, standard()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to decode data from {}: {:?}", file_name, e);
            Vec::new()
        }
    };

    read_serializable
        .into_iter()
        .map(|s| DataPx::new(s.is_alive, s.data))
        .collect()
}

pub fn save_data(file_name: &str, data: &[DataPx]) {
    let file = match File::create(file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create {}: {:?}", file_name, e);
            return;
        }
    };
    let mut writer = BufWriter::new(file);

    let write_serializable: Vec<DataPxSer> = data.iter().map(|px| px.to_serializable()).collect();

    if let Err(e) = encode_into_std_write(&write_serializable, &mut writer, standard()) {
        eprintln!("Failed to encode data to {}: {:?}", file_name, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_px;
    use std::fs;

    #[test]
    fn test_save_and_load() {
        let file_name = "test_data.bin";
        let original = vec![data_px::init_trivial()];

        // save
        save_data(file_name, &original);
        // load
        let loaded = init_from_data(file_name);

        if let Ok(meta) = fs::metadata(file_name) {
            assert!(meta.is_file());
        }
        assert_eq!(original.len(), loaded.len());

        let _ = fs::remove_file(file_name);
    }
}
