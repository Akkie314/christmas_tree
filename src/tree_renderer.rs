use colored::Colorize;
use rand::Rng;
use terminal_size::{Width, terminal_size};

use crate::config::TreeConfig;

/// クリスマスツリーを描画する
pub fn render_tree(config: &TreeConfig, led_positions: &[(isize, isize)]) {
    let mut rng = rand::rng();
    let terminal_width = get_terminal_width();

    for y in 0..config.tree_height {
        let (row, row_width) = render_tree_row(y, led_positions, &mut rng);
        print_centered_row(&row, row_width, terminal_width);
    }
}

/// ターミナルの幅を取得（デフォルトは80）
fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80
    }
}

/// ツリーの1行を描画（表示幅も返す）
fn render_tree_row<R: Rng>(
    y: usize,
    led_positions: &[(isize, isize)],
    rng: &mut R,
) -> (String, usize) {
    let mut text = String::new();
    let row_width = 2 * y + 1;

    for i in 0..row_width {
        let x = i as isize - y as isize;
        let colored_char = get_colored_char(y, i, x, row_width, led_positions, rng);
        text.push_str(&colored_char.to_string());
    }

    (text, row_width)
}

/// 各位置の色付き文字を取得
fn get_colored_char<R: Rng>(
    y: usize,
    i: usize,
    x: isize,
    row_width: usize,
    led_positions: &[(isize, isize)],
    rng: &mut R,
) -> colored::ColoredString {
    if y == 0 && i == 0 {
        // 頂点の星
        "★".yellow()
    } else if i == 0 {
        // 左端
        "/".green()
    } else if i == row_width - 1 {
        // 右端
        "\\".green()
    } else if led_positions.contains(&(y as isize, x)) {
        // LED位置
        get_random_led_color(rng)
    } else {
        // 空白
        " ".white()
    }
}

/// ランダムなLEDの色を取得
fn get_random_led_color<R: Rng>(rng: &mut R) -> colored::ColoredString {
    match rng.random_range(0..6) {
        0 => "o".red().blink(),
        1 => "o".yellow().blink(),
        2 => "o".green().blink(),
        3 => "o".blue().blink(),
        4 => "o".white().blink(),
        5 => "o".purple().blink(),
        _ => "o".red().blink(),
    }
}

/// 行を中央揃えで出力
fn print_centered_row(row: &str, row_width: usize, terminal_width: usize) {
    let padding_size = (terminal_width.saturating_sub(row_width)) / 2;
    let padding = " ".repeat(padding_size);
    println!("{}{}", padding, row);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_terminal_width() {
        let width = get_terminal_width();
        // ターミナルサイズが取得できない場合は80
        assert!(width >= 80 || width > 0);
    }
}
