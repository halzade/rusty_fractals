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
    let mut reader = BufReader::new(File::open(file_name).unwrap());

    let read_serializable: Vec<DataPxSer> = decode_from_std_read(&mut reader, standard()).unwrap();

    read_serializable
        .into_iter()
        .map(|s| DataPx::new(s.is_alive, s.data))
        .collect()
}

pub fn save_data(file_name: &str, data: &Vec<DataPx>) {
    let mut writer = BufWriter::new(File::create(file_name).unwrap());

    let write_serializable: Vec<DataPxSer> = data.iter().map(|px| px.to_serializable()).collect();

    encode_into_std_write(&write_serializable, &mut writer, standard()).unwrap();
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

        assert!(fs::metadata(file_name).unwrap().is_file());
        assert_eq!(original.len(), loaded.len());

        fs::remove_file(file_name).unwrap();
    }
}
