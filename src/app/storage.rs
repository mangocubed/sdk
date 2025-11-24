pub use dioxus_sdk::storage::{LocalStorage, SessionStorage, StorageBacking};

#[cfg(any(feature = "desktop", feature = "mobile"))]
pub fn set_directory(app_name: &str) {
    #[cfg(any(feature = "desktop", all(feature = "mobile", not(target_os = "ios"))))]
    let storage_dir = directories::ProjectDirs::from("app", "mango3", app_name)
        .expect("Could not get project dirs")
        .config_local_dir()
        .to_path_buf();

    #[cfg(all(feature = "mobile", target_os = "android"))]
    let storage_dir = {
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
    };

    dioxus_sdk::storage::set_directory(storage_dir);
}

#[cfg(any(feature = "web", feature = "server"))]
pub fn set_directory(_app_name: &str) {}
