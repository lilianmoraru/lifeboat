use std::process::Command;

fn main() {
    // Checking that go is available
    {
        let mut go_cmd = Command::new("go");
        run(go_cmd.arg("version"), "go");
    }

    // Compile "ipfs"
    {
        let mut compile_ipfs = Command::new("go");
        compile_ipfs.args(&["get", "-u", "github.com/ipfs/go-ipfs"]);

        ::std::env::set_var("GOPATH", ::std::env::var("OUT_DIR").unwrap());
        run(&mut compile_ipfs, "go(IO or Network issue)");
    }
}

fn run(cmd: &mut Command, program: &str) {
    use std::io::ErrorKind;

    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
