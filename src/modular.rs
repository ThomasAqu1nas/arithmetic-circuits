use alloy_primitives::{U256, U512};
pub trait FiniteModular {
    fn modulus() -> U256 {
        U256::from_str_radix("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47", 16).unwrap()
    }
    fn mod_add(self, rhs: Self) -> Self;
    fn mod_mul(self, rhs: Self) -> Self;
    fn mod_sub(self, rhs: Self) -> Self;
    fn mod_inv(self) -> Option<Self> where Self: Sized;
    fn mod_exp(base: U256, exp: U256) -> U256;
}

impl FiniteModular for U256 {
    // 30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47;


    fn mod_add(self, rhs: Self) -> Self {
        (self.wrapping_rem(Self::modulus()) + rhs.wrapping_rem(Self::modulus())).wrapping_rem(Self::modulus())
    }

    fn mod_mul(self, rhs: Self) -> Self {
        (self.wrapping_rem(Self::modulus()) * rhs.wrapping_rem(Self::modulus())).wrapping_rem(Self::modulus())
    }

    fn mod_sub(self, rhs: Self) -> Self {
        (self.wrapping_rem(Self::modulus()) - rhs.wrapping_rem(Self::modulus())).wrapping_rem(Self::modulus())

    }

    fn mod_inv(self) -> Option<Self> {
        if self == U256::ZERO {
            None
        } else {
            Some(Self::mod_exp(self, Self::modulus().wrapping_sub(U256::from(2))))
        }
    }
    
    fn mod_exp(base: U256, exp: U256) -> U256 {
        let (mut base, mut exp) = (U512::from(base), U512::from(exp));
        let mut result = U512::from(1);
        base = base.wrapping_rem(U512::from(Self::modulus()));
        while exp > U512::from(0) {
            if exp & U512::from(1) == U512::from(1) {
                result = result.checked_mul(base).unwrap().wrapping_rem(U512::from(Self::modulus()));
            }
            base = base.wrapping_mul(base).wrapping_rem(U512::from(Self::modulus()));
            exp = exp >> 1;
        }
        U256::from(result)
    }
    
    fn modulus() -> U256 {
        U256::from_str_radix("30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47", 16).unwrap()
    }


}

pub trait ExtendedGcd {
    fn gcd(a: Self, b: Self) -> (Self, Self, Self) where Self: Sized;
}

impl ExtendedGcd for U256 {
    fn gcd(mut a: Self, mut b: Self) -> (Self, Self, Self) {
        // Если b = 0, ответ очевиден
        if b == U256::ZERO {
            return (a, U256::from(1), U256::from(0));
        }

        // Инициализируем коэффициенты
        let (mut x0, mut x1) = (U256::from(1), U256::from(0));
        let (mut y0, mut y1) = (U256::from(0), U256::from(1));

        // Итеративный цикл
        while b != U256::from(0) {
            let q = a / b;         // Частное
            let r = a - q * b;     // Остаток

            a = b;
            b = r;

            // Обновляем коэффициенты
            let tx = x0 - q * x1;
            x0 = x1;
            x1 = tx;

            let ty = y0 - q * y1;
            y0 = y1;
            y1 = ty;
        }

        // a стало НОД, x0 и y0 – коэффициенты (x, y)
        (a, x0, y0)
    }
}
