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
