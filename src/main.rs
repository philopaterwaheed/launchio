use fltk::{prelude::*, *,enums::*};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(1000,300)
        .center_screen()
        .with_label("app_luncher");


    let flex = group::Flex::default().with_size(400, 500).column().size_of_parent(); 
    let label = frame::Frame::default().with_label("Check this text:");
    let mut input = input::Input::default();
    
    //note that the trigger should be only one 
    input.set_trigger(enums::CallbackTrigger::Changed|enums::CallbackTrigger::EnterKey);// it makes the input hijacks focus and 
    // input.set_trigger(enums::CallbackTrigger::Focus);

    let mut output = output::Output::default();
    output.set_value("You can't edit this!");



    
    let mut  output_clone = output.clone();
    input.handle( move | input , event| { // the call back when enter is pressed 
        // input.set_readonly(true);
        let key  = app::event_key() ;
        if key == enums::Key::Enter && event == Event::KeyDown {
            println!("{}" , event);
         }
        else {
        }

        false

    });
    // input.set_callback( move | input| { // the call back when enter is pressed 
    //     // input.set_readonly(true);
    //     output.set_value("philo");
    //
    // });


    flex.end();


    wind.end();
    wind.show();
    app.run().unwrap();
}
