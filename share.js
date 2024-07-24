$(document).ready(async function () {
    console.log("ready");

    $("#generate_file").on("click", async function () {
        console.log("generate_file");

        let input = $("#shared_text_input").val();

        // テキストを行ごとに分割
        let lines = input.split('\n');

        // 一行目とそれ以降の部分を取り出す
        let filename = lines[0];
        let data = lines.slice(1).join('\n');

        const blob = new Blob([data], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = $("<a>").attr("href", url).attr("download", filename).get(0);
        a.click();
        URL.revokeObjectURL(url);
        $("#output").text(`ファイル: ${filename} (ダウンロードが開始されました)`);
    });

    $("#generate_qr").on("click", async function () {
        console.log("generate_qr");

        let dataFile = $("#share_date_input")[0].files[0];
        if (!dataFile) {
            $("#output").text("ファイルを選択してください。");
            return;
        }

        let filename = dataFile.name;
        let data = await readFileAsText(dataFile);
        
        let text = filename + "\n" + data;

        // QRコードを生成するために div をリセット
        $("#output").empty();

        // QRコードを生成して表示
        $("#output").qrcode({
            text: text,
            width: 200,
            height: 200
        });
    });

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