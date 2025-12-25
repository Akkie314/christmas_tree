/// クリスマスツリーの設定値を管理する構造体
#[derive(Debug, Clone, Copy)]
pub struct TreeConfig {
    /// ツリーの高さ（行数）
    pub tree_height: usize,
    /// LEDの数
    pub led_count: usize,
    /// LED間の最小チェビシェフ距離
    pub min_distance: usize,
}


impl TreeConfig {
    /// 新しい設定を作成
    pub fn new(tree_height: usize, led_count: usize, min_distance: usize) -> Self {
        Self {
            tree_height,
            led_count,
            min_distance,
        }
    }
}
