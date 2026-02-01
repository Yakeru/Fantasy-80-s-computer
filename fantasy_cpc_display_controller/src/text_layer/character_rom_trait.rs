pub trait FantasyCpc8by8CharacterRomTrait {
    fn get_name() -> String;
    fn get_char_table() -> String;
    fn get_char(c: char) -> [u8; 8];
}