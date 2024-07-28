import init, {
    generate_aes_key, encrypt_aes, decrypt_aes,
    generate_rsa_key, encrypt_rsa, decrypt_rsa
} from "./pkg/crypto_learner.js";

$(document).ready(async function () {
    console.log("ready");

    await init();  // WASM モジュールの初期化

    // 鍵生成ボタン
    $("#generate_key").on("click", async function () {
        console.log("generate_key");

        const mode = $("#mode").val();
        if (mode === "aes") {
            // AES鍵生成
            const keyData = generate_aes_key();
            const filename = `aes-${getFormattedDate()}.pem`;

            // AES鍵ファイルとしてダウンロード
            const blob = new Blob([keyData], { type: "text/plain" });
            const url = URL.createObjectURL(blob);
            const a = $("<a>").attr("href", url).attr("download", filename).get(0);
            a.click();
            URL.revokeObjectURL(url);

            $("#output").text(`鍵ファイル: ${filename} (ダウンロードが開始されました)`);
        } else if (mode === "rsa") {
            try {
                // RSA鍵生成
                const keys = await generate_rsa_key();
                const publicKey = keys.public_key;
                const privateKey = keys.private_key;

                // 公開鍵と秘密鍵をそれぞれファイルとしてダウンロード
                const publicBlob = new Blob([publicKey], { type: "text/plain" });
                const privateBlob = new Blob([privateKey], { type: "text/plain" });

                const publicUrl = URL.createObjectURL(publicBlob);
                const privateUrl = URL.createObjectURL(privateBlob);

                // 公開鍵ファイルのダウンロード
                const publicA = $("<a>").attr("href", publicUrl).attr("download", `rsa-public-${getFormattedDate()}.pem`).get(0);
                publicA.click();

                // 秘密鍵ファイルのダウンロード
                const privateA = $("<a>").attr("href", privateUrl).attr("download", `rsa-private-${getFormattedDate()}.pem`).get(0);
                privateA.click();

                URL.revokeObjectURL(publicUrl);
                URL.revokeObjectURL(privateUrl);

                $("#output").text(`公開鍵と秘密鍵ファイルのダウンロードが開始されました`);
            } catch (error) {
                $("#output").text(`エラーが発生しました: ${error.message}`);
            }
        } else {
            $("#output").html("現在サポートされていない設定です。");
        }
    });

    // 暗号化ボタン
    $("#encrypt").on("click", async function () {
        console.log("encrypt");

        const mode = $("#mode").val();
        const dataFile = $("#data_input")[0].files[0];
        const keyFile = $("#key_input")[0].files[0];
        if (!dataFile || !keyFile) {
            $("#output").text("データファイルと鍵ファイルの両方を選択してください。");
            return;
        }

        const data = await readFileAsText(dataFile);
        const key = await readFileAsText(keyFile);
        let result;
        if (mode === "aes") {
            result = await encrypt_aes(data, key);
        } else if (mode === "rsa") {
            result = await encrypt_rsa(data, key);
        } else {
            $("#output").text("現在サポートされていない設定です。");
            return;
        }

        // 元のファイル名を取得して暗号化されたファイル名を作成
        const originalFileName = dataFile.name;
        const encryptedFileName = `encrypted-${originalFileName}`;

        // 暗号化されたデータをファイルとしてダウンロードする
        const blob = new Blob([result], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = $("<a>").attr("href", url).attr("download", encryptedFileName).get(0);
        a.click();
        URL.revokeObjectURL(url);

        $("#output").text(`暗号化されたデータを ${encryptedFileName} としてダウンロードしました。`);
    });

    // 復号化ボタン
    $("#decrypt").on("click", async function () {
        console.log("decrypt");

        const mode = $("#mode").val();
        const dataFile = $("#data_input")[0].files[0];
        const keyFile = $("#key_input")[0].files[0];
        if (!dataFile || !keyFile) {
            $("#output").text("データファイルと鍵ファイルの両方を選択してください。");
            return;
        }

        const data = await readFileAsText(dataFile);
        const key = await readFileAsText(keyFile);
        let result;
        if (mode === "aes") {
            result = await decrypt_aes(data, key);
        } else if (mode === "rsa") {
            result = await decrypt_rsa(data, key);
        } else {
            $("#output").text("現在サポートされていない設定です。");
            return;
        }

        // 元のファイル名を取得して暗号化されたファイル名を作成
        const originalFileName = dataFile.name;
        const dencryptedFileName = `dencrypted-${originalFileName}`;

        // 復号化されたデータをファイルとしてダウンロードする
        const blob = new Blob([result], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = $("<a>").attr("href", url).attr("download", dencryptedFileName).get(0);
        a.click();
        URL.revokeObjectURL(url);

        $("#output").text(`復号化されたデータを ${dencryptedFileName} としてダウンロードしました。`);
    });

});

// 現在時刻をフォーマットして取得する関数
function getFormattedDate() {
    const now = new Date();

    const year = now.getFullYear();
    const month = String(now.getMonth() + 1).padStart(2, "0");
    const day = String(now.getDate()).padStart(2, "0");
    const hours = String(now.getHours()).padStart(2, "0");
    const minutes = String(now.getMinutes()).padStart(2, "0");
    const seconds = String(now.getSeconds()).padStart(2, "0");
    // 毎秒単位のミリ秒数を取り、ゼロパディングする
    const milliseconds = String(now.getMilliseconds()).padStart(3, "0").substring(0, 2);

    // 形式を "yyyymmddhhmmss" にする
    return `${year}${month}${day}${hours}${minutes}${seconds}${milliseconds}`;
}

// ファイルをテキストとして読み込む関数
function readFileAsText(file) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsText(file);
    });
}
