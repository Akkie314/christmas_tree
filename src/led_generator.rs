use rand::Rng;
use rand::seq::SliceRandom;

use crate::config::TreeConfig;

/// LED位置を生成する（y, x座標のタプル）
/// xは中央を0とした相対座標
pub fn generate_led_positions(config: &TreeConfig) -> Vec<(isize, isize)> {
    let mut rng = rand::rng();
    
    // 候補となる座標をすべてリストアップ (y, x)
    // x は中央を 0 とした相対座標
    let candidates = generate_candidates(config.tree_height);
    
    // LED位置を選択
    select_led_positions(candidates, config.led_count, config.min_distance, &mut rng)
}

/// ツリー内の全ての候補座標を生成
fn generate_candidates(tree_height: usize) -> Vec<(isize, isize)> {
    let mut candidates = Vec::new();
    for y in 1..tree_height {
        for x in -(y as isize - 1)..=(y as isize - 1) {
            candidates.push((y as isize, x));
        }
    }
    candidates
}

/// チェビシェフ距離を考慮してLED位置を選択
fn select_led_positions<R: Rng>(
    mut candidates: Vec<(isize, isize)>,
    led_count: usize,
    min_distance: usize,
    rng: &mut R,
) -> Vec<(isize, isize)> {
    // 候補をシャッフル
    candidates.shuffle(rng);

    let mut led_positions = Vec::new();
    for (y, x) in candidates {
        if led_positions.len() >= led_count {
            break;
        }

        // すでに配置されたLEDとの距離をチェック
        if is_position_valid(&led_positions, y, x, min_distance) {
            led_positions.push((y, x));
        }
    }

    led_positions
}

/// 指定された位置が既存のLED位置と最小距離を満たすかチェック
fn is_position_valid(
    led_positions: &[(isize, isize)],
    y: isize,
    x: isize,
    min_distance: usize,
) -> bool {
    led_positions.iter().all(|&(ly, lx)| {
        let dy = (y - ly).abs();
        let dx = (x - lx).abs();
        dy.max(dx) >= min_distance as isize
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_candidates() {
        let candidates = generate_candidates(3);
        // y=1: x=0 の1つ
        // y=2: x=-1,0,1 の3つ
        // 合計4つ
        assert_eq!(candidates.len(), 4);
    }

    #[test]
    fn test_is_position_valid() {
        let led_positions = vec![(1, 0), (3, 0)];
        // (2, 0)は(1, 0)から距離1なのでmin_distance=2では無効
        assert!(!is_position_valid(&led_positions, 2, 0, 2));
        // (5, 0)は両方から十分離れている
        assert!(is_position_valid(&led_positions, 5, 0, 2));
    }
}
