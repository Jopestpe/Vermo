use bevy::prelude::*;

mod camera;
mod cena;
mod grade;
mod modelo;
mod ui;

#[cfg(target_arch = "wasm32")]
mod web;

fn main() {
    let mut aplicacao = App::new();

    #[cfg(target_arch = "wasm32")]
    web::registrar_fonte_de_assets(&mut aplicacao);

    aplicacao
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .disable::<bevy::audio::AudioPlugin>()
        )
        .insert_resource(ui::EstadoLuz::inicial())
        .init_resource::<modelo::EstadoModelo>()
        .add_systems(Startup, (cena::montar_cena_inicial, ui::montar_ui))
        .add_systems(Update, (
            modelo::ao_clicar_botao_abrir,
            camera::movimentar_camera,
            modelo::carregar_modelo_pendente,
            ui::atualizar_controles_luz,
        ))
        .run();
}
