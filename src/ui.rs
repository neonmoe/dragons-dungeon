use crate::world::{Item, World};
use fae::{Font, GraphicsContext, Spritesheet};
use std::sync::Mutex;

pub const UI_AREA_WIDTH: f32 = 300.0;

#[derive(Debug, Default)]
pub struct DebugState {
    pub entity_count: usize,
}

lazy_static::lazy_static! {
    static ref DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState::default());
}

impl DebugState {
    pub fn modify<F: FnOnce(&mut DebugState)>(f: F) {
        let lock = DEBUG_STATE.lock();
        if let Ok(mut state) = lock {
            f(&mut state);
        }
    }
}

pub struct Ui {}

impl Ui {
    pub fn new() -> Ui {
        Ui {}
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext,
        font: &Font,
        spritesheet: &Spritesheet,
        world: &World,
        show_debug_info: bool,
    ) {
        let (width, height) = (ctx.width, ctx.height);

        let mut menu_cursor_y = 10.0;
        let mut menu_cursor_x = width - UI_AREA_WIDTH + menu_cursor_y;
        spritesheet
            .draw(ctx)
            .coordinates((
                menu_cursor_x,
                menu_cursor_y,
                width - menu_cursor_x - menu_cursor_y,
                height - menu_cursor_y * 2.0,
            ))
            .color((0.2, 0.2, 0.2, 1.0))
            .z(0.75)
            .finish();
        menu_cursor_x += 10.0;
        menu_cursor_y += 10.0;

        let mut ui_text = String::new();
        let player = world.player();
        let entities = world.entities();

        // Stats:
        if let (Some(damage), Some(health)) = (&player.damage, &player.health) {
            ui_text.push_str(&format!(
                "Stats:\n Damage: {}\n Health: {}/{}\n\n",
                damage.0, health.current, health.max
            ));
        }

        // Inventory
        if let Some(inventory) = &player.inventory {
            if !inventory.is_empty() {
                ui_text.push_str("Inventory:\n");
                let mut print_item = |item: &Item| ui_text.push_str(&format!(" {}\n", item.name()));
                if let Some(item) = &inventory.item_left {
                    print_item(item);
                }
                if let Some(item) = &inventory.item_right {
                    print_item(item);
                }
                ui_text.push('\n');
            }
        }

        // Pickups:
        for pickup in entities
            .iter()
            .filter(|e| e.position.x == player.position.x && e.position.y == player.position.y)
            .filter_map(|e| e.drop)
        {
            ui_text.push_str(&format!("Pickup [,]:\n {}\n", pickup.name()));
        }

        font.draw(ctx, &ui_text, menu_cursor_x, menu_cursor_y, 24.0)
            .z(1.0)
            .color((0.9, 0.9, 0.9, 1.0))
            .finish();

        if show_debug_info {
            if let Ok(debug_state) = DEBUG_STATE.lock() {
                let ui_text = format!("{:#?}", debug_state);
                let font_size = 12.0;
                if let Some(mut rect) = font
                    .draw(ctx, &ui_text, 0.0, 0.0, font_size)
                    .visibility(false)
                    .finish()
                {
                    let (x, y) = (20.0, ctx.height - rect.height - 20.0);
                    let padding = 10.0;
                    rect.x = x - padding;
                    rect.y = y - padding;
                    rect.width += padding * 2.0;
                    rect.height += padding * 2.0;
                    spritesheet
                        .draw(ctx)
                        .coordinates(rect)
                        .color((0.1, 0.1, 0.1, 0.6))
                        .z(0.99)
                        .finish();
                    font.draw(ctx, &ui_text, x, y, font_size)
                        .z(1.0)
                        .color((1.0, 1.0, 1.0, 1.0))
                        .finish();
                }
            }
        }
    }
}
