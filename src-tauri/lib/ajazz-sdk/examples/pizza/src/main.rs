mod game_state;
mod display;
mod config;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;
use image::open;

use ajazz_sdk::{Event, list_devices, new_hidapi, AsyncAjazz};
use game_state::{GameState, MOVE_LEFT, MOVE_RIGHT};
use display::{DisplayManager, GameAssets};
use config::*;

async fn load_game_assets() -> Result<GameAssets, Box<dyn std::error::Error>> {
    let pizza_open = open("resources/open.jpg")?;
    let pizza_closed = open("resources/closed.jpg")?;
    let food = open("resources/food.jpg")?;
    let empty = open("resources/empty.jpg")?;

    println!("Game assets loaded successfully");
    Ok(GameAssets::new(pizza_open, pizza_closed, food, empty))
}

async fn setup_device(device: &AsyncAjazz) -> Result<(), Box<dyn std::error::Error>> {
    device.set_brightness(DEVICE_BRIGHTNESS).await?;
    device.clear_all_button_images().await?;
    device.flush().await?;
    Ok(())
}

async fn handle_input_events(
    device: AsyncAjazz,
    game_state: Arc<Mutex<GameState>>,
    display_manager: Arc<DisplayManager>,
    display_key_count: u8,
) {
    let reader = device.get_reader();

    loop {
        match reader.read(INPUT_TIMEOUT_MS).await {
            Ok(events) => {
                for event in events {
                    match event {
                        Event::ButtonDown(key) => {
                            if key < display_key_count {
                                println!("Button {} pressed", key);

                                let should_update = {
                                    let mut state = game_state.lock().await;
                                    if !state.has_food(key) {
                                        state.add_food(key);
                                        println!("Food added at position {}", key);
                                        true
                                    } else {
                                        false
                                    }
                                };

                                if should_update {
                                    let pizza_pos = {
                                        let state = game_state.lock().await;
                                        state.pizza_position
                                    };

                                    if let Err(e) = display_manager
                                        .update_food_at_position(&device, key, pizza_pos)
                                        .await
                                    {
                                        println!("Failed to update display: {:?}", e);
                                    }
                                }
                            }
                        }
                        Event::EncoderTwist(_dial, ticks) => {
                            let mut state = game_state.lock().await;
                            let direction = if ticks > 0 { MOVE_RIGHT } else { MOVE_LEFT };
                            state.set_direction(direction);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                println!("Input reading error: {:?}", e);
                break;
            }
        }
    }
}

async fn run_game_loop(
    device: AsyncAjazz,
    game_state: Arc<Mutex<GameState>>,
    display_manager: Arc<DisplayManager>,
    display_key_count: u8,
) {
    loop {
        sleep(ANIMATION_INTERVAL).await;

        let (ate_food, current_position) = {
            let mut state = game_state.lock().await;
            let current_pos = state.pizza_position;
            let ate = state.eat_food(current_pos);
            (ate, current_pos)
        };

        if ate_food {
            println!("pizza ate food at position {}", current_position);

            // Set eating state and update display
            {
                let mut state = game_state.lock().await;
                state.set_eating_state(true);

                if let Err(e) = display_manager.update_display(&device, &state).await {
                    println!("Display update error: {:?}", e);
                }
            }

            sleep(EATING_DURATION).await;

            // Reset eating state
            {
                let mut state = game_state.lock().await;
                state.set_eating_state(false);
            }
        }

        // Move pizza and update display
        {
            let mut state = game_state.lock().await;
            state.move_pizza(display_key_count);
            println!("pizza moved to position {}", state.pizza_position);

            if let Err(e) = display_manager.update_display(&device, &state).await {
                println!("Display update error: {:?}", e);
                break;
            }
        }
    }
}

async fn run_game_for_device(
    device: AsyncAjazz,
    display_key_count: u8,
    assets: GameAssets,
) -> Result<(), Box<dyn std::error::Error>> {
    setup_device(&device).await?;

    let game_state = Arc::new(Mutex::new(GameState::new(display_key_count)));
    let display_manager = Arc::new(DisplayManager::new(assets, display_key_count));

    // Initialize display
    {
        let state = game_state.lock().await;
        display_manager.initialize_display(&device, &state).await?;
    }

    // Spawn input and game loop tasks
    let input_task = {
        let device = device.clone();
        let game_state = game_state.clone();
        let display_manager = display_manager.clone();
        tokio::spawn(handle_input_events(
            device,
            game_state,
            display_manager,
            display_key_count,
        ))
    };

    let game_task = {
        let device = device.clone();
        let game_state = game_state.clone();
        let display_manager = display_manager.clone();
        tokio::spawn(run_game_loop(
            device,
            game_state,
            display_manager,
            display_key_count,
        ))
    };

    println!("pizza game started!");
    println!("- Press display buttons to add food");
    println!("- Turn encoders to change direction");
    println!("- Press Ctrl+C to exit");

    // Wait for tasks to complete
    tokio::select! {
        _ = input_task => println!("Input task completed"),
        _ = game_task => println!("Game loop completed"),
    }

    // Cleanup
    device.clear_all_button_images().await.ok();
    device.flush().await.ok();
    device.shutdown().await.ok();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assets = load_game_assets().await?;
    let hid = new_hidapi()?;
    let devices = list_devices(&hid);

    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }

    for (kind, serial) in devices {
        println!("Found device: {:?} {} {}", kind, serial, kind.product_id());

        let device =
            AsyncAjazz::connect_with_retries(&hid, kind, &serial, MAX_CONNECTION_RETRIES)?;

        println!(
            "Connected to '{}' with firmware version '{}'",
            device.serial_number().await?,
            device.firmware_version().await?
        );

        let display_key_count = kind.display_key_count();
        println!("Display keys available: {}", display_key_count);

        if display_key_count == 0 {
            println!("No display keys available, skipping device");
            continue;
        }

        run_game_for_device(device, display_key_count, assets).await?;
        break; // Only use first found device
    }

    Ok(())
}
