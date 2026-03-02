use bevy::prelude::*;
use crate::ui::BotaoAbrirArquivo;

#[cfg(target_arch = "wasm32")]
use crate::web::LeitorModeloWeb;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Resource, Default)]
pub struct EstadoModelo {
    pub precisa_carregar: bool,
    pub entidade_atual: Option<Entity>,
}

pub fn carregar_modelo_pendente(
    mut comandos: Commands,
    mut estado: ResMut<EstadoModelo>,
    servidor_assets: Res<AssetServer>,
) {
    if !estado.precisa_carregar {
        return;
    }
    estado.precisa_carregar = false;

    if let Some(entidade) = estado.entidade_atual {
        comandos.entity(entidade).despawn();
    }

    #[cfg(not(target_arch = "wasm32"))]
    let handle_modelo = servidor_assets.load::<Scene>("modelo_temp.glb#Scene0");

    #[cfg(target_arch = "wasm32")]
    let handle_modelo = servidor_assets.load::<Scene>("mem://modelo.glb#Scene0");

    let nova_entidade = comandos
        .spawn((SceneRoot(handle_modelo), Transform::default()))
        .id();

    estado.entidade_atual = Some(nova_entidade);
}

pub fn ao_clicar_botao_abrir(
    #[cfg(not(target_arch = "wasm32"))]
    interacoes: Query<&Interaction, (Changed<Interaction>, With<BotaoAbrirArquivo>)>,
    #[cfg(target_arch = "wasm32")]
    interacoes: Query<&Interaction, (Changed<Interaction>, With<BotaoAbrirArquivo>)>,
    mut estado: ResMut<EstadoModelo>,
    #[cfg(target_arch = "wasm32")]
    leitor: Res<LeitorModeloWeb>,
) {
    #[cfg(target_arch = "wasm32")]
    for arquivo in crate::web::fila_arquivos_web::esvaziar() {
        leitor.0.substituir_modelo(arquivo.bytes);
        estado.precisa_carregar = true;
    }

    for interacao in &interacoes {
        if *interacao == Interaction::Pressed {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(caminho_arquivo) = rfd::FileDialog::new()
                .add_filter("Modelos 3D", &["gltf", "glb"])
                .pick_file()
            {
                let destino = std::path::PathBuf::from("assets/modelo_temp.glb");
                if let Err(erro) = std::fs::copy(&caminho_arquivo, &destino) {
                    eprintln!("Erro ao copiar arquivo: {erro}");
                    return;
                }
                estado.precisa_carregar = true;
            }

            #[cfg(target_arch = "wasm32")]
            if let Some(win) = web_sys::window() {
                if let Ok(func) = js_sys::Reflect::get(&win, &"abrir_seletor_arquivo".into()) {
                    if let Some(f) = func.dyn_ref::<js_sys::Function>() {
                        let _ = f.call0(&win).ok();
                    }
                }
            }
        }
    }
}
