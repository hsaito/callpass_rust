mod callpass {
    #[allow(dead_code)]
    pub fn get_callpass(mut call:String) -> i16 {
        call = call.to_uppercase();

        let mut i = 0;
        let mut hash: i16 = 0x73e2;

        while i < call.len() {
            hash = hash ^ (call.as_bytes()[i] as i16)<<8;
            if i+1 < call.len() {
                hash = hash ^ (call.as_bytes()[i+1] as i16);
            }
            i += 2;
        }
        return hash & 0x7fff;
    }

    #[allow(dead_code)]
    pub fn check_callpass(call:String, pass: i16) -> bool {
        let code = get_callpass(call);
        return code == pass;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_callpass() {
        let code = callpass::get_callpass(String::from("TESTING"));
        assert_eq!(code, 31421);
    }

    #[test]
    fn test_check_callpass() {
        let result = callpass::check_callpass(String::from("TESTING"),31421);
        assert_eq!(result, true);
    }
}