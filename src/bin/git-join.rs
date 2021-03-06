extern crate grib;
extern crate clap;
extern crate git2;
use grib::Shell;
use grib::scratch;
use std::path::Path;

fn main()
{
    let args = clap::App::new("git-join")
        .arg(clap::Arg::with_name("source").long("source").takes_value(true))
        .arg(clap::Arg::with_name("branch").long("branch").takes_value(true))
        .arg(clap::Arg::with_name("subdir").long("subdir").takes_value(true))
        .get_matches();

    let branch = args.value_of("branch").expect("missing branch");
    let source = args.value_of("source").expect("missing source");
    let subdir = args.value_of("subdir").expect("missing subdir");

    let td = Path::new("/tmp/git-join2/");
    let scratch = scratch::new(&td.join("scratch"));
    let repo = git2::Repository::open(".").expect("can't open repo");
    let central_head = repo.revparse_single(branch).expect("can't find branch");
    let shell = Shell {
        cwd: scratch.path().to_path_buf(),
    };
    scratch
        .find_reference("refs/heads/join_source")
        .map(|mut r| {
            r.delete().ok();
        })
        .ok();
    shell.command(&format!("git fetch {} {}:join_source", source, branch));
    scratch::transfer(&scratch, &format!("{}", central_head.id()), &Path::new("."));
    let module_head = scratch
        .revparse_single("join_source")
        .expect("can'f find join_source");

    let signature = scratch.signature().unwrap();
    let result = scratch::join_to_subdir(
        &scratch,
        central_head.id(),
        &Path::new(subdir),
        module_head.id(),
        &signature,
    );

    scratch
        .find_reference("refs/heads/result")
        .map(|mut r| {
            r.delete().ok();
        })
        .ok();
    scratch
        .reference("refs/heads/join_result", result, true, "join")
        .ok();
    let shell = Shell {
        cwd: Path::new(".").to_path_buf(),
    };
    repo.find_reference("refs/heads/join")
        .map(|mut r| {
            r.delete().ok();
        })
        .ok();
    shell.command(&format!("git fetch {:?} join_result:join", scratch.path()));
}
