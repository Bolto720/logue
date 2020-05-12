use super::{Map, CombatStats, Player};
use rltk::{RGB, Rltk, Console};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let height = 20;
    let pos_y = map.height - 1 - height;
    ctx.draw_box(0, pos_y, map.width - 1, height, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP : {} / {} ", stats.hp, stats.max_hp);
        ctx.print_color(12, pos_y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &health);

        ctx.draw_bar_horizontal(28, pos_y, 51, stats.hp, stats.max_hp, RGB::named(rltk::RED), RGB::named(rltk::BLACK));
    }
}