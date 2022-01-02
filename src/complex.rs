use rand::Rng;
use std::ops;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn square(&self) -> Complex {
        let real = self.real*self.real - self.imag*self.imag;
        let imag = 2.0 * self.real * self.imag;

        return Complex {
            real,
            imag,
        }
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        return Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:.2}+{:.2}i", self.real, self.imag);
    }
}

pub fn random_complex(
    real_lower_bound: f64,
    real_upper_bound: f64,
    imag_lower_bound: f64,
    imag_upper_bound: f64,
) -> Complex {
    let mut rng = rand::thread_rng();

    let real: f64 = rng.gen_range(real_lower_bound..=real_upper_bound);
    let imag: f64 = rng.gen_range(imag_lower_bound..=imag_upper_bound);

    return Complex {
        real,
        imag,
    }
}