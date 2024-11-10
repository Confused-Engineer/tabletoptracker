
use eframe::egui;
use crate::sidepanel_dice;
use davids_awesome_library::yaml::basic_game::{self, *};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TableTopTracker {

    show_console_panel: bool,
    show_dice_panel: bool,
    
    #[serde(skip)]
    history: davids_awesome_library::database::History,
    
    #[serde(skip)]
    show_full_history: bool,
    //#[serde(skip)]
    //show_game_wizard: bool,
    //#[serde(skip)]
    //show_new_player_wizard: bool,
    

    
    dice: DiceData,


    #[serde(skip)]
    gamedata: davids_awesome_library::yaml::basic_game::GameData,
    #[serde(skip)]
    game_instance:davids_awesome_library::yaml::basic_game::Game,
    #[serde(skip)]
    player_instance: davids_awesome_library::yaml::basic_game::Player,
    #[serde(skip)]
    shop_instance: davids_awesome_library::yaml::basic_game::Shop,

    #[serde(skip)]
    temp_player_stat: (String, String),
    #[serde(skip)]
    temp_player_inventory: (String, String),
    #[serde(skip)]
    temp_shop_inventory: (String, String),

    #[serde(skip)]
    temp_i64_val: i64,

  
    
}



impl Default for TableTopTracker {
    fn default() -> Self {
        Self {
            // Example stuff:

            
            show_console_panel: false,
            show_dice_panel: true,
            show_full_history: false,
            //show_game_wizard: false,
            //show_new_player_wizard: false,
            dice: DiceData::default(),

            history: davids_awesome_library::database::History::new(),


            //sqlite: std::sync::Mutex::new(rusqlite::Connection::open("./my_db.db3").unwrap()),

            gamedata: basic_game::GameData::init(),
            game_instance: basic_game::Game::new(),
            player_instance: basic_game::Player::new(),
            shop_instance: basic_game::Shop::new(),

            temp_player_stat: (String::new(), String::new()),
            temp_player_inventory: (String::new(), String::new()),
            temp_shop_inventory: (String::new(), String::new()),

            temp_i64_val: 0,

        }
    }
}

impl TableTopTracker {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn center_panel_player(&mut self, ui: &mut eframe::egui::Ui)
    {   
        ui.columns(4, |ui| {
            if self.player_instance.get_name().is_empty()
            {
                ui[0].heading(&format!("Select Player"));
            } else {
                ui[0].heading(&format!("Player: {}", self.player_instance.get_name()));
            }
            
            ui[2].add_sized([1.0, 20.0], egui::Separator::vertical(egui::Separator::default()));
            
            if self.shop_instance.get_name().is_empty()
            {
                ui[3].heading(&format!("Select Shop"));
            } else {
                ui[3].heading(&format!("Shop: {}", self.shop_instance.get_name()));
            }
            
        });
        
        ui.separator();
        ui.columns(4, |ui| {
            if !self.player_instance.get_name().is_empty()
            {
                ui[0].heading("Stats");
                ui[0].separator();
                egui::ScrollArea::vertical().id_source("p_stat").show(&mut ui[0], |ui| {
                    for (stat, value) in self.player_instance.get_stats()
                    {
                        ui.horizontal(|ui| {
                            //ui.label(stat);
                            ui.add_sized([100.0, 15.0], egui::Label::new(stat.clone()));
                            ui.label(":");
                            ui.add_sized([100.0, 15.0], egui::Label::new(value.clone()));
                            ui.menu_button("Edit", |ui| {
                                if ui.button("update").clicked()
                                {
                                    self.temp_player_stat.0 = stat.clone();
                                    ui.close_menu();
                                }
        
                                if ui.button("remove").clicked()
                                {
                                    self.player_instance.remove_stat(stat);
                                    self.game_instance.add_player(self.player_instance.clone());
                                    ui.close_menu();
                                }
                            });
                            ui.add_space(10.0);
                            
                            
                        });
        
                    }
        
                    ui.horizontal(|ui| {
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_player_stat.0));
                        ui.label(":");
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_player_stat.1));
                        if ui.button("Add").clicked()
                        {
                            self.player_instance.set_stat(self.temp_player_stat.0.clone(), self.temp_player_stat.1.clone());
                            self.game_instance.add_player(self.player_instance.clone());
                            self.temp_player_stat = (String::new(), String::new());
                        }
                    });
    
                });
            
            


                ui[1].heading("Inventory");
                ui[1].separator();
                
                egui::ScrollArea::vertical().id_source("p_inv").show(&mut ui[1], |ui| {
                    for (item, quantity) in self.player_instance.get_inventory()
                    {
                        ui.horizontal(|ui| {
                            ui.add_sized([100.0, 15.0], egui::Label::new(item.clone()));
                            ui.label(":");
                            ui.add_sized([100.0, 15.0], egui::Label::new(quantity.to_string()));
                            ui.menu_button("Edit", |ui| {
                                if ui.button("update").clicked()
                                {
                                    self.temp_player_inventory.0 = item.clone();
                                    ui.close_menu();
                                }
        
                                if ui.button("remove").clicked()
                                {
                                    self.player_instance.remove_item(item);
                                    self.game_instance.add_player(self.player_instance.clone());
                                    ui.close_menu();
                                }
                            });
                            ui.add_space(10.0);
        
                        });
                    }
        
                    ui.horizontal(|ui| {
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_player_inventory.0));
                        ui.label(":");
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_player_inventory.1));
                        self.temp_player_inventory.1 = self.temp_player_inventory.1.clone().chars().filter(|c| c.is_digit(10)).collect();
                        if ui.button("Add").clicked()
                        {
        
                            self.temp_i64_val = self.temp_player_inventory.1.clone().parse::<i64>().unwrap_or(0);
                            self.player_instance.set_item(self.temp_player_inventory.0.clone(), self.temp_i64_val.clone());
                            self.game_instance.add_player(self.player_instance.clone());
                            self.temp_player_inventory = (String::new(), String::new());
                        }
                    });
                });

            }

            ui[2].add(egui::Separator::vertical(egui::Separator::default()));

            if !self.shop_instance.get_name().is_empty()
            {
                ui[3].heading("Inventory");
                ui[3].separator();
                
                egui::ScrollArea::vertical().id_source("s_inv").show(&mut ui[3], |ui| {
                    for (item, quantity) in self.shop_instance.get_inventory()
                    {
                        ui.horizontal(|ui| {
                            ui.add_sized([100.0, 15.0], egui::Label::new(item.clone()));
                            ui.label(":");
                            ui.add_sized([100.0, 15.0], egui::Label::new(quantity.to_string()));
                            ui.menu_button("Edit", |ui| {
                                if ui.button("update").clicked()
                                {
                                    self.temp_shop_inventory.0 = item.clone();
                                    ui.close_menu();
                                }
        
                                if ui.button("remove").clicked()
                                {
                                    self.shop_instance.remove_item(item);
                                    self.game_instance.add_shop(self.shop_instance.clone());
                                    ui.close_menu();
                                }
                            });
        
                            
                            
                        });
                    }
        
                    ui.horizontal(|ui| {
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_shop_inventory.0));
                        ui.label(":");
                        ui.add_sized([100.0, 15.0], egui::text_edit::TextEdit::singleline(&mut self.temp_shop_inventory.1));
                        self.temp_shop_inventory.1 = self.temp_shop_inventory.1.clone().chars().filter(|c| c.is_digit(10)).collect();
                        if ui.button("Add").clicked()
                        {
        
                            self.temp_i64_val = self.temp_shop_inventory.1.clone().parse::<i64>().unwrap_or(0);
                            self.shop_instance.set_item(self.temp_shop_inventory.0.clone(), self.temp_i64_val.clone());
                            self.game_instance.add_shop(self.shop_instance.clone());
                            self.temp_shop_inventory = (String::new(), String::new());
                        }
                    });
                });
            }

        });
    }

}

impl eframe::App for TableTopTracker {

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {

                ui.menu_button("File", |ui| {
                    
                    if ui.button("Save").clicked() {
                        self.gamedata.save_data();
                    }

                    if ui.button("Save and Quit").clicked() {
                        self.gamedata.save_data();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    ui.add_space(10.0);

                    if ui.button("Quit without saving").clicked() {
                        self.gamedata.save_data();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Show", |ui| {
                    if ui.button("Console").clicked() {
                        self.show_console_panel =! self.show_console_panel;
                        ui.close_menu();
                    }

                    if ui.button("Dice").clicked() {
                        self.show_dice_panel =! self.show_dice_panel;
                        ui.close_menu();
                    }


                });

                ui.menu_button("History", |ui| {
                        
                    if ui.button("Show Full").clicked()
                    {
                        self.show_full_history = !self.show_full_history;
                        ui.close_menu();
                    }
                    
                    if ui.button("Clear").clicked()
                    {
                        self.history.clear_all();
                        ui.close_menu();
                    }
                });

                ui.add_space(20.0);

                ui.menu_button("Games", |ui|{
                    
                    for game in self.gamedata.game_list()
                    {
                        if ui.button(&game).clicked()
                        {
                            self.game_instance = self.gamedata.get_game(&game).unwrap();
                            self.player_instance = basic_game::Player::new();
                            self.shop_instance = basic_game::Shop::new();
                        }
                    }

                    if ui.button("New").clicked()
                    {
                        self.gamedata.save_data();


                        let _ = std::process::Command::new(std::env::current_exe().unwrap())
                            .arg("new")
                            .spawn()
                            .expect("Failed to run");

                        

                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Players", |ui|{
                    
                    for mut player in self.game_instance.get_players()
                    {
                        if ui.add_sized([80.0,10.0], egui::Button::new(&player.get_name())).clicked()
                        {
                            self.player_instance = self.game_instance.get_single_player(&player.get_name());
                            ui.close_menu();   
                        }
                    }
                });

                ui.menu_button("Shops", |ui|{
                    
                    for mut shop in self.game_instance.get_shops()
                    {
                        if ui.add_sized([80.0,10.0], egui::Button::new(&shop.get_name())).clicked()
                        {
                            self.shop_instance = self.game_instance.get_single_shop(&shop.get_name());
                            ui.close_menu();
                        }
                    }
                    
                });
                ui.add_space(16.0);

                ui.add_space(20.0);
                
                #[cfg(debug_assertions)]
                ui.menu_button("debug", |ui| {
                    if ui.button("Player").clicked()
                    {
                        println!("{:#?}", self.player_instance.get_instance());
                    }

                    if ui.button("Shop").clicked()
                    {
                        println!("{:#?}", self.shop_instance.get_instance());
                    }

                    if ui.button("Game").clicked()
                    {
                        println!("{:#?}", self.game_instance.get_instance());
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui|{
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });
        });

        egui::SidePanel::left("left panel").exact_width(170.0).resizable(false).show_animated(ctx, self.show_dice_panel, |ui|{
            
            sidepanel_dice::show(ui, ctx, &mut self.dice, &mut self.history);
            
        });

        egui::TopBottomPanel::bottom("bottom panel").resizable(true).max_height(500.0).min_height(100.0).show_animated(ctx, self.show_console_panel, |ui|{
            ui.heading("History");
            ui.separator();

            ui.vertical(|ui|{
                egui::ScrollArea::vertical().max_width(ui.available_width()).show(ui, |ui|{
                    ui.set_width(ui.available_width());
                    
                    for entry in self.history.get_history_last_x(100)
                    {
                        ui.label(entry);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut eframe::egui::Ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if self.game_instance.get_name().is_empty()
            {
                ui.heading("Please Select a Game to Continue");
                
            } else {
                ui.heading(self.game_instance.get_name());
                ui.separator();
    
                self.center_panel_player(ui);
            }

            
        });

        if self.show_full_history
        {
            let _ = std::process::Command::new(std::env::current_exe().unwrap())
                .arg("history")
                .spawn()
                .expect("program not found");
            self.show_full_history = false;
        }

        if !self.game_instance.get_name().is_empty()
        {
        self.gamedata.add_game(self.game_instance.clone());
        }
    }
}



#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct DiceData
{
    #[serde(skip)]
    pub(crate) d20: sidepanel_dice::DicePos,
    #[serde(skip)]
    pub(crate) d12: sidepanel_dice::DicePos,
    #[serde(skip)]
    pub(crate) d06: sidepanel_dice::DicePos,
    #[serde(skip)]
    pub(crate) custom: sidepanel_dice::DicePos,
    pub(crate) custom_die_max: i32,
    pub(crate) custom_die_min: i32,
}

impl Default for DiceData
{
    fn default() -> Self {
        Self {
            d20: sidepanel_dice::DicePos::new(),
            d12: sidepanel_dice::DicePos::new(),
            d06: sidepanel_dice::DicePos::new(),
            custom: sidepanel_dice::DicePos::new(),
            custom_die_max: 10,
            custom_die_min: 1,
        }
    }
}


