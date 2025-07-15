pub trait Addressable {
    fn read(&self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> bool;

    fn read2(&self, addr: u16) -> Option<u16> {
        if let Some(lo) = self.read(addr) {
            if let Some(hi) = self.read(addr + 1) {
                return Some((lo as u16) | ((hi as u16) << 8));
            }
        }
        None
    }

    fn write2(&mut self, addr: u16, value: u16) -> bool {
        let lo = (value & 0xff) as u8;
        let hi = ((value >> 8) & 0xff) as u8;

        self.write(addr, lo) && self.write(addr + 1, hi)
    }

    fn copy(&mut self, from: u8, to: u8, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read((from + (i as u8)).into()) {
                if !self.write((to + (i as u8)).into(), x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

pub struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl LinearMemory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u16) -> Option<u8> {
        if (addr as usize) < self.size {
            Some(self.bytes[addr as usize])
        } else {
            None
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> bool {
        if (addr as usize) < self.size {
            self.bytes[addr as usize] = value;
        } else {
            return false;
        }
        return true;
    }
}
