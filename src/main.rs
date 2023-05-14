mod init;

use init::get_coords;

#[tokio::main]
async fn main() {
    let coords = get_coords().await.expect("valid coords");
    println!("{} {}", coords.0, coords.1);
}
