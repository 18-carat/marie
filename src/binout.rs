pub struct BinaryOutput {
    pub bytes: Vec<u8>,
    pub code: [i16; 1000],
}

impl BinaryOutput {
    pub fn new(code: [i16; 1000], count: usize) -> Self {
        let bytes = code
            .iter()
            .flat_map(|x| vec![(x / 100) as u8, (x % 100) as u8])
            .take(count * 2)
            .collect::<Vec<u8>>();

        Self { bytes, code }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut code = [0; 1000];
        let mut i = 0;
        let mut j = 0;

        while i < bytes.len() {
            let c0 = bytes[i] as i16;
            let c1 = bytes[i + 1] as i16;
            code[j] = c0 * 100 + c1;

            i += 2;
            j += 1;
        }

        Self { bytes, code }
    }
}
