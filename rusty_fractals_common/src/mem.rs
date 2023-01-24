// Memory object to carry calculation results
pub struct Mem {
    pub re: f64,
    pub im: f64,
}

impl Mem {
    pub fn plus(&mut self, r: f64, i: f64) {
        self.re += r;
        self.im += i;
    }

    pub fn square(&mut self) {
        let temp = (self.re * self.re) - (self.im * self.im);
        self.im = 2.0 * self.re * self.im;
        self.re = temp;
    }

    pub fn quad(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn conjugation(&mut self) {
        self.im = -1.0 * self.im;
    }
}

pub fn new(re: f64, im: f64) -> Mem {
    Mem { re, im }
}


#[test]
fn test_plus() {
    let mut m = Mem { re: 2.0, im: 3.0 };

    m.plus(0.5, 0.4);

    assert_eq!(m.re, 2.5);
    assert_eq!(m.im, 3.4);
}

#[test]
fn test_square() {
    let mut m = Mem { re: 3.0, im: 2.0 };

    m.square();

    assert_eq!(m.re, 5.0);
    assert_eq!(m.im, 12.0);
}

#[test]
fn test_quad() {
    let m = Mem { re: 3.0, im: 2.0 };

    let q = m.quad();

    assert_eq!(q, 13.0);
}

#[test]
fn test_conjugation() {
    let mut m = Mem { re: 3.0, im: 2.0 };

    m.conjugation();

    assert_eq!(m.re, 3.0);
    assert_eq!(m.im, -2.0);
}
