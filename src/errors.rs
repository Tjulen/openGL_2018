quick_error! {
    #[derive(Debug)]
    pub enum EngineError {
        GetAttrib(attrib_name: String) {
            description(attrib_name)
            display("Could not get location number of shader attribute: {}", attrib_name)
        }
        CStringCreation(message: String) {
            description(message)
            display("CString creation error: {}", message)
        }
    }
}