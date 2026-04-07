use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;

#[derive(Component)]
pub struct CameraMovimento {
    pub inclinacao: f32,
    pub rotacao: f32,
}

pub fn movimentar_camera(
    mut cameras: Query<(&mut Transform, &mut CameraMovimento)>,
    teclas: Res<ButtonInput<KeyCode>>,
    tempo: Res<Time>,
    mouse: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let (mut transformacao, mut fps) = match cameras.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        #[cfg(target_arch = "wasm32")]
        if let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector("canvas").ok().flatten()) 
        {
            let _ = canvas.request_pointer_lock();
        }
    }

    if teclas.just_pressed(KeyCode::Escape) {
        #[cfg(target_arch = "wasm32")]
        if let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector("canvas").ok().flatten()) 
        {
            let _ = canvas.request_pointer_lock();
        }
    }

    #[cfg(target_arch = "wasm32")]
    if web_sys::window()
        .and_then(|w| w.document())
        .map(|d| d.pointer_lock_element().is_some())
        .unwrap_or(false)
    {
        let mut direcao = Vec3::ZERO;
        let velocidade = 5.0;

        if teclas.pressed(KeyCode::KeyW) { direcao += *transformacao.forward(); }
        if teclas.pressed(KeyCode::KeyS) { direcao -= *transformacao.forward(); }
        if teclas.pressed(KeyCode::KeyA) { direcao -= *transformacao.right(); }
        if teclas.pressed(KeyCode::KeyD) { direcao += *transformacao.right(); }
        if teclas.pressed(KeyCode::Space) { direcao += Vec3::Y; }
        if teclas.pressed(KeyCode::ShiftLeft) { direcao -= Vec3::Y; }

        transformacao.translation += direcao.normalize_or_zero() * velocidade * tempo.delta_secs();

        let sensibilidade = 0.005;
        fps.rotacao -= mouse.delta.x * sensibilidade;
        fps.inclinacao -= mouse.delta.y * sensibilidade;
        fps.inclinacao = fps.inclinacao.clamp(-1.5, 1.5);

        transformacao.rotation = Quat::from_axis_angle(Vec3::Y, fps.rotacao)
            * Quat::from_axis_angle(Vec3::X, fps.inclinacao);
    }
}

