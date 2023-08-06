pub struct LegacyRandom {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    int_mask: u32,
    int_to_real: f64,
    bit_buffer: u32,
    bit_index: i32,
}

impl LegacyRandom {
    pub fn new(x: u32) -> LegacyRandom {
        LegacyRandom {
            x: x,
            y: 842502087,
            z: 3579807591,
            w: 273326509,
            int_mask: 0x7FFFFFFF,
            int_to_real: 1.0 / ((i32::MAX as u32 + 1) as f64),
            bit_buffer: 0,
            bit_index: 32,
        }
    }

    pub fn next_uint(&mut self) -> u32 {
        let t: u32 = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }
    pub fn next(&mut self) -> i32 {
        (self.int_mask & self.next_uint()) as i32
    }
    pub fn next_upper_bound(&mut self, upper_bound: i32) -> i32 {
        (self.next_double() * upper_bound as f64) as i32
    }
    pub fn next_lower_and_upper(&mut self, lower_bound: i32, upper_bound: i32) -> i32 {
        (lower_bound as f64 + self.next_double() * ((upper_bound - lower_bound) as f64)) as i32
    }
    pub fn next_lower_and_upper_float(&mut self, lower_bound: f64, upper_bound: f64) -> i32 {
        (lower_bound + self.next_double() * (upper_bound - lower_bound)) as i32
    }
    pub fn next_double(&mut self) -> f64 {
        self.int_to_real * self.next() as f64
    }
    pub fn next_bool(&mut self) -> bool {
        if self.bit_index == 32 {
            self.bit_buffer = self.next_uint();
            self.bit_index = 1;
        } else {
            self.bit_index += 1;
            self.bit_buffer >>= 1;
        };
        (self.bit_buffer & 1) == 1
    }
}

// pub struct UpBoundRandom {
//     legacy_random: LegacyRandom,
//     upper_bound: i32,
// }

// impl LegacyRandom for UpBoundRandom {
//     fn new(&self, &x: u32) {
//         init_random.x = x;
//     }

//     fn next(&self, upper_bound: u32) -> i32 {
//         next_double() * upper_bound as i32
//     }
// }
