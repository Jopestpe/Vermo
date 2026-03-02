use bevy::prelude::*;

pub fn gerar_grade_de_referencia(
    comandos: &mut Commands,
    malhas: &mut ResMut<Assets<Mesh>>,
    materiais: &mut ResMut<Assets<StandardMaterial>>,
) {
    let tamanho_total = 20.0;
    let tamanho_celula = 1.0;
    let cor_grade = Color::srgba(0.5, 0.5, 0.5, 0.3);
    let numero_linhas = (tamanho_total / tamanho_celula) as i32;

    for i in 0..=numero_linhas {
        let posicao = (i as f32 - tamanho_total / 2.0) * tamanho_celula;

        let linha_z = criar_segmento_de_linha(
            Vec3::new(posicao, 0.0, -tamanho_total / 2.0),
            Vec3::new(posicao, 0.0, tamanho_total / 2.0),
        );
        comandos.spawn((
            Mesh3d(malhas.add(linha_z)),
            MeshMaterial3d(materiais.add(StandardMaterial { base_color: cor_grade, unlit: true, ..default() })),
        ));

        let linha_x = criar_segmento_de_linha(
            Vec3::new(-tamanho_total / 2.0, 0.0, posicao),
            Vec3::new(tamanho_total / 2.0, 0.0, posicao),
        );
        comandos.spawn((
            Mesh3d(malhas.add(linha_x)),
            MeshMaterial3d(materiais.add(StandardMaterial { base_color: cor_grade, unlit: true, ..default() })),
        ));
    }

    let comprimento_eixo = tamanho_total / 2.0;
    for (cor_eixo, extremidade_eixo) in [
        (Color::srgb(1.0, 0.0, 0.0), Vec3::new(comprimento_eixo, 0.0, 0.0)),
        (Color::srgb(0.0, 1.0, 0.0), Vec3::new(0.0, comprimento_eixo, 0.0)),
        (Color::srgb(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, comprimento_eixo)),
    ] {
        comandos.spawn((
            Mesh3d(malhas.add(criar_segmento_de_linha(Vec3::ZERO, extremidade_eixo))),
            MeshMaterial3d(materiais.add(StandardMaterial { base_color: cor_eixo, unlit: true, ..default() })),
        ));
    }
}

fn criar_segmento_de_linha(origem: Vec3, destino: Vec3) -> Mesh {
    let mut malha = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        Default::default(),
    );
    malha.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![origem, destino]);
    malha
}
