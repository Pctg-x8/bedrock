const path = require("path");
const fs = require("fs");

const docpath = path.join("docs", "ja");
let conversions = {};
conversions[path.join("ferrite", "index.html")] = data => data
    .replace("Glue library between Vulkan and Rust", "Ferrite: Vulkan と Rust のグルーライブラリ (部分的に日本語版)")
    .replace("Some documentation comments are from Vulkan Manual Page.", "いくつかのドキュメントコメントは Vulkan のマニュアルページの簡易翻訳である。")
    .replace("Enable Vulkan implementations(functions)", "Vulkan の実装(関数)を有効にする")
    .replace("Enables to use objects from some threads(experimental)", "いくつかのオブジェクトを複数スレッドから使えるようにする(実験的)")
    .replace("Enable rendering features to Window/Display", "ウィンドウ/ディスプレイへの描画機能を有効にする")
    .replace("Enable Vulkan extensions(same name as each extensions)", "Vulkan の機能を有効にする(拡張名がそのまま使える)")
    .replace("Compile Options", "コンパイルオプション").replace("All of traits", "全トレイト").replace("Vulkan API Definitions", "Vulkan の API 定義");
conversions[path.join("ferrite", "vk", "struct.VkAttachmentDescription.html")] = data => data
    .replace("A bitmask of <code>VkAttachmentDescriptionFlagBits</code> specifying additional properties of the attachment.", "<code>VkAttachmentDescriptionFlagBits</code>のビットマスクで、アタッチメントの追加プロパティを指定する")
    .replace(/Possible Bitmasks/g, "適用可能なビットマスク").replace(/Possible Values/g, "適用可能な値").replace(/Access Type Requirements/g, "必要なアクセスタイプ")
    .replace("No flags", "指定なし").replace("The attachment aliases the same device memory as other attachments", "このアタッチメントは同一メモリ上の他のアタッチメントをエイリアスしていることを表す")
    .replace("A <code>VkFormat</code> value specifying the format of the image that will be used for the attachment", "アタッチメントとして使用されるイメージのピクセルフォーマットを<code>VkFormat</code>で指定する")
    .replace("The number of samples of the image as defined in <code>VkSampleCountFlagBits</code>", "<code>VkSampleCountFlagBits</code>で定義される、イメージのサンプル数")
    .replace("A <code>VkAttachmentLoadOp</code> value specifying how the contents of color and depth components of the attachment are\ntreated at the beginning of the subpass where it is first used",
        "アタッチメントの色/深度が、最初に使用する Subpass の開始時にどうあるべきかを指定する")
    .replace("A <code>VkAttachmentStoreOp</code> value specifying how the contents of color and depth components of the attachment are\ntreated at the end of the subpass where it is last used.",
        "アタッチメントの色/深度を、最後に使用した Subpass の終了時にどう扱うかを指定する")
    .replace("A <code>VkAttachmentLoadOp</code> value specifying how the contents of stencil components of the attachment are\ntreated at the beginning of the subpass where it is first used",
        "アタッチメントのステンシル値が、最初に使用する Subpass の開始時にどうあるべきかを指定する")
    .replace("A <code>VkAttachmentStoreOp</code> value specifying how the contents of stencil components of the attachment are\ntreated at the end of the subpass where it is last used.",
        "アタッチメントのステンシル値を、最後に使用した Subpass の終了時にどう扱うかを指定する")
    .replace(/The previous contents of the image within the render area will be preserved\./g, "以前のイメージの内容を引き継ぐことを表す")
    .replace(/The contents within the render area will be cleared to a uniform value,[^]which is specified when a render pass instance is begun\./gm,
        "特定の値(RenderPass を開始する際に指定)でクリアされることを表す")
    .replace(/The previous contents within the area need not be preserved;[^]the contents of the attachment will be undefined inside the render area\./gm,
        "以前の内容が引き継がれる必要はないことを表す。描画エリア内のアタッチメントの内容は未定義となる")
    .replace(/The contents generated during the render pass and within the render area are[^]written to memory\./mg,
        "描画エリア内において、パス中で生成された内容はメモリに書き戻されるようにする")
    .replace(/The contents within the render area are not needed after rendering,[^]and <em>may<\/em> be discarded; the contents of the attachment will be undefined inside the render area\./gm,
        "描画エリア内の内容は今後必要がなく、破棄されても構わないことを表す。アタッチメントの描画エリア内の内容は未定義となる。")
    .replace(/For attachments with a depth format/g, "深度を持つアタッチメントの場合")
    .replace(/For attachments with a color format/g, "色を持つアタッチメントの場合")
    .replace(/Both values require <code>(.+(?=<\/code>))<\/code>/g, "どちらの値も<code>$1</code>を必要とする")
    .replace(/<code>(.+(?=<\/code>))<\/code> requires <code>(.+(?=<\/code>))<\/code>/g, "<code>$1</code>は<code>$2</code>を必要とする")
    .replace("The layout the attachment image subresource will be in when a render pass instance begins.", "RenderPass が開始する際のイメージレイアウト")
    .replace("The layout the attachment image subresource will be transitioned to when a render pass instance ends.", "RenderPass が終了する際に遷移するイメージレイアウト。")
    .replace("During a render pass instance, an attachment <em>can</em> use a different layout in each subpass, if desired.", "必要であれば、同一の RenderPass の各 Subpass で異なるイメージレイアウトを使うことができる。");
conversions[path.join("ferrite", "struct.SubpassDescription.html")] = data => data
    .replace("Builder structure to construct the <code>VkSubpassDescription</code>", "<code>VkSubpassDescription</code>のビルダーオブジェクト")
    .replace("The <code>layout</code> parameter of each attachment", "各アタッチメントの<code>layout</code>パラメータについて")
    .replace("The <code>layout</code> parameter describes what layout the attachment will be in during the subpass", "<code>layout</code>パラメータには、Subpass 中でアタッチメントがとるべきイメージレイアウトを指定する")
    .replace("How <em>input attachments</em> work", "<em>入力アタッチメント</em>の挙動")
    .replace("Each element of the array corresponds to an input attachment unit number in the shader.",
        "配列の各要素は、シェーダにおける入力アタッチメントユニットの番号に対応している。")
    .replace("i. e. if the shader declares an input variable <code>layout(input_attachment_index=X, set=Y, binding=Z)</code>\nthen it uses the attachment provided in <code>input_attachments[X]</code>.",
        "例: シェーダ内で<code>layout(input_attachment_index=X, set=Y, binding=Z)</code>と指定された入力変数は、<code>add_input(X, ...)</code>にて指定されたアタッチメントを使用する。")
    .replace("Input attachments <em>must</em> also be bound to the pipeline with a descriptor set, with the input attachment descriptor\nwritten in the location (set=Y, binding=Z).",
        "また、入力アタッチメントは Descriptor Set を通してパイプラインにバインドされている必要がある。このとき、入力アタッチメントデスクリプタは(set=Y, binding=Z)の位置に置かれている必要がある。")
    .replace("Fragment shaders <em>can</em> use subpass input variables to access the contents of an input attachment at the fragment&#39;s\n(x, y, layer) framebuffer coordinates.",
        "フラグメントシェーダは、サブパス入力変数(subpass input variables)を通して入力アタッチメントの内容にアクセスできる。ただし、シェーダが現在処理しているフレームバッファ上の座標点(x, y, layer)の内容のみ。");

for(conv_path in conversions)
{
    let fp = path.join(docpath, conv_path);
    console.log(`Translating ${fp}...`);
    let data = fs.readFileSync(fp, 'utf8');
    let conv_data = conversions[conv_path](data);
    fs.writeFileSync(fp, conv_data, 'utf8');
}