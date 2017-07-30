use number_math::power_mod;

struct MyLogRandomGenerator {
    last_element: i64,
    prime: i64,
    primitive_root: i64,
}

impl MyLogRandomGenerator {
    fn new_default(start: i64) -> MyLogRandomGenerator{
        MyLogRandomGenerator{last_element:start,prime:1092402413,primitive_root:8549672}
    }
}

impl Iterator for MyLogRandomGenerator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item>{
        self.last_element=power_mod(self.primitive_root, self.last_element, self.prime);
        Some(self.last_element>self.prime/2)
    }
}

pub struct MyLogRandomIntGenerator {
    bits: i64,
    gen: MyLogRandomGenerator,
}

impl MyLogRandomIntGenerator {
    pub fn new(bits: i64, seed: i64) -> MyLogRandomIntGenerator{
        assert!(64>=bits && bits>=0);
        MyLogRandomIntGenerator{bits:bits, gen:MyLogRandomGenerator::new_default(seed)}
    }

    pub fn get(&mut self, bits: i64) -> i64{
        let mut result:i64=0;

        for pos in 0..bits{
            if self.gen.next().unwrap(){
                result|=1<<pos;
            }
        }

        result
    }
}

impl Iterator for MyLogRandomIntGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item>{
        let mut result:i64=0;

        for pos in 0..self.bits{
            if self.gen.next().unwrap(){
                result|=1<<pos;
            }
        }

        Some(result)
    }
}