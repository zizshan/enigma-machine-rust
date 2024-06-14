use std::{cell::RefCell, fmt, rc::Rc, str};

use crate::enigma_types::*;

// use in enigma machine,
// it will output a mapping charater
// according to the mapping_array it is given.
pub struct EnigmaRotor {
    window: char,
    notch: char,
    forward_mapping_array: Vec<usize>,
    backward_mapping_array: Vec<usize>,
    pre_rotor: Option<Rc<RefCell<EnigmaRotor>>>,
    next_rotor: Option<Rc<RefCell<EnigmaRotor>>>,
    offset: usize,
}

impl EnigmaRotor {
    pub fn new() -> Self {
        // default all mapping is return the same charater
        Self {
            notch: 'A',
            window: 'A',
            forward_mapping_array: (0..26_usize).into_iter().map(|c| c).collect::<Vec<usize>>(),
            backward_mapping_array: (0..26_usize).into_iter().map(|c| c).collect::<Vec<usize>>(),
            pre_rotor: None,
            next_rotor: None,
            offset: 0,
        }
    }

    // create a rotor with given mapping/wiring
    pub fn new_with_setting(
        setting: EnigmaRotorSetting,
        next_rotor: Option<Rc<RefCell<EnigmaRotor>>>,
        pre_rotor: Option<Rc<RefCell<EnigmaRotor>>>,
    ) -> Self {
        let mut ret = Self::new();

        // lazy approach
        // if wiring is invaild return default setting(which every char is mapping to itself)
        if let Ok(_) =
            ret.set_rotor_wiring_with_str(setting.wiring.forward, setting.wiring.backward)
        {
            ret.notch = setting.notch;
            ret.pre_rotor = pre_rotor;
            ret.next_rotor = next_rotor;

            ret.set_window(setting.window);
        }

        ret
    }

    // set rotor window
    pub fn set_window(&mut self, c: char) {
        self.window = c;
        self.offset = (c as u8 - b'A') as usize;
    }

    // 'link' to next rotor
    pub fn set_next_rotor(&mut self, rotor: Rc<RefCell<EnigmaRotor>>) {
        self.next_rotor = Some(rotor.clone());
    }

    // 'link' to pre rotor
    pub fn set_pre_rotor(&mut self, rotor: Rc<RefCell<EnigmaRotor>>) {
        self.pre_rotor = Some(rotor.clone());
    }

    // change rotor's mapping
    pub fn set_rotor_wiring_with_str(
        &mut self,
        forward: &str,
        backward: &str,
    ) -> Result<(), EnigmaRotorWireError> {
        let owned_forward = forward.to_owned().to_uppercase();
        let owend_backward = backward.to_owned().to_uppercase();

        // convert all char to u8, 'A' as index 0
        let mut f_v = owned_forward.as_bytes().to_owned();
        let mut b_v = owend_backward.as_bytes().to_owned();
        f_v.iter_mut().zip(b_v.iter_mut()).for_each(|(f, b)| {
            *f = (*f) - b'A';
            *b = (*b) - b'A';
        });

        self.set_rotor_wiring_with_vec(f_v, b_v)
    }

    // private function, not use outside the module
    // hlep to set up wiring
    fn set_rotor_wiring_with_vec(
        &mut self,
        forward: Vec<u8>,
        backward: Vec<u8>,
    ) -> Result<(), EnigmaRotorWireError> {
        if forward.len() != 26 || backward.len() != 26 {
            // if length is not 26 it is invaild
            return Err(EnigmaRotorWireError::InvaildLength);
        } else {
            // make all item as usize
            let forward = forward.iter().map(|&u| u as usize).collect::<Vec<usize>>();
            let backward = backward.iter().map(|&u| u as usize).collect::<Vec<usize>>();

            // then check whether wiring is valid
            if !Self::is_vec_wiring_vaild(&forward, &backward) {
                return Err(EnigmaRotorWireError::InvaildWiring);
            } else {
                // if valid, set up mapping array
                self.forward_mapping_array.clear();
                self.backward_mapping_array.clear();
                forward
                    .into_iter()
                    .zip(backward.into_iter())
                    .for_each(|(f, b)| {
                        self.forward_mapping_array.push(f);
                        self.backward_mapping_array.push(b);
                    });

                Ok(())
            }
        }
    }

    // encode a signal that pass in to the rotor
    // signal is covert to an index, for convenience.
    pub fn encode_forward_index(&self, idx: usize) -> usize {
        // each rotor will rotate after condition is matched,
        // this will create a mapping before signal enter rotor's wiring/mapping
        // the mapping it create is represented with an offset.
        let idx = idx as isize - self.offset as isize;
        let idx = if idx < 0 {
            (idx + 26) as usize
        } else {
            (idx % 26) as usize
        };

        // same as the moment enter rotor,
        // there is also a mapping when siganl leave rotor
        // so after covert we add it back
        let ret_idx = (self.forward_mapping_array[idx] + self.offset) % 26;

        // if there is a next rotor, we pass the signal to the next,
        // continue the encoding process
        if let Some(next_rotor) = self.next_rotor.clone() {
            next_rotor.borrow().encode_forward_index(ret_idx)
        } else {
            ret_idx
        }
    }

    // encode a signal that pass in to the rotor
    // signal is covert to an index, for convenience.
    pub fn encode_backward_index(&self, idx: usize) -> usize {
        // each rotor will rotate after condition is matched,
        // this will create a mapping before signal enter rotor's wiring/mapping
        // the mapping it create is represented with an offset.
        let idx = idx as isize - self.offset as isize;
        let idx = if idx < 0 {
            (idx + 26) as usize
        } else {
            (idx % 26) as usize
        };

        // same as the moment enter rotor,
        // there is also a mapping when siganl leave rotor
        // so after covert we add it back
        let ret_idx = (self.backward_mapping_array[idx] + self.offset) % 26;

        // if there is a previous rotor, we pass the signal to the previous,
        // continue the encoding process
        if let Some(pre_rotor) = self.pre_rotor.clone() {
            pre_rotor.borrow().encode_backward_index(ret_idx)
        } else {
            ret_idx
        }
    }

    //  rotate the rotor
    pub fn rotate(&mut self) {
        if self.window == self.notch {
            if let Some(next) = self.next_rotor.clone() {
                let mut b_rotor = next.borrow_mut();
                b_rotor.rotate();
            }
        }
        self.offset = (self.offset + 1) % 26;
        self.window = (self.offset as u8 + b'A') as char;
    }

    // private function, check the wiring is correct or not
    fn is_vec_wiring_vaild(f_wiring: &Vec<usize>, b_wiring: &Vec<usize>) -> bool {
        // first check frequency
        let mut freq = [0; 26];
        let mut is_vaild = true;
        f_wiring.iter().for_each(|&b| {
            freq[b] += 1;
            is_vaild = freq[b] <= 1;
        });

        // every char shoulg appear only once.
        if !is_vaild || freq.into_iter().sum::<i32>() != 26 {
            return false;
        }

        freq = [0; 26];
        // same to the backward wiring
        b_wiring.iter().for_each(|&b| {
            freq[b] += 1;
            is_vaild = freq[b] <= 1;
        });

        if !is_vaild || freq.into_iter().sum::<i32>() != 26 {
            return false;
        }

        // next check mapping
        // a char should map to a char,
        // and when backward the mapping char shoud map to that char
        for (idx, &c) in f_wiring.iter().enumerate() {
            is_vaild = idx == b_wiring[c];
            if !is_vaild {
                break;
            }
        }

        is_vaild
    }
}

// for debug
impl fmt::Display for EnigmaRotor {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f = self
            .forward_mapping_array
            .iter()
            .map(|&u| u as u8 + b'A')
            .collect::<Vec<u8>>();
        let b = self
            .backward_mapping_array
            .iter()
            .map(|&u| u as u8 + b'A')
            .collect::<Vec<u8>>();
        write!(fmt, "EnigmaRotor: {{\n\twindow: {},\n\tnotch: {},\n\toffset: {},\n\tforward: {},\n\tbackward: {}\n}}", self.window, self.notch, self.offset, str::from_utf8(f.as_slice()).unwrap(), str::from_utf8(b.as_slice()).unwrap())
    }
}

// a char can be map to other char before and after a encoding process
// plugboard basically connect two charater with a wiring ,
// so they are swap when input and output
pub struct EnigmaPlugBoard {
    mapping_array: Vec<usize>,
}

impl EnigmaPlugBoard {
    pub fn new() -> Self {
        Self {
            mapping_array: (0..26_usize).into_iter().map(|c| c).collect::<Vec<usize>>(),
        }
    }

    // add swapping charater
    pub fn add_wire(
        &mut self,
        wire: EnigmaPlugBoardWire,
    ) -> Result<EnigmaPlugBoardWire, EnigmaPlugBoardError> {
        let link = (
            (wire.0 as u8 - b'A') as usize,
            (wire.1 as u8 - b'A') as usize,
        );
        let mapping = (
            self.mapping_array[link.0] as usize,
            self.mapping_array[link.1] as usize,
        );

        // ensure there is not wire already plug in
        if mapping.0 == link.0 && mapping.1 == link.1 {
            // link two charater
            self.mapping_array[link.0] = link.1;
            self.mapping_array[link.1] = link.0;
            Ok(wire)
        } else {
            Err(EnigmaPlugBoardError::AlreadyHaveWire(wire))
        }
    }

    // swap two
    pub fn encode_index(&self, idx: usize) -> usize {
        self.mapping_array[idx]
    }
}

// reflector is place back in the last position in the machine
// it basically connect two node in the last rotor,
// so signal will send backward to the output through all rotor again,
// re-encode the signal
pub struct EnigmaReflector {
    mapping_array: Vec<usize>,
}

impl EnigmaReflector {
    pub fn new_with_str(s: &str) -> Self {
        let u8_slice = s.as_bytes();
        let u8_v = u8_slice.to_owned();
        Self::new_with_vec(u8_v)
    }

    pub fn new_with_vec(mapping: Vec<u8>) -> Self {
        let mut ret = Self {
            mapping_array: vec![],
        };
        mapping.into_iter().for_each(|v| {
            let u = v as u8;
            ret.mapping_array.push((u - b'A') as usize);
        });
        ret
    }

    pub fn set_reflect_with_str(&mut self, s: &str) {
        let u8_slice = s.as_bytes();
        let u8_v = u8_slice.to_owned();
        self.set_reflect_with_vec(u8_v);
    }

    pub fn set_reflect_with_vec(&mut self, r: Vec<u8>) {
        self.mapping_array.clear();
        r.into_iter().for_each(|v| {
            let u = v as u8;
            self.mapping_array.push((u - b'A') as usize);
        });
    }

    pub fn encode_index(&self, idx: usize) -> usize {
        self.mapping_array[idx]
    }
}
