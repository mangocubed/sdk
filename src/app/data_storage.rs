#[cfg(feature = "desktop")]
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[cfg(any(feature = "desktop", feature = "mobile"))]
use serde_json::Value;

#[cfg(any(feature = "desktop", feature = "mobile"))]
static DATA_FILE_STORAGE: std::sync::LazyLock<DataFileStorage> = std::sync::LazyLock::new(|| {
    use std::fs;

    let config_dir = get_config_dir();
    let data_file_path = config_dir.join("data.json");

    if !fs::exists(&data_file_path).expect("Could not check if data file exists") {
        fs::write(&data_file_path, b"{}").expect("Could not write data file");
    }

    DataFileStorage(data_file_path)
});
#[cfg(feature = "desktop")]
static PROJECT_DIRS: OnceLock<directories::ProjectDirs> = OnceLock::new();

pub trait DataStorage {
    fn new() -> Self;

    fn delete(&self, key: &str);

    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T>;

    fn set<T: Serialize>(&self, key: &str, value: &T);
}

#[cfg(feature = "web")]
struct DataWebStorage(web_sys::Storage);

#[cfg(feature = "web")]
impl DataStorage for DataWebStorage {
    fn new() -> Self {
        Self(
            web_sys::window()
                .expect("Could not get window")
                .local_storage()
                .ok()
                .flatten()
                .expect("Could not get localStorage"),
        )
    }

    fn delete(&self, key: &str) {
        let _ = self.0.delete(key);
    }

    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.0
            .get(key)
            .ok()
            .flatten()
            .and_then(|value| serde_json::from_str(&value).ok())
    }

    fn set<T: Serialize>(&self, key: &str, value: &T) {
        let _ = self.0.set(key, &serde_json::to_string(value).unwrap());
    }
}

#[cfg(feature = "desktop")]
fn get_config_dir() -> std::path::PathBuf {
    let config_dir = PROJECT_DIRS.get().expect("Could not get project dirs").config_dir();

    std::fs::create_dir_all(config_dir).expect("Could not create config dir");

    config_dir.to_path_buf()
}

#[cfg(all(feature = "mobile", target_os = "android"))]
fn get_config_dir() -> std::path::PathBuf {
    use std::{path, sync};

    use jni::JNIEnv;
    use jni::objects::{JObject, JString};

    let (tx, rx) = sync::mpsc::channel();

    fn run(env: &mut JNIEnv<'_>, activity: &JObject<'_>) -> Result<path::PathBuf, jni::errors::Error> {
        let files_dir = env.call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])?.l()?;
        let files_dir: JString<'_> = env
            .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?
            .l()?
            .into();
        let files_dir: String = env.get_string(&files_dir)?.into();

        Ok(path::PathBuf::from(files_dir))
    }

    dioxus::mobile::wry::prelude::dispatch(move |env, activity, _webview| tx.send(run(env, activity)).unwrap());

    rx.recv().unwrap().unwrap()
}

#[cfg(all(feature = "mobile", not(target_os = "android")))]
fn get_config_dir() -> std::path::PathBuf {
    unimplemented!()
}

#[cfg(feature = "desktop")]
pub fn set_project_dirs(application: &str) {
    let _ = PROJECT_DIRS
        .set(directories::ProjectDirs::from("app", "mango3", application).expect("Could not get project dirs"));
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
#[derive(Clone)]
struct DataFileStorage(std::path::PathBuf);

#[cfg(any(feature = "desktop", feature = "mobile"))]
impl DataFileStorage {
    fn read_data_file(&self) -> Value {
        use std::{fs, io};

        let data_file = fs::File::open(&self.0).expect("Could not open data file");
        let data_file_reader = io::BufReader::new(data_file);

        serde_json::from_reader(data_file_reader).unwrap_or_default()
    }

    fn write_data_file(&self, content: Value) {
        let data_file = std::fs::File::create(&self.0).expect("Could not create data file");

        serde_json::to_writer(data_file, &content).expect("Could not write data file");
    }
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
impl DataStorage for DataFileStorage {
    fn new() -> Self {
        DATA_FILE_STORAGE.clone()
    }

    fn delete(&self, key: &str) {
        let mut json_data = self.read_data_file();

        json_data[key] = Value::Null;

        self.write_data_file(json_data);
    }

    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let json_data = self.read_data_file();

        json_data
            .get(key)
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    fn set<T: Serialize>(&self, key: &str, value: &T) {
        let mut json_data = self.read_data_file();

        json_data[key] = serde_json::to_value(value).unwrap();

        self.write_data_file(json_data);
    }
}

impl DataStorage for () {
    fn new() -> Self {}

    fn delete(&self, _key: &str) {}

    fn get<T: for<'de> Deserialize<'de>>(&self, _key: &str) -> Option<T> {
        None
    }

    fn set<T: Serialize>(&self, _key: &str, _value: &T) {}
}

pub fn data_storage() -> impl DataStorage {
    #[cfg(feature = "web")]
    {
        DataWebStorage::new()
    }

    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        DataFileStorage::new()
    }
}
