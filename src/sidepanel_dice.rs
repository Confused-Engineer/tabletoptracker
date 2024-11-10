use eframe::egui::{self, Pos2};
use rand::Rng;

pub fn show(ui: &mut eframe::egui::Ui, ctx: &eframe::egui::Context, dice: &mut crate::app::DiceData, history: &mut davids_awesome_library::database::History)
{
    let d20_image = egui::include_image!("../assets/D20.png");
    let mut rng = rand::thread_rng();
    let dice_button_size: eframe::egui::Vec2 = [150.0,150.0].into();

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui|{
        ui.heading("D20");
        
        let d20_button = ui.add_sized(dice_button_size, egui::ImageButton::new(d20_image.clone())
        .uv(dice.d20.die_pos)
        .frame(false)
        .rounding(10.0));
        if d20_button.clicked()
        {
            dice.d20.result = rng.gen_range(1..=20);
            dice.d20.animate = true;

            history.add_entry(&format!("Rolled: {} on a D20", dice.d20.result));
            
        }  

        ui.separator();
        ui.heading("D12");
        if ui.add_sized(dice_button_size, egui::ImageButton::new(d20_image.clone())
            .uv(dice.d12.die_pos)
            .frame(false)
            .rounding(10.0)   
        ).clicked()
        {

            
            dice.d12.result = rng.gen_range(1..=12);
            dice.d12.animate = true;

            history.add_entry(&format!("Rolled: {} on a D12", dice.d12.result));
        }

        ui.separator();
        ui.heading("D06");
        if ui.add_sized(dice_button_size, egui::ImageButton::new(d20_image.clone())
            .uv(dice.d06.die_pos)
            .frame(false)
            .rounding(10.0)   
        ).clicked()
        {
            dice.d06.result = rng.gen_range(1..=6);
            dice.d06.animate = true;

            history.add_entry(&format!("Rolled: {} on a D06", dice.d06.result));
        }   

        ui.separator();
        ui.heading("Custom");

        ui.columns(4, |ui| {
            
            let max_down = ui[0].small_button("-");
            if max_down.clicked()
            {
                if ((dice.custom_die_max - 1) == 0) || ((dice.custom_die_max - 1) <= dice.custom_die_min)
                {
                    dice.custom_die_max = dice.custom_die_min + 1
                } else {
                    dice.custom_die_max -= 1;
                }
            } else if max_down.clicked_by(egui::PointerButton::Secondary) {
                if ((dice.custom_die_max - 10) == 0) || ((dice.custom_die_max - 10) <= dice.custom_die_min)
                {
                    dice.custom_die_max = dice.custom_die_min + 1
                } else {
                    dice.custom_die_max -= 10;
                }
            }

            ui[1].label(eframe::egui::RichText::new("Max:"));
            ui[2].label(eframe::egui::RichText::new(&format!("{}", dice.custom_die_max)));

            let max_up = ui[3].small_button("+");
            if max_up.clicked()
            {
                dice.custom_die_max += 1;
            } else if max_up.clicked_by(egui::PointerButton::Secondary) {
                dice.custom_die_max += 10;
            }


            let min_down = ui[0].small_button("-");
            if min_down.clicked()
            {
                if (dice.custom_die_min - 1) == -1
                {
                    dice.custom_die_min = 0;
                } else {
                    dice.custom_die_min -= 1;
                }
            } else if min_down.clicked_by(egui::PointerButton::Secondary) {
                if (dice.custom_die_min - 10) <= -1
                {
                    dice.custom_die_min = 0;
                } else {
                    dice.custom_die_min -= 10;
                }
            }

            ui[1].label(eframe::egui::RichText::new("Min:"));
            ui[2].label(eframe::egui::RichText::new(&format!("{}", dice.custom_die_min)));

            let min_up = ui[3].small_button("+");
            if min_up.clicked()
            {
                if (dice.custom_die_min + 1) >= dice.custom_die_max
                {
                    dice.custom_die_min = dice.custom_die_max - 1;
                } else {
                    dice.custom_die_min += 1;
                }
            } else if min_up.clicked_by(egui::PointerButton::Secondary) {
                if (dice.custom_die_min + 10) >= dice.custom_die_max
                {
                    dice.custom_die_min = dice.custom_die_max - 1;
                } else {
                    dice.custom_die_min += 10;
                }
            }

            
        });

        ui.columns(2, |ui| {

            
            ui[0].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui|{
                ui.label("Results:");
            });
            
            ui[1].label(format!("{}", dice.custom.result));
        });


        


        if ui.add_sized(dice_button_size, egui::ImageButton::new(d20_image.clone())
            .uv(dice.custom.die_pos)
            .frame(false)
            .rounding(10.0)   
        ).clicked()
        {
            dice.custom.result = rng.gen_range(dice.custom_die_min..=dice.custom_die_max);
            dice.custom.animate = true;

            history.add_entry(&format!("Rolled: {} on Custom | Min: {}, Max: {}", dice.custom.result, dice.custom_die_min, dice.custom_die_max));
        }  
    
    });
    

    /*
    if self.d20_animate
    {
        self.d20_pos.shuffle();

        ctx.request_repaint()
    } */

    if dice.d20.animate
    {
        dice.d20.shuffle();

        
    }

    if dice.d12.animate
    {
        dice.d12.shuffle();

    }

    if dice.d06.animate
    {
        dice.d06.shuffle();

    }

    if dice.custom.animate
    {
        dice.custom.shuffle();
    }

    if dice.d20.animate || dice.d12.animate || dice.d06.animate || dice.custom.animate
    {
        ctx.request_repaint()
    }
}









enum AnimationStage
{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Result
}
pub(crate) struct DicePos
{
    die_pos: egui::Rect,
    animate: bool,
    animation_stage: AnimationStage,
    result: i32
}

impl DicePos {
    pub fn new() -> Self
    {
        DicePos { 
            die_pos: egui::Rect::from_two_pos(Self::POS01.0, Self::POS01.1 ),
            animate: false,
            animation_stage: AnimationStage::One,
            result: 0
        }
    }

    fn shuffle_time()
    {
        std::thread::sleep(std::time::Duration::from_millis(75));
    }

    fn shuffle(&mut self) -> &mut Self
    {
        match self.animation_stage {
            AnimationStage::One => {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE01.0, Self::POS_SHUFFLE01.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Two;
            },
            AnimationStage::Two => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE02.0, Self::POS_SHUFFLE02.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Three;
            },
            AnimationStage::Three => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE03.0, Self::POS_SHUFFLE03.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Four;
            },
            AnimationStage::Four => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE04.0, Self::POS_SHUFFLE04.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Five;
            },
            AnimationStage::Five => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE05.0, Self::POS_SHUFFLE05.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Six;
            },
            AnimationStage::Six => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE01.0, Self::POS_SHUFFLE01.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Seven;
            },
            AnimationStage::Seven => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE02.0, Self::POS_SHUFFLE02.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Eight;
            },
            AnimationStage::Eight => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE03.0, Self::POS_SHUFFLE03.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Nine;
            },
            AnimationStage::Nine => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE04.0, Self::POS_SHUFFLE04.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Ten;
            },
            AnimationStage::Ten => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE05.0, Self::POS_SHUFFLE05.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Eleven;
            },
            AnimationStage::Eleven => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE01.0, Self::POS_SHUFFLE01.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Twelve;
            },
            AnimationStage::Twelve => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE02.0, Self::POS_SHUFFLE02.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Thirteen;
            },
            AnimationStage::Thirteen => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE03.0, Self::POS_SHUFFLE03.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Fourteen;
            },
            AnimationStage::Fourteen => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE04.0, Self::POS_SHUFFLE04.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Fifteen;
            },
            AnimationStage::Fifteen => 
            {
                self.die_pos = egui::Rect::from_two_pos(Self::POS_SHUFFLE05.0, Self::POS_SHUFFLE05.1 );
                Self::shuffle_time();
                self.animation_stage = AnimationStage::Result;
            },
            AnimationStage::Result => 
            {
                self.result_match();
                self.animation_stage = AnimationStage::One;
                self.animate = false;
            },
        }
        self
    }

    fn result_match(&mut self) -> &mut Self
    {
        match self.result {
          1 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS01.0, Self::POS01.1 );
          },  
          2 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS02.0, Self::POS02.1 );
          }, 
          3 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS03.0, Self::POS03.1 );
          }, 
          4 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS04.0, Self::POS04.1 );
          }, 
          5 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS05.0, Self::POS05.1 );
          }, 
          6 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS06.0, Self::POS06.1 );
          }, 
          7 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS07.0, Self::POS07.1 );
          }, 
          8 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS08.0, Self::POS08.1 );
          }, 
          9 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS09.0, Self::POS09.1 );
          }, 
          10 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS10.0, Self::POS10.1 );
          }, 
          11 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS11.0, Self::POS11.1 );
          }, 
          12 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS12.0, Self::POS12.1 );
          }, 
          13 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS13.0, Self::POS13.1 );
          }, 
          14 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS14.0, Self::POS14.1 );
          }, 
          15 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS15.0, Self::POS15.1 );
          }, 
          16 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS16.0, Self::POS16.1 );
          }, 
          17 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS17.0, Self::POS17.1 );
          }, 
          18 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS18.0, Self::POS18.1 );
          }, 
          19 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS19.0, Self::POS19.1 );
          }, 
          20 => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS20.0, Self::POS20.1 );
          }, 
          _ => {
            self.die_pos = egui::Rect::from_two_pos(Self::POS01.0, Self::POS01.1 );
          }, 

        }
        self
    }

    const COLUMN_1: (f32, f32) = (0.0, 0.178);
    const COLUMN_2: (f32, f32) = (0.203, 0.382);
    const COLUMN_3: (f32, f32) = (0.406, 0.586);
    const COLUMN_4: (f32, f32) = (0.609, 0.791);
    const COLUMN_5: (f32, f32) = (0.813, 0.990);

    const ROW_1: (f32, f32) = (0.0, 0.182);
    const ROW_2: (f32, f32) = (0.199, 0.385);
    const ROW_3: (f32, f32) = (0.399, 0.584);
    const ROW_4: (f32, f32) = (0.599, 0.787);
    const ROW_5: (f32, f32) = (0.803, 0.988);

    const POS01: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_1.0, y: Self::ROW_1.0 }, Pos2 { x: Self::COLUMN_1.1, y: Self::ROW_1.1 });
    const POS02: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_2.0, y: Self::ROW_1.0 }, Pos2 { x: Self::COLUMN_2.1, y: Self::ROW_1.1 });
    const POS03: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_3.0, y: Self::ROW_1.0 }, Pos2 { x: Self::COLUMN_3.1, y: Self::ROW_1.1 });
    const POS04: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_4.0, y: Self::ROW_1.0 }, Pos2 { x: Self::COLUMN_4.1, y: Self::ROW_1.1 });
    const POS05: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_5.0, y: Self::ROW_1.0 }, Pos2 { x: Self::COLUMN_5.1, y: Self::ROW_1.1 });
    const POS06: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_1.0, y: Self::ROW_2.0 }, Pos2 { x: Self::COLUMN_1.1, y: Self::ROW_2.1 });
    const POS07: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_2.0, y: Self::ROW_2.0 }, Pos2 { x: Self::COLUMN_2.1, y: Self::ROW_2.1 });
    const POS08: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_3.0, y: Self::ROW_2.0 }, Pos2 { x: Self::COLUMN_3.1, y: Self::ROW_2.1 });
    const POS09: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_4.0, y: Self::ROW_2.0 }, Pos2 { x: Self::COLUMN_4.1, y: Self::ROW_2.1 });
    const POS10: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_5.0, y: Self::ROW_2.0 }, Pos2 { x: Self::COLUMN_5.1, y: Self::ROW_2.1 });
    const POS11: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_1.0, y: Self::ROW_3.0 }, Pos2 { x: Self::COLUMN_1.1, y: Self::ROW_3.1 });
    const POS12: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_2.0, y: Self::ROW_3.0 }, Pos2 { x: Self::COLUMN_2.1, y: Self::ROW_3.1 });
    const POS13: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_3.0, y: Self::ROW_3.0 }, Pos2 { x: Self::COLUMN_3.1, y: Self::ROW_3.1 });
    const POS14: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_4.0, y: Self::ROW_3.0 }, Pos2 { x: Self::COLUMN_4.1, y: Self::ROW_3.1 });
    const POS15: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_5.0, y: Self::ROW_3.0 }, Pos2 { x: Self::COLUMN_5.1, y: Self::ROW_3.1 });
    const POS16: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_1.0, y: Self::ROW_4.0 }, Pos2 { x: Self::COLUMN_1.1, y: Self::ROW_4.1 });
    const POS17: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_2.0, y: Self::ROW_4.0 }, Pos2 { x: Self::COLUMN_2.1, y: Self::ROW_4.1 });
    const POS18: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_3.0, y: Self::ROW_4.0 }, Pos2 { x: Self::COLUMN_3.1, y: Self::ROW_4.1 });
    const POS19: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_4.0, y: Self::ROW_4.0 }, Pos2 { x: Self::COLUMN_4.1, y: Self::ROW_4.1 });
    const POS20: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_5.0, y: Self::ROW_4.0 }, Pos2 { x: Self::COLUMN_5.1, y: Self::ROW_4.1 });
    
    const POS_SHUFFLE01: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_1.0, y: Self::ROW_5.0 }, Pos2 { x: Self::COLUMN_1.1, y: Self::ROW_5.1 });
    const POS_SHUFFLE02: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_2.0, y: Self::ROW_5.0 }, Pos2 { x: Self::COLUMN_2.1, y: Self::ROW_5.1 });
    const POS_SHUFFLE03: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_3.0, y: Self::ROW_5.0 }, Pos2 { x: Self::COLUMN_3.1, y: Self::ROW_5.1 });
    const POS_SHUFFLE04: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_4.0, y: Self::ROW_5.0 }, Pos2 { x: Self::COLUMN_4.1, y: Self::ROW_5.1 });
    const POS_SHUFFLE05: (Pos2, Pos2) = (Pos2 { x: Self::COLUMN_5.0, y: Self::ROW_5.0 }, Pos2 { x: Self::COLUMN_5.1, y: Self::ROW_5.1 });
}

