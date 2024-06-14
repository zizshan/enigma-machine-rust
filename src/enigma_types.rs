// use to define where two wire is connected in plug board.
pub struct EnigmaPlugBoardWire(pub char, pub char);

#[derive(Clone, Copy)]
pub struct EnigmaRotorWiring<'a> {
    pub forward: &'a str,
    pub backward: &'a str,
}

#[derive(Clone, Copy)]
pub struct EnigmaRotorSetting<'a> {
    pub wiring: EnigmaRotorWiring<'a>,
    pub notch: char,
    pub window: char,
}

pub enum EnigmaRotorWireError {
    InvaildLength,
    InvaildWiring,
}
pub enum EnigmaPlugBoardError {
    // indicate that wire intend to use is already occupied.
    AlreadyHaveWire(EnigmaPlugBoardWire),
}

// some rotor setting can be use
impl EnigmaRotorSetting<'static> {
    pub const I: EnigmaRotorSetting<'static> = EnigmaRotorSetting {
        wiring: EnigmaRotorWiring {
            forward: "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            backward: "UWYGADFPVZBECKMTHXSLRINQOJ",
        },
        window: 'A',
        notch: 'Q',
    };

    pub const II: EnigmaRotorSetting<'static> = EnigmaRotorSetting {
        wiring: EnigmaRotorWiring {
            forward: "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            backward: "AJPCZWRLFBDKOTYUQGENHXMIVS",
        },
        window: 'A',
        notch: 'E',
    };

    pub const III: EnigmaRotorSetting<'static> = EnigmaRotorSetting {
        wiring: EnigmaRotorWiring {
            forward: "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            backward: "TAGBPCSDQEUFVNZHYIXJWLRKOM",
        },
        window: 'A',
        notch: 'V',
    };
}
