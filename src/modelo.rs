use bevy::prelude::*;
use crate::web::{LeitorModeloWeb, fila_arquivos_web, fila_comandos_web};
use crate::web::fila_comandos_web::ComandoWeb;

#[derive(Resource, Default)]
pub struct EstadoModelo {
    pub precisa_carregar: bool,
    pub entidade_atual: Option<Entity>,
    pub i: u32,
}

pub fn carregar_modelo_pendente(
    mut comandos: Commands,
    mut estado: ResMut<EstadoModelo>,
    servidor_assets: Res<AssetServer>,
    leitor: Res<LeitorModeloWeb>,
) {
    for arquivo in fila_arquivos_web::esvaziar() {
        leitor.0.substituir_modelo(arquivo.bytes);
        estado.precisa_carregar = true;
    }

    if !estado.precisa_carregar {
        return;
    }
    estado.precisa_carregar = false;

    if let Some(entidade) = estado.entidade_atual {
        comandos.entity(entidade).despawn();
    }

    estado.i += 1;
    let path = format!("mem://{}.glb#Scene0", estado.i);
    let handle_modelo = servidor_assets.load::<Scene>(path);

    let nova_entidade = comandos
        .spawn((SceneRoot(handle_modelo), Transform::default()))
        .id();

    estado.entidade_atual = Some(nova_entidade);
}

pub fn aplicar_comandos_luz(
    mut params: ParamSet<(
        Query<(&mut DirectionalLight, &mut Transform)>,
        Query<&mut Transform, With<SceneRoot>>,
    )>,
) {
    for cmd in fila_comandos_web::esvaziar() {
        let mut luz = params.p0();
        if let Ok((mut luz_dir, mut transform)) = luz.single_mut() {
            match cmd {
                ComandoWeb::Rotacao(x, y, z) => transform.rotation = Quat::from_euler(EulerRot::XYZ, x, y, z),
                ComandoWeb::Intensidade(v) => luz_dir.illuminance = v,
                ComandoWeb::Cor(r, g, b) => luz_dir.color = Color::srgb(r, g, b),
                ComandoWeb::Escala(v) => {
                    let mut modelos = params.p1();
                    if let Ok(mut tf) = modelos.single_mut() {
                        tf.scale = Vec3::splat(v);
                    }
                }
            }
        }
    }
}
