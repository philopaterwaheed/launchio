use fltk::{prelude::*, *,enums::*,frame::Frame};
use std::env;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::os::unix::fs::PermissionsExt;
use rust_fuzzy_search::fuzzy_search_sorted;
use std::fs;
use dirs;

fn main() {
    let mut conf_dir= String::new(); 
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push("launchio");

        // Create the directory if it doesn't exist
        if let Err(err) = fs::create_dir_all(&config_dir) {
            eprintln!("Error creating config directory: {}", err);
        }
        else {
            conf_dir = config_dir.to_string_lossy().into_owned();
        }
    }


   let  executables:  Vec<String> = get_executables() ; // get all the exes in the system
   let mut output_contains : Vec<String> =vec![] ; 
   let mut index : i32 =1 ; 
   let app = app::App::default();
   // app::set_background_color(3,5,20);
   let mut wind = window::Window::default()
       .with_size(700,400)
       .center_screen()
       .with_label("app_luncher");

   let mut input = input::Input::default();

   let mut flex = group::Flex::default().with_size(601, 500).column().size_of_parent(); // the outter flex 
   let inside_flex = group::Flex::default().with_size(400, 400).row();


   if let  Ok(mut image) = image::JpegImage::load("./nar.jpg"){
       let mut frame = Frame::default().size_of_parent();
       frame.set_frame(FrameType::OFlatFrame); // Remove the frame around the image
       image.scale(600, 200, true, true);
       frame.set_image(Some(image));
   }

   
   let mut _app_name_label = frame::Frame::default().with_label("launchio\nby philo").set_color(Color::from_rgb(0,0,0));

   if let  Ok(mut imagee) = image::JpegImage::load("./sas.jpg"){
       let mut frame = Frame::default().with_size(560, 260).with_pos(0,0);
       frame.set_frame(FrameType::OFlatFrame); // Remove the frame around the image
       imagee.scale(400, 200, true, true);
       frame.set_image(Some(imagee));
   }


   inside_flex.end();



   flex.add_resizable(&input);
   flex.fixed(&mut input, 30);

   //note that the trigger should be only one 
   input.set_trigger(enums::CallbackTrigger::Changed|enums::CallbackTrigger::EnterKey);// it makes the input hijacks focus and 
   // input.set_trigger(enums::CallbackTrigger::Focus);

   let mut output = browser::SelectBrowser::new(10, 10, 900 - 20, 300 - 20, "");





   
   let mut  output_clone = output.clone();
   let _output_contains_clone = output_contains.clone();
   input.handle( move | _input , event| { // the call back when enter is pressed 
       // input.set_readonly(true);
       let key  = app::event_key() ;
       if key == enums::Key::Enter && event == Event::KeyDown {
           if output.size()> 0  {
           if let Some(text) = output.text(index as i32){
                // if let Ok(status) = run_executable(format!("{} &",text).as_str()) {
                    // println!("{}" , status);
                // }
                Command::new(text.as_str()).spawn();
           }
        }
       app.quit()
       }
       if key == enums::Key::Down && event == Event::KeyDown {
           if index <= output.size()-1 {
               index = index + 1 ; 
           }

       }

       else if key == enums::Key::Up && event == Event::KeyDown {
           if index >= 2 {
               index = index - 1 ; 
          }

       } 
      else if event == Event::KeyDown {
          index =1 ; 
      }
      output.select(index);
       false

   });
   input.set_callback(move |input| {
       output_contains.clear();
       output_clone.clear();
       output_contains = find_exe(&input.value(), &executables).clone();
       for out in &output_contains {
           output_clone.add(out);
       }
   });


   flex.end();

   wind.end();
   wind.show();
   app.run().unwrap();


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
    // for str in &executables
    // {
    //     println!("{}" ,str);
    // }
    executables

}
// Function to check if a file is executable
fn is_executable(path: &Path) -> bool {
    path.is_file() && path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

// Function to execute an executable by name


fn find_exe<'a>(s: &str, executables: &'a Vec<String>) -> Vec<String> {
    let list: Vec<&str> = executables.iter().map(|xx| xx.as_str()).collect();
    let res: Vec<(&str, f32)> = fuzzy_search_sorted(s, &list);
    res.iter().filter(|s| s.1 > 0.0).map(|s| s.0.to_string()).collect()
}
