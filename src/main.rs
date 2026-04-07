use bevy::prelude::*;

mod camera;
mod cena;
mod grade;
mod modelo;
mod web;

fn main() {
    let mut app = App::new();
    web::registrar_fonte_de_assets(&mut app);

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .disable::<bevy::audio::AudioPlugin>(),
    )
    .init_resource::<modelo::EstadoModelo>()
    .add_systems(Startup, cena::montar_cena_inicial)
    .add_systems(
        Update,
        (
            camera::movimentar_camera,
            modelo::carregar_modelo_pendente,
            modelo::aplicar_comandos_luz,
        ),
    )
    .run();
}
