class EntradaNumerica extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });

        const value = parseFloat(this.getAttribute('value')) || 0;
        const min = parseFloat(this.getAttribute('min')) || 0;
        const max = parseFloat(this.getAttribute('max')) || 1000;
        const step = parseFloat(this.getAttribute('step')) || 1;

        this.valor = value;

        const style = document.createElement('style');
        style.textContent = `
        :host {
            display: inline-block;
        }
        .entrada-numerica {
            display: flex;
            aspect-ratio: 3 / 1;
            height: var(--altura-4);
            border: var(--borda-normal);
            border-radius: var(--raio-borda);
            font-family: var(--font-family, 'Cantarell', 'Segoe UI', 'Tahoma', sans-serif);
            font-size: var(--tamanho-fonte);
            color: var(--branco);
            background-color: transparent;
            box-sizing: border-box;
            transition: border 0.2s;
        }
        .entrada-numerica:focus-within {
            border: var(--borda-destaque);
        }
        .entrada-numerica * {
            appearance: none;
            -moz-appearance: textfield;
            width: 33.33%;
            height: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
            background-color: transparent;
            border: none;
            font-family: inherit;
            font-size: inherit;
            color: inherit;
            outline: none;
        }
        input::-webkit-outer-spin-button,
        input::-webkit-inner-spin-button {
            -webkit-appearance: none;
            margin: 0;
        }
        .entrada-numerica button:hover {
            cursor: pointer;
            color: var(--azul);
        }`;

        const container = document.createElement('div');
        container.className = 'entrada-numerica';

        const btnMenos = document.createElement('button');
        btnMenos.textContent = '◀';
        const input = document.createElement('input');
        input.type = 'number';
        input.value = value;
        input.min = min;
        input.max = max;
        const btnMais = document.createElement('button');
        btnMais.textContent = '▶';

        container.append(btnMenos, input, btnMais);
        this.shadowRoot.append(style, container);

        const atualizar = (novoValor) => {
            if (isNaN(novoValor)) novoValor = min;
            if (novoValor < min) novoValor = min;
            if (novoValor > max) novoValor = max;
            this.valor = novoValor;
            input.value = novoValor;
            this.dispatchEvent(new CustomEvent('change', { detail: novoValor }));
            this.dispatchEvent(new CustomEvent('input', { detail: novoValor }));
        }

        let intervalo;
        const repetir = (fn) => intervalo = setInterval(fn, 100);
        const parar = () => clearInterval(intervalo);

        btnMais.addEventListener('click', () => atualizar(parseFloat(input.value) + step));
        btnMenos.addEventListener('click', () => atualizar(parseFloat(input.value) - step));

        btnMais.addEventListener('mousedown', () => repetir(() => atualizar(parseFloat(input.value) + step)));
        btnMenos.addEventListener('mousedown', () => repetir(() => atualizar(parseFloat(input.value) - step)));

        [btnMais, btnMenos].forEach(b => {
            b.addEventListener('mouseup', parar);
            b.addEventListener('mouseleave', parar);
        });

        input.addEventListener('input', () => atualizar(parseFloat(input.value)));
    }

    get value() {
        return this.valor;
    }

    set value(v) {
        this.valor = v;
        this.shadowRoot.querySelector('input').value = v;
    }
}

customElements.define('entrada-numerica', EntradaNumerica);
