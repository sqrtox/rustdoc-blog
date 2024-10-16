//! Rustdocという静的サイトジェネレータを使用したブログです🙌

/// # Rustdocブログはじめました
///
/// (大嘘)
///
/// ## コードシンタックスハイライト
///
/// Rustのコードシンタックスハイライト完備です。
/// ```
/// println!("Hello, world!");
///
/// match opt {
///   Some(v) => v,
///   None => {}
/// }
/// ```
///
/// ```js
/// console.log("Hello, world!"); // JavaScript
/// ```
/// 残念ながら他の言語はハイライトされません。
///
/// ## マークダウン
///
/// RustdocはCommonMarkに基づいています。
///
/// **bold**
/// ~~strikethrough~~
/// *italic*
///
/// - a
/// - b
/// - c
///   - d
///
/// ## ビルド
///
/// ビルドにはRustが必要です。
/// `cargo doc`でできます。
/// また、ホットリロードのようなことはできません。記事に変更を加えたら自分でビルドし直す必要があります。
pub const 最初にして唯一の記事: u32 = 0;
