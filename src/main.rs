use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const BABEL_TEXT: &str = "BABEL ";
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 16;

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
    let event_loop = EventLoop::new().unwrap();

    let window = if fullscreen {
        WindowBuilder::new()
            .with_title("H.O.S BABEL Screensaver")
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
            .with_decorations(false)
            .build(&event_loop)
            .unwrap()
    } else {
        WindowBuilder::new()
            .with_title("H.O.S BABEL Screensaver")
            .with_inner_size(LogicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    let mut babel_texts: Vec<BabelText> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut last_add = Instant::now();

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::KeyboardInput {
                        event: KeyEvent { .. },
                        ..
                    } => {
                        // キー入力でスクリーンセーバーを終了
                        if fullscreen {
                            elwt.exit();
                        }
                    }
                    WindowEvent::MouseInput { .. } => {
                        // マウスクリックでスクリーンセーバーを終了
                        if fullscreen {
                            elwt.exit();
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        // マウス移動でスクリーンセーバーを終了
                        if fullscreen {
                            elwt.exit();
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();

                        // 100msごとに新しいBABELテキストを追加
                        if now.duration_since(last_add) > Duration::from_millis(100) {
                            let x = rng.gen_range(
                                0..window_size.width as i32
                                    - (BABEL_TEXT.len() * FONT_WIDTH) as i32,
                            );
                            let y =
                                rng.gen_range(0..window_size.height as i32 - FONT_HEIGHT as i32);

                            babel_texts.push(BabelText {
                                x,
                                y,
                                lifetime: now,
                            });

                            last_add = now;
                        }

                        // 古いテキストを削除（5秒経過したもの）
                        babel_texts.retain(|text| {
                            now.duration_since(text.lifetime) < Duration::from_secs(5)
                        });

                        // 画面を黒でクリア
                        let frame = pixels.frame_mut();
                        for pixel in frame.chunks_exact_mut(4) {
                            pixel[0] = 0; // R
                            pixel[1] = 0; // G
                            pixel[2] = 0; // B
                            pixel[3] = 255; // A
                        }

                        // すべてのBABELテキストを描画
                        for text in &babel_texts {
                            draw_text(
                                frame,
                                window_size.width,
                                window_size.height,
                                text.x,
                                text.y,
                                BABEL_TEXT,
                                [255, 0, 0, 255], // 赤色
                            );
                        }

                        if pixels.render().is_err() {
                            elwt.exit();
                            return;
                        }
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                    elwt.set_control_flow(ControlFlow::Poll);
                }
                _ => {}
            }
        })
        .unwrap();
}

// シンプルなビットマップフォントで文字列を描画
fn draw_text(
    frame: &mut [u8],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    text: &str,
    color: [u8; 4],
) {
    for (i, ch) in text.chars().enumerate() {
        let char_x = x + (i as i32 * FONT_WIDTH as i32);
        draw_char(frame, width, height, char_x, y, ch, color);
    }
}

// 1文字を描画（8x16のシンプルなビットマップフォント）
fn draw_char(frame: &mut [u8], width: u32, height: u32, x: i32, y: i32, ch: char, color: [u8; 4]) {
    let glyph = get_glyph(ch);

    for (row, &bits) in glyph.iter().enumerate() {
        for col in 0..8 {
            if (bits >> (7 - col)) & 1 == 1 {
                let px = x + col;
                let py = y + row as i32;

                if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                    let index = ((py as u32 * width + px as u32) * 4) as usize;
                    if index + 3 < frame.len() {
                        frame[index] = color[0];
                        frame[index + 1] = color[1];
                        frame[index + 2] = color[2];
                        frame[index + 3] = color[3];
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
