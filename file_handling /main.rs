mod mods;
fn main() {
    let msg_write_all = b"Hello from write_all()!";
    let msg_write = b"Hello from write()!";
    let file_to_read = "./example.txt";

    mods::file_mod::create_files::create_file_write_all(file_to_read, msg_write_all);
    // Create file using the write()
    mods::file_mod::create_files::create_file_write(file_to_read, msg_write);
    println!("Hello Modules");
}
