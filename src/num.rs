use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone,Copy,PartialEq,Eq)]
pub struct Fraction {
    n:u64,
    d:u64
}
#[inline]
fn gcd(a:u128,b:u128) -> u128 {
    if b > a {
        gcd(b,a)
    } else if b == 0 {
        a
    } else {
        gcd(b,a%b)
    }
}
impl Fraction {
    #[inline]
    pub fn new(n:u64) -> Fraction {
        Fraction {
            n:n,
            d:1
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.n == 0
    }
}

impl Add for Fraction {
    type Output = Fraction;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if self.d == 1 && rhs.d == 1 {
            Fraction {
                n: self.n + rhs.n,
                d: 1
            }
        } else if self.d == rhs.d {
            let d = self.d as u128;
            let n = self.n as u128 + rhs.n as u128;

            let g = gcd(n,d);

            if g == 1 {
                Fraction {
                    n: n as u64,
                    d: d as u64
                }
            } else {
                Fraction {
                    n: (n / g) as u64,
                    d: (d / g) as u64
                }
            }
        } else {
            let ad = self.d as u128;
            let bd = rhs.d as u128;
            let an = self.n as u128 * bd;
            let bn = rhs.n as u128 * ad;
            let d = ad * bd;
            let n = an + bn;

            let g = gcd(n, d);

            if g == 1 {
                Fraction {
                    n: n as u64,
                    d: d as u64
                }
            } else {
                Fraction {
                    n: (n / g) as u64,
                    d: (d / g) as u64
                }
            }
        }
    }
}
impl AddAssign for Fraction {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Fraction {
    type Output = Fraction;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        if self.d == 1 && rhs.d == 1 {
            Fraction {
                n: self.n - rhs.n,
                d: 1
            }
        } else if self.d == rhs.d {
            let d = self.d as u128;
            let n = self.n as u128 - rhs.n as u128;

            let g = gcd(n,d);

            if g == 1 {
                Fraction {
                    n: n as u64,
                    d: d as u64
                }
            } else {
                Fraction {
                    n: (n / g) as u64,
                    d: (d / g) as u64
                }
            }
        } else {
            let ad = self.d as u128;
            let bd = rhs.d as u128;
            let an = self.n as u128 * bd;
            let bn = rhs.n as u128 * ad;
            let d = ad * bd;
            let n = an - bn;

            let g = gcd(n, d);

            if g == 1 {
                Fraction {
                    n: n as u64,
                    d: d as u64
                }
            } else {
                Fraction {
                    n: (n / g) as u64,
                    d: (d / g) as u64
                }
            }
        }
    }
}
impl SubAssign for Fraction {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Div<u64> for Fraction {
    type Output = Fraction;
    #[inline]
    fn div(self, rhs: u64) -> Self::Output {
        let n = self.n as u128;
        let d = self.d as u128 * rhs as u128;

        let g = gcd(n,d);

        if g == 1 {
            Fraction {
                n: n as u64,
                d: d as u64
            }
        } else {
            Fraction {
                n: (n / g) as u64,
                d: (d / g) as u64
            }
        }
    }
}
impl DivAssign<u64> for Fraction {
    #[inline]
    fn div_assign(&mut self, rhs: u64) {
        *self = *self / rhs;
    }
}
impl Mul<u64> for Fraction {
    type Output = Fraction;
    #[inline]
    fn mul(self, rhs: u64) -> Self::Output {
        let n = self.n as u128 * rhs as u128;
        let d = self.d as u128;

        let g = gcd(n,d);

        if g == 1 {
            Fraction {
                n: n as u64,
                d: d as u64
            }
        } else {
            Fraction {
                n: (n / g) as u64,
                d: (d / g) as u64
            }
        }
    }
}
impl MulAssign<u64> for Fraction {
    #[inline]
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self * rhs;
    }
}
impl Ord for Fraction {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.d == other.d {
            self.n.cmp(&other.n)
        } else {
            let ad = self.d as u128;
            let bd = other.d as u128;
            let an = self.n as u128;
            let bn = other.n as u128;

            let an = an * bd;
            let bn = bn * ad;

            an.cmp(&bn)
        }
    }
}
impl PartialOrd for Fraction {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Debug for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} / {}",self.n,self.d)
    }
}