use captcha::filters::Noise;
use captcha::Captcha;
use std::path::{Path, PathBuf};

struct DisposableCaptcha {
    path: Box<PathBuf>,
}

impl Drop for DisposableCaptcha {
    fn drop(&mut self) {
        std::fs::remove_file(self.path.as_ref()).unwrap();
    }
}

fn generate() -> DisposableCaptcha {
    let path = PathBuf::from("/tmp/captcha.png");
    let path = Box::new(path);

    Captcha::new()
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(220, 120)
        .save(&path)
        .expect("save failed");

    DisposableCaptcha { path }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_and_destroy() {
        let x = generate();
        std::thread::sleep_ms(1_000);
    }
}
