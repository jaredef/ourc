pub struct Config {
    level: u32,
    ratio: f64,
    name: String,
}

impl Config {
    pub const DEFAULT_LEVEL: u32 = 3;

    pub fn tweak(base: Config, up: bool) -> Config {
        let lvl = if up { base.level + 1 } else { base.level };
        Config { level: lvl, ..base }
    }

    pub fn score(&self) -> f64 {
        self.ratio * 1.5
    }

    pub fn pick(&self, opt: Option<u32>) -> u32 {
        if let Some(x) = opt {
            x
        } else if self.level > Self::DEFAULT_LEVEL {
            self.level
        } else {
            0
        }
    }
}

mod inner {
    pub fn helper() -> i32 {
        -3
    }
}
