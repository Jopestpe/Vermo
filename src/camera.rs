use bevy::prelude::*;

#[derive(Component)]
pub struct CameraOrbitante;

pub fn movimentar_camera(
    mut consulta_camera: Query<&mut Transform, With<CameraOrbitante>>,
    teclado: Res<ButtonInput<KeyCode>>,
    tempo: Res<Time>,
) {
    let mut transformacao = match consulta_camera.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut direcao = Vec3::ZERO;
    let velocidade = 5.0;

    if teclado.pressed(KeyCode::KeyW) { direcao += *transformacao.forward(); }
    if teclado.pressed(KeyCode::KeyS) { direcao -= *transformacao.forward(); }
    if teclado.pressed(KeyCode::KeyA) { direcao -= *transformacao.right(); }
    if teclado.pressed(KeyCode::KeyD) { direcao += *transformacao.right(); }
    if teclado.pressed(KeyCode::Space) { direcao += Vec3::Y; }
    if teclado.pressed(KeyCode::ShiftLeft) || teclado.pressed(KeyCode::ShiftRight) {
        direcao -= Vec3::Y;
    }

    transformacao.translation += direcao.normalize_or_zero() * velocidade * tempo.delta_secs();
    transformacao.look_at(Vec3::ZERO, Vec3::Y);
}
