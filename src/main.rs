slint::include_modules!();
fn main() {
    println!("OpenFrp Launcher!");
    let main = MainPage::new().unwrap();
    let weak = main.as_weak();
    main.run().unwrap();
}
