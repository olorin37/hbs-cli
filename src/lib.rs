#[derive(Debug)]
pub struct HelperDecl {
    pub name: String,
    pub file: String,
}
impl HelperDecl {
    fn new() -> HelperDecl{
        HelperDecl {
            name: String::new(),
            file: String::new(),
        }
    }
    pub fn from_str(inp: &str) -> HelperDecl {
        let xs: Vec<&str> = inp.split(":").collect();

        let mut helper = HelperDecl::new();
        let def_len = xs.len();
        if 0 < def_len && def_len <= 2 && xs[0].len() > 0 {
            helper.name.push_str(xs[ 0 ]);
            helper.file.push_str(xs[ def_len - 1 ]);
        } else {
            panic!("Helper must be provided in one of two formats: <name>:<file> or <file>. (There is {} items)", def_len);
        }

        helper
    }
}


#[test]
fn test_single() {
    let input = "helper";
    let h = HelperDecl::from_str(input);
    assert!(h.name == "helper");
    assert!(h.file == "helper");
}

#[test]
fn test_named_one() {
    let input = "nas:numfmt";
    let h = HelperDecl::from_str(input);
    assert!(h.name == "nas");
    assert!(h.file == "numfmt");
}

#[test]
#[should_panic]
fn test_empty() {
    let input = "";
    HelperDecl::from_str(input);
}

#[test]
#[should_panic]
fn test_too_many_params() {
    let input = "y:nas:numfmt";
    HelperDecl::from_str(input);
}

#[test]
#[should_panic]
fn test_too_many_seps() {
    let input = ":nas:";
    HelperDecl::from_str(input);
}
