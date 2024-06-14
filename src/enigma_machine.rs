use std::{cell::RefCell, fmt, rc::Rc, str};

use crate::{enigma_types::*, enigma_component::*};

pub struct EnigmaMachine {
    rotors: Vec<Rc<RefCell<EnigmaRotor>>>,
    reflector: EnigmaReflector,
    plug_board: EnigmaPlugBoard,
}

impl EnigmaMachine {
    fn new_with_reflector_and_empty_rotors(reflector_setting: &str) -> Self {
        Self {
            rotors: vec![],
            reflector: EnigmaReflector::new_with_str(reflector_setting),
            plug_board: EnigmaPlugBoard::new(),
        }
    }

    pub fn new_with_all_setting(
        rotor_settings: Vec<EnigmaRotorSetting>,
        reflector_setting_str: &str,
    ) -> Self {
        // first get one with no rotor init.
        let mut ret = Self::new_with_reflector_and_empty_rotors(reflector_setting_str);

        // chain all rotor that is created base on the setting it given up.
        let mut pre_rotor: Option<Rc<RefCell<EnigmaRotor>>> = None;
        ret.rotors = rotor_settings
            .iter()
            .map(|&setting| {
                // create rotor
                let rotor = EnigmaRotor::new_with_setting(setting, None, pre_rotor.clone());
                let rc_rotor = Rc::new(RefCell::new(rotor));

                // if there is a previous rotor, 
                // set this rotor is the next rotor to the previous one.
                if let Some(rc_pre_rotor) = pre_rotor.clone() {
                    rc_pre_rotor.borrow_mut().set_next_rotor(rc_rotor.clone());
                }

                pre_rotor = Some(rc_rotor.clone());
                return rc_rotor;
            })
            .collect::<_>();

        ret
    }

    // set up reflector
    pub fn set_reflector(&mut self, s: &str) {
        self.reflector.set_reflect_with_str(s);
    }

    // each rotor can be set it's start up position
    // given it a fixed offset
    pub fn set_window(&mut self, s: &str) {
        self.rotors
            .iter()
            .zip(s.chars().into_iter())
            .for_each(|(rc_rotor, window)| {
                rc_rotor.borrow_mut().set_window(window);
            })
    }

    // add swap charater in plugboard
    pub fn add_plug_wire(
        &mut self,
        wire: EnigmaPlugBoardWire,
    ) -> Result<EnigmaPlugBoardWire, EnigmaPlugBoardError> {
        self.plug_board.add_wire(wire)
    }

    // encode process
    pub fn encode_charater(&mut self, c: char) -> char {
        // first conver to usize
        let char_in_usize = (c as u8 - b'A') as usize;

        // rotate all rotor(if needed)
        self.rotors[0].borrow_mut().rotate();

        // convert in plugboard
        let step1 = self.plug_board.encode_index(char_in_usize);

        // encode in rotors
        let step2 = self.rotors[0].borrow().encode_forward_index(step1);

        // in reflector
        let reflect_index = self.reflector.encode_index(step2);

        // encode in rotors (backward)
        let last_rotor = self.rotors.last().unwrap().borrow();
        let step3 = last_rotor.encode_backward_index(reflect_index);

        // finally in plugboard
        let last = self.plug_board.encode_index(step3);

        return (last as u8 + b'A') as char;
    }

    // same but will encode a str
    pub fn encode_str(&mut self, s: &str) -> String {
        let v8: Vec<u8> = s
            .chars()
            .into_iter()
            .map(|c| self.encode_charater(c) as u8)
            .collect::<Vec<u8>>();
        String::from(str::from_utf8(v8.as_slice()).unwrap())
    }
}

// for debug
impl fmt::Display for EnigmaMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rotor_string = self
            .rotors
            .iter()
            .map(|rc| rc.borrow().to_string())
            .collect::<Vec<String>>();
        write!(
            f,
            "EnigmaMachine:{{\n\thas {} rotors:\n\t\t{}}}",
            self.rotors.len(),
            rotor_string.join("\n\t\t")
        )
    }
}
