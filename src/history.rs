use eframe::egui;



pub struct History {
    // Example stuff:

    history: davids_awesome_library::database::History
}

impl Default for History {
    fn default() -> Self {
        Self {
            history: davids_awesome_library::database::History::new()
        }
    }
}

impl History {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.


        Default::default()
    }
}

impl eframe::App for History {
    /// Called by the frame work to save state before shutdown.


    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
                
                
        
            ui.heading("History");
            ui.separator();

            ui.vertical(|ui|{
                egui::ScrollArea::vertical().max_width(ui.available_width()).show(ui, |ui|{
                    ui.set_width(ui.available_width());
                    for entry in self.history.get_history_full()
                    {
                        ui.label(entry);
                    }
                    
                });
            });
        
        
        });

        ctx.request_repaint();
    }
}

