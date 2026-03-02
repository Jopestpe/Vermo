use bevy::prelude::*;
use crate::camera::CameraOrbitante;
use crate::grade::gerar_grade_de_referencia;
use crate::ui::EstadoLuz;

pub fn montar_cena_inicial(
    mut comandos: Commands,
    mut malhas: ResMut<Assets<Mesh>>,
    mut materiais: ResMut<Assets<StandardMaterial>>,
    estado_luz: Res<EstadoLuz>,
) {
    comandos.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraOrbitante,
    ));

    comandos.spawn((
        DirectionalLight {
            illuminance: estado_luz.intensidade,
            color: Color::srgb(estado_luz.vermelho, estado_luz.verde, estado_luz.azul),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            estado_luz.rotacao_x,
            estado_luz.rotacao_y,
            estado_luz.rotacao_z,
        )),
    ));

    gerar_grade_de_referencia(&mut comandos, &mut malhas, &mut materiais);
}
