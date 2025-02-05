use std::fmt;

/// A struct to represent elements of the field \( \mathbb{F}_{5^2} \)
#[derive(Clone, Copy, Debug, PartialEq)]
struct F5x2 {
    a: u8, // Coefficient for 1
    b: u8, // Coefficient for t
}

impl fmt::Display for F5x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),
            (0, b) => write!(f, "{}t", b),
            (a, b) => write!(f, "{} + {}t", a, b),
        }
    }
}

impl F5x2 {
    fn new(a: u8, b: u8) -> Self {
        F5x2 { a: a % 5, b: b % 5 }
    }

    fn add(self, other: F5x2) -> F5x2 {
        F5x2::new((self.a + other.a) % 5, (self.b + other.b) % 5)
    }

    fn sub(self, other: F5x2) -> F5x2 {
        F5x2::new((self.a + 5 - other.a) % 5, (self.b + 5 - other.b) % 5)
    }

    fn mul(self, other: F5x2) -> F5x2 {
        let a = self.a as i16;
        let b = self.b as i16;
        let c = other.a as i16;
        let d = other.b as i16;

        let ac = (a * c) % 5;
        let bd = (b * d) % 5;
        let ad_bc = (a * d + b * c) % 5;

        let new_a = (ac + 3 * bd) % 5;
        let new_b = ad_bc % 5;

        F5x2::new(new_a as u8, new_b as u8)
    }

    fn div(self, other: F5x2) -> F5x2 {
        let inv = other.inverse();
        self.mul(inv)
    }

    fn inverse(self) -> F5x2 {
        let a = self.a as i16;
        let b = self.b as i16;
        let denominator = (a * a - 3 * b * b).rem_euclid(5) as u8;
        let inv_denominator = Self::mod_inverse(denominator, 5);

        let new_a = (a * inv_denominator as i16).rem_euclid(5) as u8;
        let new_b = (5 - (b * inv_denominator as i16).rem_euclid(5)) as u8;

        F5x2::new(new_a, new_b)
    }

    fn mod_inverse(x: u8, p: u8) -> u8 {
        for i in 1..p {
            if (x as u16 * i as u16) % p as u16 == 1 {
                return i;
            }
        }
        1
    }
}

/// A struct to represent a point on the elliptic curve
#[derive(Clone, Copy, Debug)]
struct Point {
    x: Option<F5x2>,
    y: Option<F5x2>,
}

impl Point {
    fn new(x: Option<F5x2>, y: Option<F5x2>) -> Self {
        Point { x, y }
    }

    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

fn point_add(p: Point, q: Point, a: F5x2) -> Point {
    if p.is_infinity() {
        return q;
    }
    if q.is_infinity() {
        return p;
    }

    let (x1, y1) = (p.x.unwrap(), p.y.unwrap());
    let (x2, y2) = (q.x.unwrap(), q.y.unwrap());

    if x1 == x2 && y1 != y2 {
        return Point::new(None, None); // Point at infinity
    }

    let lambda = if x1 == x2 && y1 == y2 {
        let numerator = x1.mul(x1).mul(F5x2::new(3, 0)).add(a);
        let denominator = y1.mul(F5x2::new(2, 0));
        numerator.div(denominator)
    } else {
        let numerator = y2.sub(y1);
        let denominator = x2.sub(x1);
        numerator.div(denominator)
    };

    let x3 = lambda.mul(lambda).sub(x1).sub(x2);
    let y3 = lambda.mul(x1.sub(x3)).sub(y1);

    Point::new(Some(x3), Some(y3))
}

/// Scalar multiplication using double-and-add
fn point_mul(n: u8, p: Point, a: F5x2) -> Point {
    let mut result = Point::new(None, None); // Point at infinity
    let mut base = p;
    let mut k = n;

    while k > 0 {
        if k & 1 == 1 {
            result = point_add(result, base, a);
        }
        base = point_add(base, base, a);
        k >>= 1; // Right shift to divide by 2
    }

    result
}

fn main() {
    let a = F5x2::new(1, 0);
    let _b = F5x2::new(1, 0);

    let p = Point::new(Some(F5x2::new(1, 2)), Some(F5x2::new(4, 4)));
    let n = 3;

    let np = point_mul(n, p, a);

    match np {
        Point { x: Some(x), y: Some(y) } => println!("[{}]P = ({}, {})", n, x, y),
        _ => println!("[{}]P = Point at Infinity", n),
    }
}
