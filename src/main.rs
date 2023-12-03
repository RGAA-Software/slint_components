
slint::include_modules!();
use slint::*;

pub fn main() {

    let window = AppWindow::new().unwrap();
    // window.on_btn_clicked(|name: slint::SharedString| {
    //     println!("Clicked: {}", name);
    // });

    // window.set_item_name(slint::SharedString::from("auth"));

    let _ = window.as_weak().upgrade_in_event_loop(move |w| {
        // w.global::<AppInfo>().set_version(slint::SharedString::from(configs.app_info.version));
        // w.global::<Settings>().set_version(slint::SharedString::from("V:1.0.0"));
    });

    let _ = window.as_weak().upgrade_in_event_loop(|w| {
        // w.global::<Settings>().on_value_changed(|k: slint::SharedString, v: slint::SharedString| {
        //     println!("k: {}, v: {}", k, v);
        // });
    });

    window.run().unwrap();

}