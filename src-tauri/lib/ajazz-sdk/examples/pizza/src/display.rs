use std::sync::Arc;
use image::DynamicImage;
use ajazz_sdk::AsyncAjazz;
use crate::game_state::{GameState, MOVE_LEFT};

pub struct GameAssets {
    pub pizza_open: Arc<DynamicImage>,
    pub pizza_closed: Arc<DynamicImage>,
    pub food: Arc<DynamicImage>,
    pub empty: Arc<DynamicImage>,
}

impl GameAssets {
    pub fn new(
        pizza_open: DynamicImage,
        pizza_closed: DynamicImage,
        food: DynamicImage,
        empty: DynamicImage,
    ) -> Self {
        Self {
            pizza_open: Arc::new(pizza_open),
            pizza_closed: Arc::new(pizza_closed),
            food: Arc::new(food),
            empty: Arc::new(empty),
        }
    }
}

pub struct DisplayManager {
    assets: GameAssets,
    display_key_count: u8,
}

impl DisplayManager {
    pub fn new(assets: GameAssets, display_key_count: u8) -> Self {
        Self {
            assets,
            display_key_count,
        }
    }

    pub async fn initialize_display(
        &self,
        device: &AsyncAjazz,
        game_state: &GameState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        device.clear_all_button_images().await?;

        // Set food images
        for i in 0..self.display_key_count {
            if game_state.has_food(i) && i != game_state.pizza_position {
                device
                    .set_button_image(i, (*self.assets.food).clone())
                    .await?;
            }
        }

        // Set pizza image
        let pizza_image = self.get_pizza_image(game_state);
        device
            .set_button_image(game_state.pizza_position, pizza_image)
            .await?;
        device.flush().await?;

        Ok(())
    }

    pub async fn update_display(
        &self,
        device: &AsyncAjazz,
        game_state: &GameState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Update previous pizza position if it changed
        if game_state.position_changed() {
            let previous_pos = game_state.get_previous_position();
            let image = if game_state.has_food(previous_pos) {
                (*self.assets.food).clone()
            } else {
                (*self.assets.empty).clone()
            };
            device.set_button_image(previous_pos, image).await?;
        }

        // Update current pizza position if state changed
        if game_state.has_state_changed() {
            let pizza_image = self.get_pizza_image(game_state);
            device
                .set_button_image(game_state.pizza_position, pizza_image)
                .await?;
        }

        device.flush().await?;
        Ok(())
    }

    pub async fn update_food_at_position(
        &self,
        device: &AsyncAjazz,
        position: u8,
        pizza_position: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if position != pizza_position {
            device
                .set_button_image(position, (*self.assets.food).clone())
                .await?;
            device.flush().await?;
        }
        Ok(())
    }

    fn get_pizza_image(&self, game_state: &GameState) -> DynamicImage {
        let base_image = if game_state.pizza_eating {
            &self.assets.pizza_closed
        } else {
            &self.assets.pizza_open
        };

        if game_state.direction == MOVE_LEFT {
            base_image.fliph()
        } else {
            (**base_image).clone()
        }
    }
}
