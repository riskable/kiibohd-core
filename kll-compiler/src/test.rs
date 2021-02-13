#[cfg(test)]
mod tests {
    use crate::types::KllFile;

    #[test]
    fn test_define() {
        let result = KllFile::from_str("myDefine => myCDefine;\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_quoted() {
        let result = KllFile::from_str("\"Foo Bar\" = \"Baz Cubed\";\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_array() {
        let result = KllFile::from_str("Name_Foo[0] = myKeymapFile;\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_capability() {
        let result = KllFile::from_str("myCapability => myCFunction(arg1:1, arg2:2);\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_scancode() {
        let result = KllFile::from_str("S100 : U\"A\";\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_pixelmap() {
        let result = KllFile::from_str("P[5](30:8) : S13;\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_position() {
        let result = KllFile::from_str("P[30] <= x:20,rx:15;\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_animation() {
        let result = KllFile::from_str("A[MyEyesAreBleeding] <= start, loop:3;\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_frame() {
        let result = KllFile::from_str("A[Bleeed, 5] <= P[2](255,255,255);\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_result() {
        let result = KllFile::from_str("S100 : P[23](+43,+21,-40);\n");
        assert!(result.is_ok());
    }
}
