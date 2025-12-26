use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub width: f32,
    pub height: f32,
}

impl Star {
    pub fn new_rng(window_w: f32, window_h: f32) -> Self {
        let mut rng = rand::rng();
        let size = rng.random_range(100.0..200.0);

        Self {
            x: rng.random_range(-window_w..window_w),
            y: rng.random_range(-window_h..window_h),
            z: rng.random_range(1.0..2000.0),

            width: size,
            height: size,
        }
    }

    pub fn update(&mut self, speed: f32, window_w: f32, window_h: f32) {
        self.z = self.z - speed;

        if self.z < 1.0 {
            self.z = 2000.0;

            let mut rng = rand::rng();
            self.x = rng.random_range(-window_w..window_w);
            self.y = rng.random_range(-window_h..window_h);
        }
    }
}
