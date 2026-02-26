pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) { // Write: anything you can write bytes into
    for line in content.lines(){
        if line.contains(pattern){}
        println!("{}", line);
    }

}
