use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SetWindowLongPtrW, SetWindowPos, GWL_STYLE, GWL_EXSTYLE,
    SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
    WS_POPUP, WS_VISIBLE, WS_EX_TOPMOST, SM_CXSCREEN, SM_CYSCREEN,
};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;

const BABEL_TEXT: &str = "BABEL ";
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 16;
const FONT_THICKNESS: usize = 2; // 文字の太さ (1=通常, 2=太字, 3=極太)

#[derive(Debug)]
enum ScreensaverMode {
    Normal,    // /s または引数なし - フルスクリーンモード
    Preview,   // /p - プレビューモード
    Configure, // /c - 設定ダイアログ
}

struct BabelText {
    x: i32,
    y: i32,
    lifetime: Instant,
}

fn main() {
    // コマンドライン引数を解析
    let args: Vec<String> = std::env::args().collect();
    let mode = parse_arguments(&args);

    match mode {
        ScreensaverMode::Configure => {
            // 設定ダイアログは省略（シンプルなスクリーンセーバーなので設定不要）
            println!("このスクリーンセーバーには設定項目がありません。");
            return;
        }
        ScreensaverMode::Preview => {
            // プレビューモードは小さいウィンドウで実行
            run_screensaver(false);
        }
        ScreensaverMode::Normal => {
            // 通常モード（フルスクリーン）
            run_screensaver(true);
        }
    }
}

fn parse_arguments(args: &[String]) -> ScreensaverMode {
    if args.len() > 1 {
        let arg = args[1].to_lowercase();
        if arg.starts_with("/s") || arg.starts_with("-s") {
            return ScreensaverMode::Normal;
        } else if arg.starts_with("/p") || arg.starts_with("-p") {
            return ScreensaverMode::Preview;
        } else if arg.starts_with("/c") || arg.starts_with("-c") {
            return ScreensaverMode::Configure;
        }
    }
    // デフォルトは通常モード
    ScreensaverMode::Normal
}

fn run_screensaver(fullscreen: bool) {
    // 画面サイズを取得
    let (width, height) = if fullscreen {
        // 実際のプライマリディスプレイサイズを取得
        get_screen_size()
    } else {
        (800, 600)
    };

    let mut window_options = WindowOptions::default();
    window_options.borderless = true; // 常にボーダーレス（枠なし）
    window_options.resize = false;    // リサイズ不可
    window_options.title = false;     // タイトルバーを非表示
    window_options.topmost = true;    // 最前面に配置

    let mut window = Window::new(
        "", // 空文字列でタイトルバーを非表示にする
        width,
        height,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window {}", e);
    });

    // Windows でタイトルバーを完全に削除
    #[cfg(target_os = "windows")]
    remove_window_titlebar(&window);

    // フレームレートを設定
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut babel_texts: Vec<BabelText> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut last_add = Instant::now();

    // マウス位置の初期化（終了判定用）
    let initial_mouse_pos = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap_or((0.0, 0.0));
    let mut last_mouse_pos = initial_mouse_pos;

    // 起動直後のキーイベントを無視するための待機時間
    let start_time = Instant::now();
    let grace_period = Duration::from_millis(500); // 500ms の猶予期間

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        // フルスクリーンモードでは、マウス移動やキー入力で終了（起動後の猶予期間を過ぎてから）
        if fullscreen && now.duration_since(start_time) > grace_period {
            // マウスが動いたら終了
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                let dx = (x - last_mouse_pos.0).abs();
                let dy = (y - last_mouse_pos.1).abs();
                if dx > 5.0 || dy > 5.0 {
                    break;
                }
            }

            // 何かキーが押されたら終了
            if !window.get_keys().is_empty() {
                break;
            }
        }

        // 100msごとに新しいBABELテキストを追加
        if now.duration_since(last_add) > Duration::from_millis(10) {
            let x = rng.gen_range(0..(width as i32 - (BABEL_TEXT.len() * FONT_WIDTH) as i32));
            let y = rng.gen_range(0..(height as i32 - FONT_HEIGHT as i32));

            babel_texts.push(BabelText {
                x,
                y,
                lifetime: now,
            });

            last_add = now;
        }

        // 古いテキストを削除（5秒経過したもの）
        babel_texts.retain(|text| now.duration_since(text.lifetime) < Duration::from_secs(100));

        // 画面を黒でクリア
        buffer.fill(0);

        // すべてのBABELテキストを描画
        for text in &babel_texts {
            draw_text(
                &mut buffer,
                width,
                height,
                text.x,
                text.y,
                BABEL_TEXT,
                0x00FF0000, // 赤色 (ARGB: Alpha=0, Red=255, Green=0, Blue=0)
            );
        }

        // バッファを更新
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

// シンプルなビットマップフォントで文字列を描画
fn draw_text(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    text: &str,
    color: u32,
) {
    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * FONT_WIDTH as i32);
        draw_char(buffer, width, height, char_x, y, ch, color);
    }
}

// 1文字を描画（8x16のシンプルなビットマップフォント）
fn draw_char(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    ch: char,
    color: u32,
) {
    let glyph = get_glyph(ch);

    for (row, &bits) in glyph.iter().enumerate() {
        for col in 0..8 {
            if (bits >> (7 - col)) & 1 == 1 {
                // FONT_THICKNESS に応じて複数のピクセルを描画
                for dy in 0..FONT_THICKNESS {
                    for dx in 0..FONT_THICKNESS {
                        let px = x + col + dx as i32;
                        let py = y + row as i32 + dy as i32;

                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let index = (py as usize * width + px as usize) as usize;
                            if index < buffer.len() {
                                buffer[index] = color;
                            }
                        }
                    }
                }
            }
        }
    }
}

// シンプルな8x16ビットマップフォント定義
fn get_glyph(ch: char) -> [u8; 16] {
    match ch {
        'B' => [
            0b01111100, 0b01000010, 0b01000010, 0b01000010, 0b01111100, 0b01000010, 0b01000010,
            0b01000010, 0b01000010, 0b01000010, 0b01111100, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ],
        'A' => [
            0b00111100, 0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b01111110, 0b01000010,
            0b01000010, 0b01000010, 0b01000010, 0b01000010, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ],
        'E' => [
            0b01111110, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01111100, 0b01000000,
            0b01000000, 0b01000000, 0b01000000, 0b01111110, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ],
        'L' => [
            0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000, 0b01000000,
            0b01000000, 0b01000000, 0b01000000, 0b01111110, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ],
        ' ' => [0; 16],
        _ => [
            0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110,
            0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ],
    }
}

// プライマリディスプレイのサイズを取得
#[cfg(target_os = "windows")]
fn get_screen_size() -> (usize, usize) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (width as usize, height as usize)
    }
}

// Windows以外の環境ではデフォルト値を返す
#[cfg(not(target_os = "windows"))]
fn get_screen_size() -> (usize, usize) {
    // macOS や Linux での開発時のデフォルト値
    (1920, 1080)
}

// Windows でタイトルバーを完全に削除する
#[cfg(target_os = "windows")]
fn remove_window_titlebar(window: &Window) {
    unsafe {
        // ウィンドウハンドルを取得
        if let Ok(handle) = window.window_handle() {
            if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                let hwnd = HWND(win32_handle.hwnd.get() as *mut core::ffi::c_void);

                // WS_POPUP スタイルに変更（タイトルバーなし）
                SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE).0 as isize);

                // WS_EX_TOPMOST スタイルを設定（最前面）
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_TOPMOST.0 as isize);

                // 変更を適用
                let _ = SetWindowPos(
                    hwnd,
                    HWND(std::ptr::null_mut()),
                    0,
                    0,
                    0,
                    0,
                    SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER,
                );
            }
        }
    }
}
