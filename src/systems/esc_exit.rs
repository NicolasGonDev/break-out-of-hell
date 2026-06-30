use bevy::{
    app::AppExit,
    ecs::{message::MessageWriter, system::Res},
    input::{ButtonInput, keyboard::KeyCode},
};

pub fn esc_exit(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
