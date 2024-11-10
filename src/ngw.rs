use eframe::egui;
use davids_awesome_library::yaml::basic_game::{self, *};



pub struct GameWizard {
    // Example stuff:

    state: AppState,
    sub_state: SubState,

    gamedata: basic_game::GameData,
    game_instance: basic_game::Game,
    player_instance: basic_game::Player,
    shop_instance: basic_game::Shop,

    game_name: String,
    player_name: String,
    shop_name: String,

    stat_key: String,
    stat_value: String,
    inv_key: String,
    inv_value: String,
    inv_count: i64,

}

impl Default for GameWizard {
    fn default() -> Self {
        Self {

            state: AppState::Default,
            sub_state: SubState::Player,

            gamedata: basic_game::GameData::init(),
            game_instance: basic_game::Game::new(),
            player_instance: basic_game::Player::new(),
            shop_instance: basic_game::Shop::new(),

            game_name: String::new(),
            player_name: String::new(),
            shop_name: String::new(),
            stat_key: String::new(),
            stat_value: String::new(),
            inv_key: String::new(),
            inv_value: String::new(),
            inv_count: 1,

        }
    }
}

impl GameWizard {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.


        Default::default()
    }


    
    fn player_reset(&mut self)
    {
        self.player_instance = basic_game::Player::new();
        self.player_name = String::new();
        self.stat_key = String::new();
        self.stat_value = String::new();
        self.inv_key = String::new();
        self.inv_value = String::new();

    }

    fn shop_reset(&mut self)
    {
        self.shop_instance = basic_game::Shop::new();
        self.shop_name = String::new();
        self.stat_key = String::new();
        self.stat_value = String::new();
        self.inv_key = String::new();
        self.inv_value = String::new();

    }

    fn welcome(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Welcome to the Game Manager");
        ui.label("This is where you can create, edit, and delete games.");
        ui.separator();
        ui.heading("Usage");
        ui.label("Select 'New' on the left to make a new game, or select an existing game to make any changes to qualities including \nGame, Player, Shop Names. Duplicate or Delete Games.\nWhen you are done make sure to save before closing or else risk losing data.");
        ui.separator();
        ui.heading("Example");
        ui.label("1) Create a game called default (or whatever you want) with a set of players and shops, save it. Then copy it and rename it accordingly when you want to make a new game. This will allow you to save time during character and shop creation when you want to make a new game.");
        ui.label("2) If you made a game and then a new player wants to join later, edit the game to add the new player");
        ui.label("3) Or do whatever tf you want I can't stop you at this point");

    }

    fn error(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Rut Ro Raggy, An Error Occured");
        if ui.button("Click To Return to Main Menu").clicked()
        {
            self.state = AppState::Default;
            self.sub_state = SubState::Player;
        }
    }

    fn new_player_template(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("New Game Template");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Provide a Unique Name for your Game:");
            ui.text_edit_singleline(&mut self.game_name);
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Provide a Player Name:");
            ui.text_edit_singleline(&mut self.player_name);
        });
        ui.separator();
        
        ui.columns(2, |ui| {
            ui[0].label("Create stats for your player in the pattern:\nStat Name: Value. (e.g. Health: 20)");
            ui[0].horizontal(|ui| {
                ui.label("Stat:");
                ui.add_sized([150.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.stat_key));
                ui.label(":");
                ui.add_sized([75.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.stat_value));
                if ui.button("Add").clicked()
                {
                    self.player_instance.set_stat(self.stat_key.clone(), self.stat_value.clone());
                    self.stat_key = String::new();
                    self.stat_value = String::new();
                }
            });

            ui[1].label("Add inventory items to your player in the pattern:\nItem: Quantity");
            ui[1].horizontal(|ui| {
                ui.label("Item:");
                ui.add_sized([150.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.inv_key));
                ui.label(":");
                ui.add_sized([50.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.inv_value));
                self.inv_value = self.inv_value.clone().chars().filter(|c| c.is_digit(10)).collect();
                if ui.button("Add").clicked()
                {
                    
                    
                    
                    self.inv_count = self.inv_value.clone().parse::<i64>().unwrap_or(0);
                    
                    self.player_instance.set_item(self.inv_key.clone(), self.inv_count.clone());

                    self.inv_key = String::new();
                    self.inv_value = String::new();
                }
            });
        });
        

        ui.separator();

        
        
        ui.columns(2, |ui| {
            ui[0].heading("Current Stats");
            for (key, val) in self.player_instance.get_stats()
            {
                ui[0].horizontal(|ui|{
                    ui.label(key.clone());
                    ui.label(":");
                    ui.label(val);
                    if ui.button("Remove").clicked()
                    {
                        self.player_instance.remove_stat(key);
                    }
                });
            }

            ui[1].heading("Current Inventory");
            for (key, val) in self.player_instance.get_inventory()
            {
                ui[1].horizontal(|ui|{
                    ui.label(key.clone());
                    ui.label(":");
                    ui.label(val.to_string());
                    if ui.button("Remove").clicked()
                    {
                        self.player_instance.remove_item(key);
                    }
                });
            }
        });
        
        
    }


    fn new_shop_template(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("New Game Template");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Provide a Unique Name for your Game:");
            ui.text_edit_singleline(&mut self.game_name);
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Provide a Shop Name:");
            ui.text_edit_singleline(&mut self.shop_name);
        });
        ui.separator();


        ui.label("Add inventory items to your Shop in the pattern:\nItem: Quantity");
            ui.horizontal(|ui| {
                ui.label("Item:");
                ui.add_sized([150.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.inv_key));
                ui.label(":");
                ui.add_sized([50.0, 17.0], egui::widgets::TextEdit::singleline(&mut self.inv_value));
                self.inv_value = self.inv_value.clone().chars().filter(|c| c.is_digit(10)).collect();
                if ui.button("Add").clicked()
                {
                    

                    self.inv_count = self.inv_value.clone().parse::<i64>().unwrap_or(0);
                    
                    self.shop_instance.set_item(self.inv_key.clone(), self.inv_count.clone());

                    self.inv_key = String::new();
                    self.inv_value = String::new();
                }
            });

            ui.separator();


        ui.heading("Current Inventory");
            for (key, val) in self.shop_instance.get_inventory()
            {
                ui.horizontal(|ui|{
                    ui.label(key.clone());
                    ui.label(":");
                    ui.label(val.to_string());
                    if ui.button("Remove").clicked()
                    {
                        self.shop_instance.remove_item(key);
                    }
                });
            }
    }

    fn edit_game_undetermined(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Pick an action Edit, Duplicate or Delete");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.add_sized([60.0,25.0], egui::widgets::Button::new("Create New From")).clicked()
            {
                self.state = AppState::NewGame;
                self.sub_state = SubState::Player;
            }
            if ui.add_sized([60.0,25.0], egui::widgets::Button::new("Duplicate as-is")).clicked()
            {
                if !self.game_name.is_empty()
                {
                    let mut temp = self.game_instance.clone();
                    temp.set_name(&self.game_name);
                    self.gamedata.add_game(temp);
                    self.game_instance = basic_game::Game::new();
                    self.game_name = String::new();
                    self.state = AppState::Default;
                    self.sub_state = SubState::Player;
                }
                

            }
            if ui.add_sized([60.0,25.0], egui::widgets::Button::new("Delete")).clicked()
            {
                self.gamedata.remove_game(&self.game_instance.get_name());
                self.game_instance = basic_game::Game::new();
                self.state = AppState::Default;
                self.sub_state = SubState::Player;
            }
        });

        ui.label("If using 'Duplicate as-is', provide a new name:");
        ui.text_edit_singleline(&mut self.game_name);
        ui.separator();
        ui.label("Note: When Using 'Create New From' if you keep the name the same as the original you will effectively edit the game instead of making a new one, but providing a unique name will generate a new game.");
    }


    fn bottom_panel_new_player(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            
            if self.player_name.is_empty() && !self.game_instance.get_players().is_empty()
            {
                if ui.add_sized([60.0,25.0], egui::widgets::Button::new("Continue to Shops >")).clicked()
                {
                    self.state = AppState::NewGame;
                    self.sub_state = SubState::Shop;
                }
            }

            if !self.player_name.is_empty()
            {
                
                
                if ui.add_sized([40.0,25.0], egui::widgets::Button::new("Save and Next Player >")).clicked()
                {
                    self.player_instance.set_name(&self.player_name.clone());
                    self.game_instance.add_player(self.player_instance.clone());
                    
                    self.player_reset();

                }
            }
        });
    }

    fn bottom_panel_default(&mut self, ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context)
    {
        ui.horizontal(|ui| {

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                if ui.add_sized([40.0,25.0], egui::widgets::Button::new("Exit W/O Saving")).clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });


            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.add_sized([40.0,25.0], egui::widgets::Button::new("Save and Exit")).clicked()
                {
                    self.gamedata.save_data();
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ui.add_sized([40.0,25.0], egui::widgets::Button::new("Save")).clicked()
                {
                    self.gamedata.save_data();
                }


            });
        });
    }

    

    fn bottom_panel_new_shop(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            
            if self.shop_name.is_empty() && !self.game_instance.get_shops().is_empty() && !self.game_name.is_empty()
            {
                if ui.add_sized([60.0,25.0], egui::widgets::Button::new("Finish >")).clicked()
                {
                    self.game_instance.set_name(&self.game_name);
                    self.gamedata.add_game(self.game_instance.clone());
                    self.game_instance = basic_game::Game::new();
                    self.state = AppState::Default;
                    self.sub_state = SubState::Player;
                }
            }

            if !self.shop_name.is_empty()
            {
                
                
                if ui.add_sized([40.0,25.0], egui::widgets::Button::new("Save and Next Shop >")).clicked()
                {
                    self.shop_instance.set_name(&self.shop_name.clone());
                    self.game_instance.add_shop(self.shop_instance.clone());
                    
                    self.shop_reset();

                }
            }
        });
    }

    fn bottom_panel_edit(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            
            
            if ui.add_sized([60.0,25.0], egui::widgets::Button::new("< Back")).clicked()
            {
                self.game_instance = basic_game::Game::new();
                self.state = AppState::Default;
                self.sub_state = SubState::Player;
            }
        });
    }

    fn side_panel_default(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Current Game List");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for game in self.gamedata.game_list()
            {
                if ui.add_sized([ui.available_width(),50.0], egui::widgets::Button::new(game.clone())).clicked()
                {
                    let temp = self.gamedata.get_game(&game);
                    if temp.is_err()
                    {
                        self.state = AppState::Error;
                    }else {
                        self.game_instance = temp.unwrap();
                        self.state = AppState::EditGame;
                        self.sub_state = SubState::Undetermined;
                    }

                }
                
                
                
            }

            

            if ui.add_sized([ui.available_width(),50.0], egui::widgets::Button::new("New")).clicked()
            {
                self.state = AppState::NewGame;
                self.sub_state = SubState::Player;
            }
        });
    }

    fn side_panel_new_game(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Current Players");
        for mut player in self.game_instance.get_players()
        {

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(player.get_name());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Remove").clicked()
                    {
                        self.game_instance.remove_player(&player.get_name());
                    }
                });
            });
            
        }
        ui.separator();
        ui.heading("Current Shops");
        for mut shop in self.game_instance.get_shops()
        {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(shop.get_name());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Remove").clicked()
                    {
                        self.game_instance.remove_shop(&shop.get_name());
                    }
                });
            });
        }
    }

    fn side_panel_player_edit(&mut self, ui: &mut eframe::egui::Ui)
    {
        ui.heading("Current Game");
        ui.label(self.game_instance.get_name());
        ui.separator();
        ui.heading("Existing Games");
        for game in self.gamedata.game_list()
        {
            ui.label(&game);
        }

    }

}

impl eframe::App for GameWizard {
    /// Called by the frame work to save state before shutdown.


    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(debug_assertions)]
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            
                if ui.button("print logs").clicked()
                {
                    println!("{:#?}", self.game_instance);
                }

                if ui.button("Trigger Error").clicked()
                {
                    self.state = AppState::Error;
                }

            });
        });
        
        egui::SidePanel::left("leftpanel").exact_width(175.0).show(ctx, |ui|{

            if self.state == AppState::Default
            {
                self.side_panel_default(ui);
            }

            if self.state == AppState::NewGame
            {
                self.side_panel_new_game(ui);
            }


            if self.state == AppState::EditGame
            {
                self.side_panel_player_edit(ui);
            }

            

            
        });
                

        
            egui::TopBottomPanel::bottom("bottom").default_height(40.0).exact_height(40.0).show(ctx, |ui| {
                ui.add_space(7.5);
                
                if self.state == AppState::Default
                {
                    self.bottom_panel_default(ui, ctx);
                }


                if self.state == AppState::NewGame
                {

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            if ui.add_sized([40.0,25.0], egui::widgets::Button::new("< Exit w/o Save")).clicked()
                            {
                                self.state = AppState::Default;
                                self.sub_state = SubState::Player;
                            }
                        });
        
        
                        if self.sub_state == SubState::Player
                        {
                            self.bottom_panel_new_player(ui);
                        }

                        if self.sub_state == SubState::Shop
                        {
                            self.bottom_panel_new_shop(ui);
                        }
                    });

                }

                if self.state == AppState::EditGame
                {
                    self.bottom_panel_edit(ui);
                }
            });
        



        egui::CentralPanel::default().show(ctx, |ui| {


            if self.state == AppState::Default
            {
                self.welcome(ui);
            }

            if self.state == AppState::NewGame
            {
                if self.sub_state == SubState::Player
                {
                    self.new_player_template(ui);
                }

                if self.sub_state == SubState::Shop
                {
                    self.new_shop_template(ui);
                }
                
            }   

            if self.state == AppState::EditGame
            {
                if self.sub_state == SubState::Undetermined
                {
                    self.edit_game_undetermined(ui);
                }

                if self.sub_state == SubState::Player
                {

                }
            } 

            if self.state == AppState::Error
            {
                self.error(ui);
            }




            
        
        });

        ctx.request_repaint();
    }
}


#[derive(PartialEq)]
enum AppState {
    Default,
    NewGame,
    EditGame,
    Error,


}

#[derive(PartialEq)]
enum SubState
{
    Player,
    Shop,
    Undetermined,

}