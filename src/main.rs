use slint::ComponentHandle;
use slint::LogicalPosition;
use std::path::PathBuf;
use rfd::FileDialog;

fn open_imgfile_dialog() -> PathBuf {
    let img_file = FileDialog::new().add_filter("图片 Image", &["jpg","png","svg"]).pick_file();
    match img_file {
        Some(file) => {
            return file;
        },
        None => {
            return open_imgfile_dialog();
        }
    }
}

slint::include_modules!();
fn main() {
    println!("OpenFrp Launcher!");
    let main = MainPage::new().unwrap();
    let handle4 = main.as_weak();
    main.global::<User>().on_rfd_load_img(move || {
        let main = handle4.upgrade().unwrap();
        let img = slint::Image::load_from_path(&open_imgfile_dialog()).unwrap();
        main.global::<User>().set_avatar(img)
    });

    let handle = main.as_weak();
    main.on_close_win(move || {
        handle.upgrade().unwrap().hide().unwrap();
    });

    let handle2 = main.as_weak();
    main.on_move_win(move |offset_x,offset_y| {
        let main = handle2.upgrade().unwrap();
        main.window().set_position(LogicalPosition::new(offset_x,offset_y));
    });

    let handle3 = main.as_weak();
    main.on_get_win_pos(move || {
        let main = handle3.upgrade().unwrap();
        let logical_pos = main.window().position().to_logical(main.window().scale_factor());
        (logical_pos.x,logical_pos.y)
    });

    main.on_dbug(move |val| {
        println!("{val}")
    });

    main.run().unwrap();
}
