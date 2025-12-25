mod config;
mod led_generator;
mod tree_renderer;

use config::TreeConfig;
use led_generator::generate_led_positions;
use tree_renderer::render_tree;

use std::{env, usize};

fn main() {
    let mut tree_height = 12;

    let args:Vec<String>= env::args().collect();

    if args.len() > 1 { tree_height = args[1].parse::<usize>().unwrap_or(12) }

    // デフォルト設定を使用
    let config = TreeConfig::new(tree_height, tree_height * 2, 2);

    // LED位置を生成
    let led_positions = generate_led_positions(&config);

    // ツリーを描画
    render_tree(&config, &led_positions);
}
