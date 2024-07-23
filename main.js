// Import the necessary functions from the compiled WASM module
import init, { generate_aes_key, encrypt_aes, decrypt_aes, generate_rsa_key, encrypt_rsa, decrypt_rsa } from './pkg/crypto_learner.js';

// Initialize the application
async function initApp() {
    // Initialize the WASM module
    await init();

    // Get references to HTML elements
    const modeSelect = document.getElementById('mode');
    const fileInput = document.getElementById('input');
    const generateKeyButton = document.getElementById('generate_key');
    const encryptButton = document.getElementById('encrypt');
    const decryptButton = document.getElementById('decrypt');
    const outputDiv = document.getElementById('output');

    let keyFileName = '';

    // Add event listener for the "鍵生成" button
    generateKeyButton.addEventListener('click', async () => {
        const mode = modeSelect.value;
        if (mode === 'aes') {
            // Generate an AES key and display the filename
            keyFileName = generate_aes_key();
            outputDiv.textContent = `AES鍵が生成されました: ${keyFileName}`;
        } else if (mode === 'rsa') {
            // Generate RSA keys and display the filename
            keyFileName = generate_rsa_key();
            outputDiv.textContent = `RSA鍵が生成されました: ${keyFileName}`;
        }
    });

    // Add event listener for the "暗号化する" button
    encryptButton.addEventListener('click', async () => {
        const mode = modeSelect.value;
        const file = fileInput.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = async () => {
                const text = reader.result;
                let encryptedText = '';
                if (mode === 'aes') {
                    // Encrypt the text using AES and display the result
                    encryptedText = encrypt_aes(text, keyFileName);
                } else if (mode === 'rsa') {
                    // Encrypt the text using RSA and display the result
                    encryptedText = encrypt_rsa(text, keyFileName);
                }
                outputDiv.textContent = `暗号化されたテキスト: ${encryptedText}`;
            };
            reader.readAsText(file);
        }
    });

    // Add event listener for the "復号する" button
    decryptButton.addEventListener('click', async () => {
        const mode = modeSelect.value;
        const file = fileInput.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = async () => {
                const encryptedText = reader.result;
                let decryptedText = '';
                if (mode === 'aes') {
                    // Decrypt the text using AES and display the result
                    decryptedText = decrypt_aes(encryptedText, keyFileName);
                } else if (mode === 'rsa') {
                    // Decrypt the text using RSA and display the result
                    decryptedText = decrypt_rsa(encryptedText, keyFileName);
                }
                outputDiv.textContent = `復号されたテキスト: ${decryptedText}`;
            };
            reader.readAsText(file);
        }
    });

    // Add event listener for the "QRコード生成" button
    const qrButton = document.getElementById('generate_qr');
    qrButton.addEventListener('click', async () => {
        // Placeholder for QR code generation
        outputDiv.textContent = 'QRコード生成機能はまだ実装されていません。';
    });
}

// Initialize the app
initApp();
