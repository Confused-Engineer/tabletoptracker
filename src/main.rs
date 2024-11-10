#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui::{self, IconData};
const SIZE: [f32; 2] = [1200.0, 1050.0];
const NGW_SIZE: [f32; 2] = [850.0, 700.0];
const YAML_FILE: &str = "gamedata.yaml";



fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).



    let icon = eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..]).unwrap();
    
    let input: Vec<String> = std::env::args().collect();

    if input.contains(&String::from("new"))
    {
        let _ = new_game_wizard(icon.clone());
        let result = primary(icon.clone());
        return result;
    }

    if input.contains(&String::from("history"))
    {
        let result = history(icon.clone());
        return result;
    }

    if input.contains(&String::from("game"))
    {
        let result = primary(icon.clone());
        return result;
    }


    let result = init(icon.clone());

    let path_str = format!("{}/.tabletop/{}",davids_awesome_library::env::get_home().unwrap(), YAML_FILE);
    let path = std::path::Path::new(&path_str);

    if !path.exists()
    {
        let _ = new_game_wizard(icon.clone());
    }

    let path = std::path::Path::new(&path_str);
    if path.exists()
    {
        
        let result = primary(icon.clone());
        return result;
    } 


    result

}


fn primary(icon: IconData) -> eframe::Result
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_fullscreen(true)
        .with_min_inner_size(SIZE)
        .with_icon(
            // NOTE: Adding an icon is optional
            icon.clone(),
        ),
        ..Default::default()
        
    };
    
    eframe::run_native(
        "Table Top Tracker",
        options,
        Box::new(|cc| 
            {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Ok(Box::new(tabletoptracker::TableTopTracker::new(cc)))
            }),
    )
}

fn new_game_wizard(icon: IconData) -> eframe::Result
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size(NGW_SIZE)
        .with_min_inner_size(NGW_SIZE)
        .with_icon(
            // NOTE: Adding an icon is optional
            icon.clone(),
        ),
        ..Default::default()
        
    };


    eframe::run_native(
        "Game Manager",
        options,
        Box::new(|cc| 
            {
                
                Ok(Box::new(tabletoptracker::GameWizard::new(cc)))
            }),
    )
}

fn init(icon: IconData) -> eframe::Result
{

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        //.with_always_on_top()
        .with_fullscreen(true)
        .with_transparent(true)
        .with_decorations(false)
        .with_icon(
            // NOTE: Adding an icon is optional
            icon,
        ),
        ..Default::default()
        
    };


eframe::run_native("init",
    options, 
    Box::new(|cc| {

        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::new(tabletoptracker::Screen::new(cc)))
    }))
}


fn history(icon: IconData) -> eframe::Result {


    let size = [500.0, 700.0];

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_icon(
            // NOTE: Adding an icon is optional
            icon,
        ),
        ..Default::default()
        
    };


    eframe::run_native(
        "Full History",
        options,
        Box::new(|cc| 
            {
                
                Ok(Box::new(tabletoptracker::History::new(cc)))
            }),
    )

}