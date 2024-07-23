import init, { 
    generate_aes_key, encrypt_aes, decrypt_aes, 
    generate_rsa_key, encrypt_rsa, decrypt_rsa 
} from './pkg/crypto_wasm.js';

async function run() {
    await init();  // WASM モジュールの初期化

    const modeSelect = document.getElementById('mode');
    const inputFile = document.getElementById('input');
    const generateKeyBtn = document.getElementById('generate_key');
    const encryptBtn = document.getElementById('encrypt');
    const decryptBtn = document.getElementById('decrypt');
    const outputDiv = document.getElementById('output');

    // 鍵生成ボタン
    generateKeyBtn.addEventListener('click', () => {
        const mode = modeSelect.value;
        let filename;
        if (mode === 'aes') {
            filename = generate_aes_key();
        } else if (mode === 'rsa') {
            filename = generate_rsa_key();
        }
        outputDiv.textContent = `生成した鍵ファイル: ${filename}`;
    });

    // 暗号化ボタン
    encryptBtn.addEventListener('click', async () => {
        const mode = modeSelect.value;
        const file = inputFile.files[0];
        if (!file) {
            outputDiv.textContent = 'ファイルを選択してください。';
            return;
        }

        const reader = new FileReader();
        reader.onload = async () => {
            const text = reader.result;
            let result;
            const keyFile = 'ファイル名をここに';  // 適切なファイル名を設定する必要があります
            if (mode === 'aes') {
                result = encrypt_aes(text, keyFile);
            } else if (mode === 'rsa') {
                result = encrypt_rsa(text, keyFile);
            }
            outputDiv.textContent = `暗号化されたデータ: ${result}`;
        };
        reader.readAsText(file);
    });

    // 復号化ボタン
    decryptBtn.addEventListener('click', async () => {
        const mode = modeSelect.value;
        const file = inputFile.files[0];
        if (!file) {
            outputDiv.textContent = 'ファイルを選択してください。';
            return;
        }

        const reader = new FileReader();
        reader.onload = async () => {
            const text = reader.result;
            let result;
            const keyFile = 'ファイル名をここに';  // 適切なファイル名を設定する必要があります
            if (mode === 'aes') {
                result = decrypt_aes(text, keyFile);
            } else if (mode === 'rsa') {
                result = decrypt_rsa(text, keyFile);
            }
            outputDiv.textContent = `復号化されたデータ: ${result}`;
        };
        reader.readAsText(file);
    });
}

run();
