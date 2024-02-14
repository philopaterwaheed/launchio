use fltk::{app, prelude::*, window::Window,button::Button,output,*};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(1000,800)
        .center_screen()
        .with_label("app_luncher");

    let _but2 = Button::default()
    .with_pos(100, 10)
    .with_size(80, 40)
    .with_label("Button 2");

    let flex = group::Flex::default().with_size(400, 500).column().center_of_parent(); 
    let label = frame::Frame::default().with_label("Check this text:");
    let mut input = input::IntInput::default();
    let mut btn = button::Button::default().with_label("Submit");
    let mut output = output::Output::default();
    output.set_value("You can't edit this!");
    btn.set_callback(move |btn| {
        // input.set_readonly(true);
        output.set_value(&input.value()[..]);
    });
    flex.end();


    wind.end();
    wind.show();
    app.run().unwrap();
}
