/// The callpass module
pub mod callpass {
    ///
    /// Generate callpass from the callsign.
    ///
    ///  # Example
    ///
    /// ```
    /// use callpass_rust::callpass;
    /// let result = callpass::get_callpass("TESTING".to_string());
    /// # assert_eq!(result, 31421);
    /// ```
    #[allow(dead_code)]
    pub fn get_callpass(call: String) -> i16 {
        let call = call.to_uppercase();

        let mut i = 0;
        let mut hash: i16 = 0x73e2;

        while i < call.len() {
            hash = hash ^ (call.as_bytes()[i] as i16) << 8;
            if i + 1 < call.len() {
                hash = hash ^ (call.as_bytes()[i + 1] as i16);
            }
            i += 2;
        }

        hash & 0x7fff
    }

    ///
    /// Verify callpass against the callsign
    ///
    /// # Example
    ///
    /// ```
    /// use callpass_rust::callpass;
    /// let result = callpass::check_callpass("TESTING".to_string(), 31421);
    /// # assert_eq!(result, true);
    /// ```
    #[allow(dead_code)]
    pub fn check_callpass(call: String, pass: i16) -> bool {
        let code = get_callpass(call);
        code == pass
    }
}