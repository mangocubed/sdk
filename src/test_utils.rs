use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use fake::Fake;
use fake::faker::address::en::CountryCode;
use fake::faker::chrono::en::DateTimeBefore;
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::faker::name::en::Name;
use rand::rng;
use uuid::Uuid;

use crate::core::generate_random_string;
use crate::core::identity_client::Auth;

fn unique_fake<T, F>(prefix: &str, fake_fn: F) -> T
where
    F: Fn() -> T,
    T: Display,
{
    let file_path = std::env::temp_dir().join("used_fakes");

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(&file_path)
        .expect("Could not open file");

    let mut file_content = String::new();

    let _ = file.read_to_string(&mut file_content);

    let mut lines = file_content
        .lines()
        .map(|line| line.to_owned())
        .collect::<HashSet<String>>();

    if lines.len() > 100 {
        for line in lines.clone().iter().take(lines.len() - 100) {
            lines.remove(line);
        }
    }

    let _ = file.set_len(0);

    for line in &lines {
        let _ = file.write_all(format!("{line}\n").as_bytes());
    }

    let mut fake = fake_fn();

    while !lines.insert(format!("{prefix}_{fake}")) {
        fake = fake_fn();
    }

    let _ = file.write_all(format!("{prefix}_{fake}\n").as_bytes());

    fake
}

pub fn fake_auth<'a>() -> Auth<'a> {
    let token = Cow::Owned(generate_random_string(32));
    let expires_at = Utc::now() + TimeDelta::hours(1);

    Auth {
        token,
        expires_at,
        refreshed_at: None,
    }
}

pub fn fake_birthdate() -> NaiveDate {
    DateTimeBefore(Utc::now()).fake::<DateTime<Utc>>().date_naive()
}

pub fn fake_country_alpha2() -> String {
    CountryCode().fake()
}

pub fn fake_email() -> String {
    unique_fake("email", || FreeEmail().fake_with_rng(&mut rng()))
}

pub fn fake_name() -> String {
    unique_fake("name", || {
        let mut name: String = Name().fake_with_rng(&mut rng());
        name.truncate(256);
        name
    })
}

pub fn fake_password() -> String {
    Password(6..128).fake()
}

pub fn fake_username() -> String {
    unique_fake("username", || {
        let mut username: String = Username().fake_with_rng(&mut rng());
        username.truncate(16);
        username
    })
}

pub fn fake_uuid() -> Uuid {
    Uuid::new_v4()
}
