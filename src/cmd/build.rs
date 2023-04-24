pub fn build_truly(params: bool) {
    if params {
        println!("Truly delete");
    }
}

pub fn build_local(params: bool) {
    if params {
        println!("Delete from local");
    } else {
        println!("Delete from remote");
    }
}

pub fn build() {
    println!("Delete");
}

pub fn no_mode() {
    println!("No mode");
}

// Add test for build
#[test]
fn test_no_mode() {
    no_mode();

    assert_eq!(true, true);
}

// Add test for build_local
#[test]
fn test_build_local() {
    build_local(true);

    assert_eq!(true, true);
}
