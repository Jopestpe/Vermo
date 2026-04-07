use bevy::prelude::*;
use bevy::asset::io::{AssetReader, AssetReaderError, AssetSourceBuilder, AssetSourceId, PathStream, Reader};
use wasm_bindgen::prelude::*;

const TAMANHO_MAXIMO_BYTES: u32 = 500 * 1024 * 1024;

pub mod fila_arquivos_web {
    use std::sync::Mutex;

    pub struct ArquivoRecebido {
        pub bytes: Vec<u8>,
    }

    static FILA: Mutex<Vec<ArquivoRecebido>> = Mutex::new(Vec::new());

    pub fn enfileirar(bytes: Vec<u8>) {
        if let Ok(mut fila) = FILA.lock() {
            fila.push(ArquivoRecebido { bytes });
        }
    }

    pub fn esvaziar() -> Vec<ArquivoRecebido> {
        FILA.lock()
            .map(|mut fila| fila.drain(..).collect())
            .unwrap_or_default()
    }
}

pub mod fila_comandos_web {
    use std::sync::Mutex;

    pub enum ComandoWeb {
        Rotacao(f32, f32, f32),
        Intensidade(f32),
        Cor(f32, f32, f32),
        Escala(f32),
    }

    static FILA: Mutex<Vec<ComandoWeb>> = Mutex::new(Vec::new());

    pub fn enfileirar(cmd: ComandoWeb) {
        if let Ok(mut fila) = FILA.lock() {
            fila.push(cmd);
        }
    }

    pub fn esvaziar() -> Vec<ComandoWeb> {
        FILA.lock()
            .map(|mut fila| fila.drain(..).collect())
            .unwrap_or_default()
    }
}

#[wasm_bindgen]
pub fn bevy_carregar_modelo(bytes: js_sys::Uint8Array) {
    if bytes.length() > TAMANHO_MAXIMO_BYTES {
        web_sys::console::error_1(&"Arquivo muito grande (limite: 500MB)".into());
        return;
    }
    fila_arquivos_web::enfileirar(bytes.to_vec());
}

#[wasm_bindgen]
pub fn bevy_definir_luz_rotacao(x: f32, y: f32, z: f32) {
    fila_comandos_web::enfileirar(fila_comandos_web::ComandoWeb::Rotacao(x, y, z));
}

#[wasm_bindgen]
pub fn bevy_definir_luz_intensidade(v: f32) {
    fila_comandos_web::enfileirar(fila_comandos_web::ComandoWeb::Intensidade(v));
}

#[wasm_bindgen]
pub fn bevy_definir_luz_cor(r: f32, g: f32, b: f32) {
    fila_comandos_web::enfileirar(fila_comandos_web::ComandoWeb::Cor(r, g, b));
}

#[wasm_bindgen]
pub fn bevy_definir_modelo_escala(v: f32) {
    fila_comandos_web::enfileirar(fila_comandos_web::ComandoWeb::Escala(v));
}

#[derive(Default, Clone)]
pub struct LeitorEmMemoria {
    pub bytes_do_modelo: std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>,
}

impl LeitorEmMemoria {
    pub fn substituir_modelo(&self, novos_bytes: Vec<u8>) {
        if let Ok(mut bytes) = self.bytes_do_modelo.lock() {
            *bytes = Some(novos_bytes);
        }
    }
}

impl AssetReader for LeitorEmMemoria {
    async fn read<'a>(&'a self, _caminho: &'a std::path::Path) -> Result<impl Reader + 'a, AssetReaderError> {
        use bevy::asset::io::VecReader;
        let guard = self.bytes_do_modelo.lock()
            .unwrap_or_else(|envenenado| envenenado.into_inner());
        let bytes = guard.clone()
            .ok_or_else(|| AssetReaderError::NotFound("modelo_em_memoria".into()))?;
        Ok(VecReader::new(bytes))
    }
    async fn read_meta<'a>(&'a self, caminho: &'a std::path::Path) -> Result<impl Reader + 'a, AssetReaderError> {
        use bevy::asset::io::VecReader;
        Err::<VecReader, _>(AssetReaderError::NotFound(caminho.into()))
    }
    async fn read_directory<'a>(&'a self, caminho: &'a std::path::Path) -> Result<Box<PathStream>, AssetReaderError> {
        Err(AssetReaderError::NotFound(caminho.into()))
    }
    async fn is_directory<'a>(&'a self, _caminho: &'a std::path::Path) -> Result<bool, AssetReaderError> {
        Ok(false)
    }
}

#[derive(Resource, Clone)]
pub struct LeitorModeloWeb(pub LeitorEmMemoria);

pub fn registrar_fonte_de_assets(aplicacao: &mut App) {
    let leitor = LeitorEmMemoria::default();
    let leitor_para_source = leitor.clone();
    aplicacao
        .insert_resource(LeitorModeloWeb(leitor))
        .register_asset_source(
            AssetSourceId::Name("mem".into()),
            AssetSourceBuilder::new(move || Box::new(leitor_para_source.clone())),
        );
}
