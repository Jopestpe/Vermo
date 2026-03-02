use bevy::prelude::*;

#[derive(Component)]
pub struct BotaoAbrirArquivo;

#[derive(Component)]
pub struct BotaoAumentarRotacaoX;
#[derive(Component)]
pub struct BotaoDiminuirRotacaoX;

#[derive(Component)]
pub struct BotaoAumentarRotacaoY;
#[derive(Component)]
pub struct BotaoDiminuirRotacaoY;

#[derive(Component)]
pub struct BotaoAumentarRotacaoZ;
#[derive(Component)]
pub struct BotaoDiminuirRotacaoZ;

#[derive(Component)]
pub struct BotaoAumentarIntensidade;
#[derive(Component)]
pub struct BotaoDiminuirIntensidade;

#[derive(Component)]
pub struct BotaoAumentarVermelho;
#[derive(Component)]
pub struct BotaoDiminuirVermelho;

#[derive(Component)]
pub struct BotaoAumentarVerde;
#[derive(Component)]
pub struct BotaoDiminuirVerde;

#[derive(Component)]
pub struct BotaoAumentarAzul;
#[derive(Component)]
pub struct BotaoDiminuirAzul;

#[derive(Resource, Default)]
pub struct EstadoLuz {
    pub rotacao_x: f32,
    pub rotacao_y: f32,
    pub rotacao_z: f32,
    pub intensidade: f32,
    pub vermelho: f32,
    pub verde: f32,
    pub azul: f32,
}

impl EstadoLuz {
    pub fn inicial() -> Self {
        Self {
            rotacao_x: (-135_f32).to_radians(),
            rotacao_y: 0.0,
            rotacao_z: 0.0,
            intensidade: 15_000.0,
            vermelho: 1.0,
            verde: 1.0,
            azul: 1.0,
        }
    }
}

#[derive(Component)]
pub struct TextoRotacaoX;
#[derive(Component)]
pub struct TextoRotacaoY;
#[derive(Component)]
pub struct TextoRotacaoZ;
#[derive(Component)]
pub struct TextoIntensidade;
#[derive(Component)]
pub struct TextoVermelho;
#[derive(Component)]
pub struct TextoVerde;
#[derive(Component)]
pub struct TextoAzul;

const COR_PAINEL: Color = Color::srgb(0.12, 0.12, 0.12);
const COR_BOTAO: Color = Color::srgb(0.25, 0.25, 0.25);
const COR_TEXTO: Color = Color::srgb(0.9, 0.9, 0.9);
const COR_ROTULO: Color = Color::srgb(0.6, 0.6, 0.6);

fn spawnar_linha_controle<M, A, D>(
    pai: &mut ChildSpawnerCommands,
    rotulo: &str,
    texto_valor: &str,
    marcador_texto: M,
    marcador_aumentar: A,
    marcador_diminuir: D,
) where
    M: Component,
    A: Component,
    D: Component,
{
    pai.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        margin: UiRect::vertical(Val::Px(3.0)),
        ..default()
    }).with_children(|linha: &mut ChildSpawnerCommands| {
        linha.spawn((
            Text::new(rotulo),
            TextFont { font_size: 13.0, ..default() },
            TextColor(COR_ROTULO),
            Node { width: Val::Px(30.0), ..default() },
        ));

        linha.spawn((
            Button,
            Node {
                width: Val::Px(22.0),
                height: Val::Px(22.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COR_BOTAO),
            marcador_diminuir,
        )).with_child((
            Text::new("-"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(COR_TEXTO),
        ));

        linha.spawn((
            Text::new(texto_valor),
            TextFont { font_size: 13.0, ..default() },
            TextColor(COR_TEXTO),
            Node { width: Val::Px(60.0), ..default() },
            marcador_texto,
        ));

        linha.spawn((
            Button,
            Node {
                width: Val::Px(22.0),
                height: Val::Px(22.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COR_BOTAO),
            marcador_aumentar,
        )).with_child((
            Text::new("+"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(COR_TEXTO),
        ));
    });
}

pub fn montar_ui(mut comandos: Commands, estado_luz: Res<EstadoLuz>) {
    comandos
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            Button,
            BotaoAbrirArquivo,
        ))
        .with_child((
            Text::new("Abrir Arquivo"),
            TextFont { font_size: 18.0, ..default() },
            TextColor(Color::WHITE),
        ));

    comandos.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(12.0)),
            width: Val::Px(180.0),
            ..default()
        },
        BackgroundColor(COR_PAINEL),
    )).with_children(|painel| {
        painel.spawn((
            Text::new("Luz Direcional"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(COR_TEXTO),
            Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
        ));

        spawnar_linha_controle(
            painel, "Rot X",
            &format!("{:.0}", estado_luz.rotacao_x.to_degrees()),
            TextoRotacaoX, BotaoAumentarRotacaoX, BotaoDiminuirRotacaoX,
        );
        spawnar_linha_controle(
            painel, "Rot Y",
            &format!("{:.0}", estado_luz.rotacao_y.to_degrees()),
            TextoRotacaoY, BotaoAumentarRotacaoY, BotaoDiminuirRotacaoY,
        );
        spawnar_linha_controle(
            painel, "Rot Z",
            &format!("{:.0}", estado_luz.rotacao_z.to_degrees()),
            TextoRotacaoZ, BotaoAumentarRotacaoZ, BotaoDiminuirRotacaoZ,
        );

        spawnar_linha_controle(
            painel, "Int",
            &format!("{:.0}", estado_luz.intensidade),
            TextoIntensidade, BotaoAumentarIntensidade, BotaoDiminuirIntensidade,
        );

        spawnar_linha_controle(
            painel, "R",
            &format!("{:.2}", estado_luz.vermelho),
            TextoVermelho, BotaoAumentarVermelho, BotaoDiminuirVermelho,
        );
        spawnar_linha_controle(
            painel, "G",
            &format!("{:.2}", estado_luz.verde),
            TextoVerde, BotaoAumentarVerde, BotaoDiminuirVerde,
        );
        spawnar_linha_controle(
            painel, "B",
            &format!("{:.2}", estado_luz.azul),
            TextoAzul, BotaoAumentarAzul, BotaoDiminuirAzul,
        );
    });

    comandos.spawn((
        Text::new("WASD: mover | SPACE: subir | SHIFT: descer"),
        TextFont { font_size: 14.0, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

pub fn atualizar_controles_luz(
    interacoes: Query<(&Interaction, AnyOf<(
        &BotaoAumentarRotacaoX, &BotaoDiminuirRotacaoX,
        &BotaoAumentarRotacaoY, &BotaoDiminuirRotacaoY,
        &BotaoAumentarRotacaoZ, &BotaoDiminuirRotacaoZ,
        &BotaoAumentarIntensidade, &BotaoDiminuirIntensidade,
        &BotaoAumentarVermelho, &BotaoDiminuirVermelho,
        &BotaoAumentarVerde, &BotaoDiminuirVerde,
        &BotaoAumentarAzul, &BotaoDiminuirAzul,
    )>), Changed<Interaction>>,
    mut estado: ResMut<EstadoLuz>,
    mut luz: Query<(&mut DirectionalLight, &mut Transform)>,
    mut textos: ParamSet<(
        Query<&mut Text, With<TextoRotacaoX>>,
        Query<&mut Text, With<TextoRotacaoY>>,
        Query<&mut Text, With<TextoRotacaoZ>>,
        Query<&mut Text, With<TextoIntensidade>>,
        Query<&mut Text, With<TextoVermelho>>,
        Query<&mut Text, With<TextoVerde>>,
        Query<&mut Text, With<TextoAzul>>,
    )>,
) {
    let passo_rotacao = 5_f32.to_radians();
    let passo_intensidade = 1_000.0;
    let passo_cor = 0.05;

    let mut mudou = false;

    for (interacao, botoes) in &interacoes {
        if *interacao != Interaction::Pressed { continue; }
        mudou = true;

        if botoes.0.is_some() { estado.rotacao_x += passo_rotacao; }
        if botoes.1.is_some() { estado.rotacao_x -= passo_rotacao; }
        if botoes.2.is_some() { estado.rotacao_y += passo_rotacao; }
        if botoes.3.is_some() { estado.rotacao_y -= passo_rotacao; }
        if botoes.4.is_some() { estado.rotacao_z += passo_rotacao; }
        if botoes.5.is_some() { estado.rotacao_z -= passo_rotacao; }
        if botoes.6.is_some() { estado.intensidade = (estado.intensidade + passo_intensidade).min(200_000.0); }
        if botoes.7.is_some() { estado.intensidade = (estado.intensidade - passo_intensidade).max(0.0); }
        if botoes.8.is_some()  { estado.vermelho = (estado.vermelho + passo_cor).min(1.0); }
        if botoes.9.is_some()  { estado.vermelho = (estado.vermelho - passo_cor).max(0.0); }
        if botoes.10.is_some() { estado.verde = (estado.verde + passo_cor).min(1.0); }
        if botoes.11.is_some() { estado.verde = (estado.verde - passo_cor).max(0.0); }
        if botoes.12.is_some() { estado.azul = (estado.azul + passo_cor).min(1.0); }
        if botoes.13.is_some() { estado.azul = (estado.azul - passo_cor).max(0.0); }
    }

    if !mudou { return; }

    if let Ok((mut luz_dir, mut transform)) = luz.single_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            estado.rotacao_x,
            estado.rotacao_y,
            estado.rotacao_z,
        );
        luz_dir.illuminance = estado.intensidade;
        luz_dir.color = Color::srgb(estado.vermelho, estado.verde, estado.azul);
    }

    if let Ok(mut t) = textos.p0().single_mut() { **t = format!("{:.0}", estado.rotacao_x.to_degrees()); }
    if let Ok(mut t) = textos.p1().single_mut() { **t = format!("{:.0}", estado.rotacao_y.to_degrees()); }
    if let Ok(mut t) = textos.p2().single_mut() { **t = format!("{:.0}", estado.rotacao_z.to_degrees()); }
    if let Ok(mut t) = textos.p3().single_mut() { **t = format!("{:.0}", estado.intensidade); }
    if let Ok(mut t) = textos.p4().single_mut() { **t = format!("{:.2}", estado.vermelho); }
    if let Ok(mut t) = textos.p5().single_mut() { **t = format!("{:.2}", estado.verde); }
    if let Ok(mut t) = textos.p6().single_mut() { **t = format!("{:.2}", estado.azul); }
}
