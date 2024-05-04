// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    pub red: i16,
    pub green: i16,
    pub blue: i16,
}

struct ColorTupleStruct(i16, i16, i16 /* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

impl UnitLikeStruct {
    pub fn unit_like_struct() -> String {
        return "UnitLikeStruct".to_string();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct::unit_like_struct();
        let message = format!("{}s are fun!", unit_like_struct.as_str());

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
