macro_rules! data {
    ($name: ident, $file: expr) => {
        mod _data {
            #[allow(non_upper_case_globals)]
            pub const $name: &'static str = include_str!(concat!("../resources/", $file));
        }
        impl $name {
            pub fn data() -> Result<Self,Box<Error>> {
                let json = try!(Json::from_str(_data::$name));
                Ok(try!(Self::from_json(&json)))
            }
        }
    }
}

