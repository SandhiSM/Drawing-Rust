# 今後追加する機能

```Rust main.rs
fn exist_checker(file_name: &str) -> bool {
    // ファイルが存在するかの判定 実装完了
}
fn mode_branch(mut image: drawing::Draw) {
    match &*mode {
        "load" => (),
        "temporary_save" => (),
        "save -e" => (),
    }
}
// 実装完了
```

```Rust drawing.rs
pub fn load(name: &str, file_format: &str){
    // 下記temporary_saveで保存したファイルの読み出し、及びDrawの情報更新 実装完了
}
pub fn show(&self) {
    // ウィンドウに画像を表示
    // 現在はconfirm関数で代用中
}
pub fn temporary_save(&self){
    // Draw.pictureの情報を何らかのファイルに保存したい 実装完了
}
pub fn save(&mut self, edible: bool) {
    let file_name = format!(r".\output\completed\{}.{}", self.title, self.file_format);
    self.image.save(file_name).expect("Failed to save.");
    println!("Saved.");
    if ediable {
        self.temporary_save();
    }
    // 上記temporary_save関数が閑静っすればこれは可能 実装完了
}
pub fn show_color_sample(&self, rgba: [u8; 4]) {
    // ウィンドウに指定した色の画像を表示
    // 現在はconfirm_color_sampleで代用中
}
```

**要するに、ファイル保存の方法がわかれば良い。**
