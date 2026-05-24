use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const SCRIPT_NAME: &str = "print-satty.sh";

fn bin_dir() -> Result<PathBuf, std::io::Error> {
    let exe = env::current_exe()?;
    let parent = exe.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Diretório pai não encontrado")
    })?;

    Ok(parent.to_path_buf())
}

fn write_script(target: &Path) -> Result<(), std::io::Error> {
    let script = include_str!("../print-satty.sh");
    fs::write(target, script)?;

    let mut perms = fs::metadata(target)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(target, perms)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let dir = bin_dir()?;
    let script_path = dir.join(SCRIPT_NAME);

    write_script(&script_path)?;

    let path_str = script_path.display().to_string();
    let padding_len = 46_usize.saturating_sub(4 + path_str.len());
    let padding = " ".repeat(padding_len);

    println!();
    println!("  ╔══════════════════════════════════════════════╗");
    println!("  ║  Screenshot script installed!                ║");
    println!("  ╠══════════════════════════════════════════════╣");
    println!("  ║  Run this command to take a screenshot:      ║");
    println!("  ║                                              ║");
    println!("  ║    {}{}║", path_str, padding);
    println!("  ║                                              ║");
    println!("  ║  Requires: grim, slurp, satty, wl-clipboard  ║");
    println!("  ╚══════════════════════════════════════════════╝");
    println!();

    Ok(())
}
