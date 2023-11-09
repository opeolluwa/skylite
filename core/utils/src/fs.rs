// pub mod storage_information;
/// a function to compute file size
/// accept files size in byte and parse it to human readable KB, MB, TB, GB e.t.c
pub fn compute_file_size(size: u128) -> String {
    if size > (1024 * 1024 * 1024 * 1024) {
        format!("{:.2} TB", size / (1024 * 1024 * 1024 * 1024))
    } else if size > (1024 * 1024 * 1024) {
        format!("{:.2} GB", size / (1024 * 1024 * 1024))
    } else if size > (1024 * 1024) {
        format!("{:.2} MB", size / (1024 * 1024))
    } else if size > 1024 {
        format!("{:.2} KB", size / (1024))
    } else {
        format!("{:.2} B", size)
    }
}
