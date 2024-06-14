mod enigma_machine;
pub use enigma_machine::*;

mod enigma_types;
pub use enigma_types::*;

mod enigma_component;
pub use enigma_component::*;

#[cfg(test)]
mod test {
    use crate::{*};

    #[test]
    fn test_diff_cipher_and_plain() {
        let mut enigma_machine = EnigmaMachine::new_with_all_setting(
            vec![
                EnigmaRotorSetting::I,
                EnigmaRotorSetting::II,
                EnigmaRotorSetting::III,
            ],
            "YRUHQSLDPXNGOKMIEBFZCWVJAT",
        );

        enigma_machine.set_window("AAA");
        let plain_text = "ILOVERUST";
        let cipher_text = enigma_machine.encode_str(plain_text);
        enigma_machine.set_window("AAA");
        let decode_text = enigma_machine.encode_str(cipher_text.as_str());

        // decode should be the same.
        assert_eq!(plain_text, decode_text.as_str());
        // but cipher should be different.
        assert_ne!(plain_text, cipher_text.as_str());
        // assert!(false);
    }
    
    #[test]
    fn test_plugboard() {
        let mut enigma_machine = EnigmaMachine::new_with_all_setting(
            vec![
                EnigmaRotorSetting::I,
                EnigmaRotorSetting::II,
                EnigmaRotorSetting::III,
            ],
            "YRUHQSLDPXNGOKMIEBFZCWVJAT",
        );

        enigma_machine.set_window("AAA");
        let plain_text = "ILOVERUST";

        // connect R and A
        let plugboard_wire = EnigmaPlugBoardWire('A', 'R');

        enigma_machine.set_window("AAA");
        // plaintext doesn't contain 'A', so we got lazy and don't replace A.
        let compare_before_plugboard = enigma_machine.encode_str(plain_text.replace("R", "A").as_str());
        // imitate process in plug board, R->A, A->R
        let compare_before_plugboard = compare_before_plugboard.chars().map(|c| {
            if c == 'A' {
                return 'R';
            } else if c == 'R' {
                return 'A';
            } else {
                return c;
            }
        }).collect::<String>();

        enigma_machine.set_window("AAA");
        let _ = enigma_machine.add_plug_wire(plugboard_wire);
        let with_plugboard_cipher = enigma_machine.encode_str(plain_text);

        enigma_machine.set_window("AAA");
        let plugboard_decode = enigma_machine.encode_str(with_plugboard_cipher.as_str());

        // two should be the same
        assert_eq!(compare_before_plugboard, with_plugboard_cipher);
        // even with plugboard, decode should still work
        assert_eq!(plugboard_decode.as_str(), plain_text);
        println!("plainText:\t{}\nmodify cipher:\t{}\nplugboard cipher:\t{}\ndecode:\t{}", plain_text, compare_before_plugboard, with_plugboard_cipher, plugboard_decode);
    }

    #[test]
    fn test_repeating_char() {
        let mut enigma_machine = EnigmaMachine::new_with_all_setting(
            vec![
                EnigmaRotorSetting::I,
                EnigmaRotorSetting::II,
                EnigmaRotorSetting::III,
            ],
            "YRUHQSLDPXNGOKMIEBFZCWVJAT",
        );

        enigma_machine.set_window("AAA");
        let plain_text = "TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT";
        let cipher_text = enigma_machine.encode_str(plain_text);
        enigma_machine.set_window("AAA");
        let decode_text = enigma_machine.encode_str(cipher_text.as_str());

        println!("plain: {}\ncipher: {}\ndecode: {}", plain_text, cipher_text, decode_text);
        // decode should be the same.
        assert_eq!(plain_text, decode_text.as_str());
        // but cipher should be different.
        assert_ne!(plain_text, cipher_text.as_str());
        // fail test intentionally to see output cipher whether the char is not a repeating pattern.
        // assert!(false);
    }
}
