use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui::AlignSelf::Start;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: "贪吃蛇游戏".to_string(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}


fn setup(mut commands: Commands) {
    // 添加 2D 摄像机
    commands.spawn(Camera2dBundle::default());

    // 在控制台打印欢迎信息
    println!("贪吃蛇游戏窗口已启动！");
    println!("按 ESC 键退出游戏");
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    // 按 ESC 键退出游戏
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
