use bevy::prelude::*;
use crate::camera::CameraMovimento;
use crate::grade::gerar_grade_de_referencia;

pub fn montar_cena_inicial(
    mut comandos: Commands,
    mut malhas: ResMut<Assets<Mesh>>,
    mut materiais: ResMut<Assets<StandardMaterial>>,
) {
    comandos.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraMovimento {
            inclinacao: 0.0,
            rotacao: 0.0,
        },
    ));

    comandos.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            color: Color::srgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            (-135_f32).to_radians(),
            0.0,
            0.0,
        )),
    ));

    gerar_grade_de_referencia(&mut comandos, &mut malhas, &mut materiais);
}

