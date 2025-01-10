use bincode::{deserialize, serialize};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{self, Read, Write},
};

const DATA_DIR_NAME: &str = "sav-serialize";
const SAVE_FILE_NAME: &str = "save-data.sav";

#[derive(Debug, Default, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct SaveData {
    person: Person,
}

// Get the full path to the save file (including directory)
fn get_save_file_path(save_file_name: impl Into<String>) -> PathBuf {
    let data_dir = AppDirs::new(Some(DATA_DIR_NAME), true).unwrap().data_dir;
    data_dir.join(save_file_name.into())
}

// Read the saved data from the file
// If the file does not exist, it returns a default SaveData object.
fn read_save_data(save_file_path: &PathBuf) -> Result<SaveData, io::Error> {
    // If the file doesn't exist, return default SaveData
    if !std::path::Path::new(&save_file_path).exists() {
        return Ok(SaveData::default());
    }

    // Read the contents of the file into a vector
    let mut file = File::open(save_file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Deserialize the data and return it as SaveData
    Ok(deserialize(&data)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
        .unwrap_or_default())
}

// Serialize the SaveData and write it to the file
fn write_save_data(save_data: SaveData, save_file_path: &PathBuf) -> Result<(), io::Error> {
    // Serialize the SaveData into a byte vector
    let data =
        serialize(&save_data).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    // Create the file (or overwrite if it exists)
    let mut file = File::create(save_file_path)?;
    // Write the serialized data to the file
    file.write_all(&data)?;
    Ok(())
}

fn main() {
    // Get save file path
    let save_file_path = get_save_file_path(SAVE_FILE_NAME);
    println!("save file path: {:?}", save_file_path);

    // Read the saved data from file
    let mut save_data = read_save_data(&save_file_path).unwrap();
    println!("{:?}", save_data.person);

    // Toggle between "Tom" and "Jim" for the person's name, and change age
    if save_data.person.name == "Tom" {
        save_data.person.name = "Jim".into();
        save_data.person.age = 28;
    } else {
        save_data.person.name = "Tom".into();
        save_data.person.age = 32;
    }

    // Write the modified save data back to the file
    write_save_data(save_data, &save_file_path).unwrap();
}
