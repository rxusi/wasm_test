#![allow(non_snake_case)]

pub struct Rng {
    x: u128,
    y: u128,
    z: u128,
    w: u128,
}

impl Rng {
    pub fn new(seed: u128) -> Self {
        let (x, y, z, w): (u128, u128, u128, u128);

        x = 998244353;
        y = (seed >> 32) & 0xffffffff;
        z = seed & 0xffffffff;
        w = x ^ z;

        let rng: Self = Self { x, y, z, w };

        return rng;
    }

    pub fn rand_usize(&mut self, _min: usize, _max: usize) -> usize {
        let (min, max): (u128, u128) = (_min as u128, _max as u128);

        let t: u128 = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);

        return (self.w % (max - min) + min) as usize;
    }
}