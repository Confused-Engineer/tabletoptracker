pub struct Screen
{
    i: i64
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            i: 0
        }
    }
}

impl Screen {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        
        Default::default()
    }
}

impl  eframe::App for Screen {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        eframe::egui::CentralPanel::frame(eframe::egui::CentralPanel::default(), eframe::egui::Frame::none()).show(ctx, |ui| {
            ctx.request_repaint();
            ui.visuals_mut().window_fill = eframe::egui::Color32::TRANSPARENT;
            ui.visuals_mut().panel_fill = eframe::egui::Color32::TRANSPARENT;
            ui.visuals_mut().faint_bg_color = eframe::egui::Color32::TRANSPARENT;
            let rect = ctx.input(|i: &eframe::egui::InputState| i.screen_rect());
        
           
            
            eframe::egui::Image::new(eframe::egui::include_image!("../assets/load.png"))
            //.bg_fill(eframe::egui::Color32::TRANSPARENT)
            .paint_at(ui, rect);

            std::thread::sleep(std::time::Duration::from_millis(10));
            self.i += 1;
            if self.i == 200
            {
    
                ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Close);
                ctx.request_repaint();
            }

            
            
            
            
        });
    
    }
}