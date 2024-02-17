use fltk::{prelude::*, *,enums::*,frame::Frame};
use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::os::unix::fs::PermissionsExt;



fn main() {

    let mut executables:Vec<String> = get_executables() ; // get all the exes in the system
    let app = app::App::default();
    // app::set_background_color(3,5,20);
    let mut wind = window::Window::default()
        .with_size(700,400)
        .center_screen()
        .with_label("app_luncher");

    let mut input = input::Input::default();
    input.set_size(50, 50);

    let mut flex = group::Flex::default().with_size(601, 500).column().size_of_parent(); // the outter flex 
    let inside_flex = group::Flex::default().with_size(400, 400).row();


    let mut image = image::JpegImage::load("./nar.jpg").unwrap();
    let mut imagee = image::JpegImage::load("./sas.jpg").unwrap();

    let mut frame = Frame::default().size_of_parent();
    frame.set_frame(FrameType::OFlatFrame); // Remove the frame around the image
    image.scale(600, 200, true, true);
    frame.set_image(Some(image));
    
    let mut _app_name_label = frame::Frame::default().with_label("launchio").set_color(Color::from_rgb(0,0,0));

    let mut frame = Frame::default().with_size(560, 260).with_pos(0,0);
    frame.set_frame(FrameType::OFlatFrame); // Remove the frame around the image
    imagee.scale(400, 200, true, true);
    frame.set_image(Some(imagee));


    inside_flex.end();



    flex.add_resizable(&input);
    flex.fixed(&mut input, 30);
    input.set_size(601,50);
    flex.recalc();
    flex.layout();

    //note that the trigger should be only one 
    input.set_trigger(enums::CallbackTrigger::Changed|enums::CallbackTrigger::EnterKey);// it makes the input hijacks focus and 
    // input.set_trigger(enums::CallbackTrigger::Focus);

    let mut output = browser::SelectBrowser::new(10, 10, 900 - 20, 300 - 20, "");

//    output.set_value("You can't edit this!");
    output.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
    output.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
    output.select(2);




    
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

       // Get the value of the PATH environment variable
}
fn get_executables ()-> Vec<String> {
    let mut executables:Vec<String> = Vec :: new () ; 
    if let Some(paths) = env::var_os("PATH") {// get the path from the var PATH
        // Split the PATH into individual directories
        for path in env::split_paths(&paths) {
            // Iterate over files in the directory
            if let Ok(entries) = path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        // Check if the entry is an executable file
                        if is_executable(&path) {
                            if let Some(name) = path.file_name() {
                                 if let Some(name) = name.to_str() {
                                     //if let Ok(status) = run_executable(name) {
                                            executables.push(name.to_string());
                                     //}
                                 }
                            }
                        }
                    }
                }
            }
        }
    } 
    for str in &executables
    {
        println!("{}" ,str);
    }
    executables

}
// Function to check if a file is executable
fn is_executable(path: &Path) -> bool {
    path.is_file() && path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

// Function to execute an executable by name
fn run_executable(name: &str) -> Result<ExitStatus, std::io::Error> {
   Command::new(name).status()
}
