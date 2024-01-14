use slint::ComponentHandle;
use slint::LogicalPosition;

slint::include_modules!();
fn main() {
    println!("OpenFrp Launcher!");
    let main = MainPage::new().unwrap();
    
    let handle = main.as_weak();
    main.on_close_win(move || {
        handle.upgrade().unwrap().hide().unwrap();
    });

    let handle2 = main.as_weak();
    main.on_move_win(move |offset_x,offset_y| {
        let main = handle2.upgrade().unwrap();
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x,logical_pos.y + offset_y));
    });

    main.run().unwrap();
}
