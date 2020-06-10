use captcha::{CaptchaName, Difficulty};
use std::path::PathBuf;
use uuid::Uuid;

pub struct CaptchFile {
    path: Box<PathBuf>,
    value: String,
}

impl CaptchFile {
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl Drop for CaptchFile {
    fn drop(&mut self) {
        std::fs::remove_file(self.path.as_ref()).unwrap();
    }
}

fn generate() -> CaptchFile {
    let uuid = Uuid::new_v4();
    let filename = uuid.to_string() + ".png";
    let path = PathBuf::from("/tmp").join(filename);
    let path = Box::new(path);

    let c = captcha::by_name(Difficulty::Medium, CaptchaName::Mila);

    c.save(&path).expect("save failed");

    let value = c.chars_as_string();

    CaptchFile { path, value }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_and_destroy() {
        #[allow(unused_assignments)]
        let mut path = Box::new(PathBuf::default());
        {
            let x = generate();
            path = x.path.clone();
            assert!(path.exists());
        }
        assert!(!path.exists());
    }

    #[test]
    fn captcha_value() {
        let x = generate();
        let value = x.value();
        dbg!(value);
    }
}
