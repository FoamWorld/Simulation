#[derive(Debug)]
/// Refering to Jung's theory.  
/// `libito`: energy collected  
/// `functions`: mental functions activity
/// * recorded structure: \[Se, Ne, Te, Fe, Si, Ni, Ti, Fi\]
/// * recorded data: uses u8 to express general percentage
pub struct Character {
    libido: f32,
    functions: [u8; 8],
}
impl Character {
    pub fn new() -> Self {
        Character {
            libido: 0.0f32,
            functions: core::array::from_fn::<_, 8, _>(|_| 0u8),
        }
    }
    /// E for extroverts, I for introverts  
    /// S for sensation, N for intuition  
    /// T for thinking, F for feeling  
    /// J for judging, P for perceiving
    pub fn from_code(code: String, majors: [u8; 4]) -> Self {
        assert_eq!(4, code.len());
        let bytes = code.as_bytes();
        let decoded = [
            bytes[0] == b'E',
            bytes[1] == b'S',
            bytes[2] == b'T',
            bytes[3] == b'J',
        ];
        let mut reserve = core::array::from_fn::<_, 8, _>(|_| 0u8);
        for i in 0..=3 {
            let mut target = 0;
            if !(decoded[0] ^ decoded[3]) {
                // E..J or I..P
                if i == 0 {
                    target = if decoded[2] { 2 } else { 3 };
                } else if i == 1 {
                    target = if decoded[1] { 0 } else { 1 };
                } else if i == 2 {
                    target = if decoded[1] { 1 } else { 0 };
                } else {
                    target = if decoded[2] { 3 } else { 2 };
                }
            } else {
                // E..P or I..J
                if i == 0 {
                    target = if decoded[1] { 0 } else { 1 };
                } else if i == 1 {
                    target = if decoded[2] { 2 } else { 3 };
                } else if i == 2 {
                    target = if decoded[2] { 3 } else { 2 };
                } else {
                    target = if decoded[1] { 1 } else { 0 };
                }
            }
            // check 'e' or 'i'
            if decoded[0] ^ (i % 2 == 0) {
                reserve[target] = 0xffu8 - majors[i];
                reserve[target + 4] = majors[i];
            } else {
                reserve[target] = majors[i];
                reserve[target + 4] = 0xffu8 - majors[i];
            }
        }
        Character {
            libido: 0.0f32,
            functions: reserve,
        }
    }
}
