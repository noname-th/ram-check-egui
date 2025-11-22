fn main() {
    println!("builing...");
    let mut res = winres::WindowsResource::new();
    res.set_icon("./ramCheck.ico");
    res.compile().unwrap();
}
