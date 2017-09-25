#[derive(Clone)]
pub struct Bits {
    height: usize,
    width: usize,
    bits: Vec<u64>,
}
impl Bits {
    pub fn new(height: usize, width: usize) -> Self {
        let words = (height * width) / 64 + 1;
        Self{height, width, bits: vec![0; words]}
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> bool {
        let ind = (x + y * self.width) / 64;
        let pos = (x + y * self.width) % 64;
        // (self.bits[ind] & (1 << pos)) != 0
        (unsafe{self.bits.get_unchecked(ind)} & (1 << pos)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize) {
        let ind = (x + y * self.width) / 64;
        let pos = (x + y * self.width) % 64;
        // self.bits[ind] |= 1 << pos;
        *unsafe{self.bits.get_unchecked_mut(ind)} |= 1 << pos;
    }
}
