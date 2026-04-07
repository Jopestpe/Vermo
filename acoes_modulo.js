import init, { bevy_carregar_modelo, bevy_definir_luz_rotacao, bevy_definir_luz_intensidade, bevy_definir_luz_cor, bevy_definir_modelo_escala } from './vermo_bevy.js';
await init();

document.getElementById('botao_importar').addEventListener('click', () => {
  const seletor = document.createElement('input');
  seletor.type = 'file';
  seletor.accept = '.glb';
  seletor.onchange = async e => {
    const arq = e.target.files[0];
    if(!arq) return;
    bevy_carregar_modelo(new Uint8Array(await arq.arrayBuffer()));
  };
  seletor.click();
});

document.getElementById('botao_cor_sol').addEventListener('click', () => { entrada_cor_sol.click(); });
document.getElementById('entrada_cor_sol').addEventListener('input', e => {
  const hex = e.target.value;
  bevy_definir_luz_cor(
    parseInt(hex.slice(1,3),16)/255,
    parseInt(hex.slice(3,5),16)/255,
    parseInt(hex.slice(5,7),16)/255,
  );
  document.getElementById('botao_cor_sol').style.backgroundColor = hex;
});

const rot_x = document.getElementById('rotacao_x');
const rot_y = document.getElementById('rotacao_y');
const rot_z = document.getElementById('rotacao_z');
const intensidade = document.getElementById('intensidade');

rot_x.addEventListener('change', e => bevy_definir_luz_rotacao(
  parseFloat(e.target.value) * Math.PI/180,
  parseFloat(rot_y.value) * Math.PI/180,
  parseFloat(rot_z.value) * Math.PI/180
));
rot_y.addEventListener('change', e => bevy_definir_luz_rotacao(
  parseFloat(rot_x.value) * Math.PI/180,
  parseFloat(e.target.value) * Math.PI/180,
  parseFloat(rot_z.value) * Math.PI/180
));
rot_z.addEventListener('change', e => bevy_definir_luz_rotacao(
  parseFloat(rot_x.value) * Math.PI/180,
  parseFloat(rot_y.value) * Math.PI/180,
  parseFloat(e.target.value) * Math.PI/180
));

intensidade.addEventListener('change', e => bevy_definir_luz_intensidade(parseFloat(e.target.value)));

const escala = document.getElementById('escala');
escala.addEventListener('input', e => bevy_definir_modelo_escala(e.detail));
