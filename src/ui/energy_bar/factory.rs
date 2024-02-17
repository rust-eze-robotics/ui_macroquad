use std::{collections::HashMap, rc::Rc};

use macroquad::prelude::*;

const BAR_ARCHER_1_ID: u8 = 0;
const BAR_ARCHER_2_ID: u8 = 1;
const BAR_ARCHER_3_ID: u8 = 2;
const BAR_ARCHER_4_ID: u8 = 3;
const BAR_ARCHER_5_ID: u8 = 4;
const BAR_ARCHER_6_ID: u8 = 5;
const BAR_ARCHER_7_ID: u8 = 6;
const BAR_ARCHER_8_ID: u8 = 7;
const BAR_PAWN_1_ID: u8 = 8;
const BAR_PAWN_2_ID: u8 = 9;
const BAR_PAWN_3_ID: u8 = 10;
const BAR_PAWN_4_ID: u8 = 11;
const BAR_PAWN_5_ID: u8 = 12;
const BAR_PAWN_6_ID: u8 = 13;
const BAR_PAWN_7_ID: u8 = 14;
const BAR_PAWN_8_ID: u8 = 15;
const BAR_TORCH_1_ID: u8 = 16;
const BAR_TORCH_2_ID: u8 = 17;
const BAR_TORCH_3_ID: u8 = 18;
const BAR_TORCH_4_ID: u8 = 19;
const BAR_TORCH_5_ID: u8 = 20;
const BAR_TORCH_6_ID: u8 = 21;
const BAR_TORCH_7_ID: u8 = 22;
const BAR_TORCH_8_ID: u8 = 23;
const BAR_WARRIOR_1_ID: u8 = 24;
const BAR_WARRIOR_2_ID: u8 = 25;
const BAR_WARRIOR_3_ID: u8 = 26;
const BAR_WARRIOR_4_ID: u8 = 27;
const BAR_WARRIOR_5_ID: u8 = 28;
const BAR_WARRIOR_6_ID: u8 = 29;
const BAR_WARRIOR_7_ID: u8 = 30;
const BAR_WARRIOR_8_ID: u8 = 31;

pub struct BarFactory {
    textures: HashMap<u8, Rc<Texture2D>>,
}

impl BarFactory {
    pub async fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            BAR_ARCHER_1_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_1.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_2_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_2.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_3_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_3.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_4_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_4.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_5_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_5.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_6_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_6.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_7_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_7.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_ARCHER_8_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/archer/archer_8.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_1_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_1.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_2_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_2.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_3_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_3.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_4_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_4.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_5_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_5.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_6_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_6.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_7_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_7.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_PAWN_8_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/pawn/pawn_8.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_1_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_1.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_2_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_2.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_3_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_3.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_4_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_4.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_5_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_5.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_6_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_6.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_7_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_7.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_TORCH_8_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/torch/torch_8.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_1_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_1.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_2_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_2.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_3_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_3.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_4_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_4.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_5_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_5.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_6_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_6.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_7_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_7.png")
                    .await
                    .unwrap(),
            ),
        );

        textures.insert(
            BAR_WARRIOR_8_ID,
            Rc::new(
                load_texture("assets/textures/ui/bars/warrior/warrior_8.png")
                    .await
                    .unwrap(),
            ),
        );

        Self { textures }
    }

    pub fn get_archer_texture(&self, energy: usize) -> Rc<Texture2D> {
        if energy >= 875 {
            self.textures[&BAR_ARCHER_8_ID].clone()
        } else if energy >= 750 {
            self.textures[&BAR_ARCHER_7_ID].clone()
        } else if energy >= 625 {
            self.textures[&BAR_ARCHER_6_ID].clone()
        } else if energy >= 500 {
            self.textures[&BAR_ARCHER_5_ID].clone()
        } else if energy >= 375 {
            self.textures[&BAR_ARCHER_4_ID].clone()
        } else if energy >= 250 {
            self.textures[&BAR_ARCHER_3_ID].clone()
        } else if energy >= 125 {
            self.textures[&BAR_ARCHER_2_ID].clone()
        } else {
            self.textures[&BAR_ARCHER_1_ID].clone()
        }
    }

    pub fn get_pawn_texture(&self, energy: usize) -> Rc<Texture2D> {
        if energy >= 875 {
            self.textures[&BAR_PAWN_8_ID].clone()
        } else if energy >= 750 {
            self.textures[&BAR_PAWN_7_ID].clone()
        } else if energy >= 625 {
            self.textures[&BAR_PAWN_6_ID].clone()
        } else if energy >= 500 {
            self.textures[&BAR_PAWN_5_ID].clone()
        } else if energy >= 375 {
            self.textures[&BAR_PAWN_4_ID].clone()
        } else if energy >= 250 {
            self.textures[&BAR_PAWN_3_ID].clone()
        } else if energy >= 125 {
            self.textures[&BAR_PAWN_2_ID].clone()
        } else {
            self.textures[&BAR_PAWN_1_ID].clone()
        }
    }

    pub fn get_torch_texture(&self, energy: usize) -> Rc<Texture2D> {
        if energy >= 875 {
            self.textures[&BAR_TORCH_8_ID].clone()
        } else if energy >= 750 {
            self.textures[&BAR_TORCH_7_ID].clone()
        } else if energy >= 625 {
            self.textures[&BAR_TORCH_6_ID].clone()
        } else if energy >= 500 {
            self.textures[&BAR_TORCH_5_ID].clone()
        } else if energy >= 375 {
            self.textures[&BAR_TORCH_4_ID].clone()
        } else if energy >= 250 {
            self.textures[&BAR_TORCH_3_ID].clone()
        } else if energy >= 125 {
            self.textures[&BAR_TORCH_2_ID].clone()
        } else {
            self.textures[&BAR_TORCH_1_ID].clone()
        }
    }

    pub fn get_warrior_texture(&self, energy: usize) -> Rc<Texture2D> {
        if energy >= 875 {
            self.textures[&BAR_WARRIOR_8_ID].clone()
        } else if energy >= 750 {
            self.textures[&BAR_WARRIOR_7_ID].clone()
        } else if energy >= 625 {
            self.textures[&BAR_WARRIOR_6_ID].clone()
        } else if energy >= 500 {
            self.textures[&BAR_WARRIOR_5_ID].clone()
        } else if energy >= 375 {
            self.textures[&BAR_WARRIOR_4_ID].clone()
        } else if energy >= 250 {
            self.textures[&BAR_WARRIOR_3_ID].clone()
        } else if energy >= 125 {
            self.textures[&BAR_WARRIOR_2_ID].clone()
        } else {
            self.textures[&BAR_WARRIOR_1_ID].clone()
        }
    }
}
