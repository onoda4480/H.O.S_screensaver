use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

fn main() {
    // ターミナルアニメーション版
    screensaver_mode();
}

fn screensaver_mode() {
    let mut stdout = stdout();

    // 代替スクリーンに入り、カーソルを隠す
    execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All)).unwrap();

    // Ctrl+Cでクリーンアップするためのハンドラ設定
    let result = std::panic::catch_unwind(|| {
        run_screensaver();
    });

    // 終了時のクリーンアップ
    execute!(stdout, Show, LeaveAlternateScreen).unwrap();

    if let Err(e) = result {
        eprintln!("エラーが発生しました: {:?}", e);
    }
}

fn run_screensaver() {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    // ターミナルサイズを取得
    let (width, height) = terminal::size().unwrap_or((80, 24));

    // 色のバリエーション
    let colors = vec![Color::Red];

    loop {
        // ランダムな位置を生成
        let x = rng.gen_range(0..width.saturating_sub(6)); // "BABEL " の長さ分引く
        let y = rng.gen_range(0..height);

        // ランダムな色を選択
        let color = colors[rng.gen_range(0..colors.len())];

        // カーソルを移動して、色付きで "BABEL" を表示
        execute!(
            stdout,
            MoveTo(x, y),
            SetForegroundColor(color),
            Print("BABEL "),
            ResetColor
        )
        .unwrap();

        stdout.flush().unwrap();

        // 少し待つ（スクリーンセーバーらしい速度）
        thread::sleep(Duration::from_millis(100));
    }
}
