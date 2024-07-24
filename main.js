import init, {
    generate_aes_key, encrypt_aes, decrypt_aes,
    generate_rsa_key, encrypt_rsa, decrypt_rsa
} from './pkg/crypto_learner.js';

$(document).ready(async function () {
    console.log("ready");

    await init();  // WASM モジュールの初期化

    // 鍵生成ボタン
    $('#generate_key').on('click', async function () {
        console.log("generate_key");

        const mode = $('#mode').val();
        let keyData;
        let filename;
        if (mode === 'aes') {
            keyData = generate_aes_key();
            filename = `aes-${Date.now()}.key`;
        } else if (mode === 'rsa') {
            keyData = generate_rsa_key();
            filename = `rsa-${Date.now()}.key`;
        } else {
            $('#output').html('現在サポートされていない設定です。');
            return;
        }

        // 鍵データをファイルとしてダウンロードする
        const blob = new Blob([keyData], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = $('<a>').attr('href', url).attr('download', filename).get(0);
        a.click();
        URL.revokeObjectURL(url);

        $('#output').text(`鍵ファイル: ${filename} (ダウンロードが開始されました)`);
    });

    //暗号化ボタン
    $('#encrypt').on('click', async function () {
        console.log("encrypt");

        const mode = $('#mode').val();
        const dataFile = $('#data_input')[0].files[0];
        const keyFile = $('#key_input')[0].files[0];
        if (!dataFile || !keyFile) {
            $('#output').text('データファイルと鍵ファイルの両方を選択してください。');
            return;
        }

        const data = await readFileAsText(dataFile);
        const key = await readFileAsText(keyFile);
        let result;
        if (mode === 'aes') {
            result = await encrypt_aes(data, key);
        } else if (mode === 'rsa') {
            result = await encrypt_rsa(data, key);
        } else {
            $('#output').text('現在サポートされていない設定です。');
            return;
        }

        // 元のファイル名を取得して暗号化されたファイル名を作成
        const originalFileName = dataFile.name;
        const encryptedFileName = `encrypted-${originalFileName}`;

        // 暗号化されたデータをファイルとしてダウンロードする
        const blob = new Blob([result], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = $('<a>').attr('href', url).attr('download', encryptedFileName).get(0);
        a.click();
        URL.revokeObjectURL(url);

        $('#output').text(`暗号化されたデータを ${encryptedFileName} としてダウンロードしました。`);
    });


    // 復号化ボタン
    $('#decrypt').on('click', async function () {
        console.log("decrypt");

        const mode = $('#mode').val();
        const dataFile = $('#data_input')[0].files[0];
        const keyFile = $('#key_input')[0].files[0];
        if (!dataFile || !keyFile) {
            $('#output').text('データファイルと鍵ファイルの両方を選択してください。');
            return;
        }

        const data = await readFileAsText(dataFile);
        const key = await readFileAsText(keyFile);
        let result;
        if (mode === 'aes') {
            result = await decrypt_aes(data, key);
        } else if (mode === 'rsa') {
            result = await decrypt_rsa(data, key);
        } else {
            $('#output').text('現在サポートされていない設定です。');
            return;
        }

        // 元のファイル名を取得して暗号化されたファイル名を作成
        const originalFileName = dataFile.name;
        const dencryptedFileName = `dencrypted-${originalFileName}`;

        // 暗号化されたデータをファイルとしてダウンロードする
        const blob = new Blob([result], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = $('<a>').attr('href', url).attr('download', dencryptedFileName).get(0);
        a.click();
        URL.revokeObjectURL(url);

        $('#output').text(`暗号化されたデータを ${dencryptedFileName} としてダウンロードしました。`);
    });

    // ファイルをテキストとして読み込むヘルパー関数
    function readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result);
            reader.onerror = reject;
            reader.readAsText(file);
        });
    }
});
