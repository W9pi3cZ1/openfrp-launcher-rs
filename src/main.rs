use md5::{Digest, Md5};
use rfd::FileDialog;
use slint::LogicalPosition;
use std::path::PathBuf;

fn open_imgfile_dialog() -> PathBuf {
    let img_file = FileDialog::new()
        .add_filter("图片 Image", &["jpg", "png", "svg"])
        .pick_file();
    match img_file {
        Some(file) => {
            return file;
        }
        None => {
            return open_imgfile_dialog();
        }
    }
}

fn load_image_from_url<Url: reqwest::IntoUrl>(
    client: reqwest::blocking::Client,
    url: Url,
) -> Result<slint::SharedPixelBuffer<slint::Rgba8Pixel>, Box<dyn std::error::Error>> {
    let image_raw_data = client.get(url).send()?.bytes()?;
    let image_buffer = image::load_from_memory(&image_raw_data)?.into_rgba8();
    return Ok(slint::SharedPixelBuffer::clone_from_slice(
        &image_buffer.as_raw(),
        image_buffer.width(),
        image_buffer.height(),
    ));
}

fn md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn fresh_user_info(
    handle: slint::Weak<MainPage>,
    logged_in: bool,
    username: slint::SharedString,
    avatar_buffer: slint::SharedPixelBuffer<slint::Rgba8Pixel>,
) {
    handle.upgrade_in_event_loop(move |main| {
        main.global::<User>().set_logged_in(logged_in);
        main.global::<User>().set_name(username);
        main.global::<User>()
            .set_avatar(slint::Image::from_rgba8(avatar_buffer));
        main.global::<User>().set_logging_in(false);
    }).unwrap();
}

slint::include_modules!();
fn main() {
    println!("OpenFrp Launcher!");
    // let http_client = reqwest::blocking::Client::new();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let rt2 = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let main = MainPage::new().unwrap();
    let handle4 = main.as_weak();

    main.global::<User>().on_rfd_load_img(move || {
        let handle = handle4.clone();
        rt2.spawn_blocking(move ||{
            let img_path = open_imgfile_dialog();
            handle.upgrade_in_event_loop(move |main|{
                main.global::<User>().set_avatar(slint::Image::load_from_path(&img_path).unwrap());
            }).unwrap();
        });
    });
    let handle5 = main.as_weak();
    main.global::<User>().on_login(move |email, _password| {
        let handle = handle5.clone();
        let main = handle5.upgrade().unwrap();
        main.global::<User>().set_logging_in(true);
        rt.spawn_blocking(move || {
            let http_client = reqwest::blocking::Client::new();
            let email_md5: &str = &md5(&email.to_ascii_lowercase());
            let avatar = load_image_from_url(
                http_client.clone(),
                format!("https://cravatar.cn/avatar/{}?s=128", email_md5),
            )
            .unwrap();
            fresh_user_info(handle.clone(), true, "Unknown".into(), avatar);
        });
    });

    let handle = main.as_weak();
    main.on_close_win(move || {
        handle.upgrade().unwrap().hide().unwrap();
    });

    let handle2 = main.as_weak();
    main.on_move_win(move |offset_x, offset_y| {
        let main = handle2.upgrade().unwrap();
        main.window()
            .set_position(LogicalPosition::new(offset_x, offset_y));
        // println!("{:#?}",LogicalPosition::new(offset_x, offset_y));
    });

    let handle3 = main.as_weak();
    main.on_get_win_pos(move || {
        let main = handle3.upgrade().unwrap();
        let logical_pos = main
            .window()
            .position()
            .to_logical(main.window().scale_factor());
        // println!("{:#?}",logical_pos);
        (logical_pos.x, logical_pos.y)
    });

    main.on_dbug(move |val| println!("{val}"));

    main.run().unwrap();
}
