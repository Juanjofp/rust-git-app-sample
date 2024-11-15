use git_info::GitInfo;

fn main() {
    println!("Hello, world!");

    let git_info = GitInfo::new(String::from(""));

    let me = git_info.me();

    let Ok(me) = me else {
        println!("Error: {:?}", me);
        return;
    };

    println!("Me: {}", me.name);
}
