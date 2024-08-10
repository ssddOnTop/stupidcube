use crate::faces::Colours;

#[derive(Default)]
pub struct WindowParams {
    width: f64,
    height: f64,
    fov: f64,
    cube_width: f64,
}

impl WindowParams {
    pub fn new(width: f64, height: f64, fov: f64, cube_width: f64) -> Self {
        Self {
            width,
            height,
            fov,
            cube_width,
        }
    }

    pub fn calc(
        &self,
        char: Colours,
        z_buffer: &mut [f64],
        char_buffer: &mut [char],
        calculation_params: CalculationParams,
    ) {
        const K1: f64 = 40.0;

        let (x, y, z) = calculation_params.calculate_for_surface(self.fov);

        let xp = (self.width / 2f64 - 2f64 * self.cube_width + K1 * z * x * 2f64) as i64;
        let yp = (self.height / 2f64 + K1 * z * y) as i64;

        let idx = xp + yp * self.width as i64;

        if idx >= 0 && idx < (self.width * self.height) as i64 && z > z_buffer[idx as usize] {
            z_buffer[idx as usize] = z;
            char_buffer[idx as usize] = char.as_char();
        }
    }
}

#[derive(derive_getters::Getters, Default)]
pub struct CalculationParams {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl CalculationParams {
    fn calculate_for_surface(&self, fov: f64) -> (f64, f64, f64) {
        let x = self.calculate_x();
        let y = self.calculate_y();
        let z = self.calculate_z() + fov;

        let ooz = 1.0 / z;

        (x, y, ooz)
    }
    fn calculate_x(&self) -> f64 {
        f64::from(self.y) * self.a.sin() * self.b.sin() * self.c.cos()
            - f64::from(self.z) * self.a.cos() * self.b.sin() * self.c.cos()
            + f64::from(self.y) * self.a.cos() * self.c.sin()
            + f64::from(self.z) * self.a.sin() * self.c.sin()
            + f64::from(self.x) * self.b.cos() * self.c.cos()
    }

    fn calculate_y(&self) -> f64 {
        f64::from(self.y) * self.a.cos() * self.c.cos()
            + f64::from(self.z) * self.a.sin() * self.c.cos()
            - f64::from(self.y) * self.a.sin() * self.b.sin() * self.c.sin()
            + f64::from(self.z) * self.a.cos() * self.b.sin() * self.c.sin()
            - f64::from(self.x) * self.b.cos() * self.c.sin()
    }

    fn calculate_z(&self) -> f64 {
        f64::from(self.z) * self.a.cos() * self.b.cos()
            - f64::from(self.y) * self.a.sin() * self.b.cos()
            + f64::from(self.x) * self.b.sin()
    }
}
