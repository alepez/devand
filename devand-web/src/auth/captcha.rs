use captcha::{CaptchaName, Difficulty};
use std::path::PathBuf;
use uuid::Uuid;

pub struct CaptchaFile {
    path: Box<PathBuf>,
    value: String,
}

impl CaptchaFile {
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn into_data(self) -> Vec<u8> {
        std::fs::read(self.path.as_ref()).unwrap()
    }
}

impl Drop for CaptchaFile {
    fn drop(&mut self) {
        std::fs::remove_file(self.path.as_ref()).unwrap();
    }
}

impl CaptchaFile {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let filename = uuid.to_string() + ".png";
        let path = PathBuf::from("/tmp").join(filename);
        let path = Box::new(path);

        let c = captcha::by_name(Difficulty::Medium, CaptchaName::Mila);

        c.save(&path).expect("save failed");

        let value = c.chars_as_string();

        CaptchaFile { path, value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_and_destroy() {
        #[allow(unused_assignments)]
        let mut path = Box::new(PathBuf::default());
        {
            let x = CaptchaFile::new();
            path = x.path.clone();
            assert!(path.exists());
        }
        assert!(!path.exists());
    }

    #[test]
    fn captcha_value() {
        let x = CaptchaFile::new();
        let _value = x.value();
    }
}
