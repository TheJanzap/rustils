use std::{fs::Permissions, os::unix::fs::PermissionsExt};

fn digit_to_permission(digit: &str) -> Result<String, String> {
    if digit.len() != 1 || !digit.chars().all(char::is_numeric) {
        return Err(format!("{digit} is not a valid permission digit"));
    }

    Ok(match digit {
        "1" => "--x",
        "2" => "-w-",
        "3" => "-wx",
        "4" => "r--",
        "5" => "r-x",
        "6" => "rw-",
        "7" => "rwx",
        _ => "---",
    }
    .to_string())
}

fn type_to_permission(r#type: &str) -> Result<char, String> {
    if r#type.len() != 3 || !r#type.chars().all(char::is_numeric) {
        return Err(format!("{type} is not a valid type string"));
    }

    Ok(match r#type {
        "020" => 'c', // character device
        "040" => 'd', // directory
        "060" => 'b', // block device
        "100" => '-', // regular file
        "120" => 'l', // symlink
        _ => '?',     // unknown
    })
}

pub fn parse_permissions(perm: Permissions) -> Result<String, String> {
    #[cfg(windows)]
    return Err(String::from("The -l flag does not work on Windows"));

    let bits = perm.mode();
    let bits_str = format!("{bits:06o}");

    let r#type = type_to_permission(&bits_str[0..3])?;
    let user = digit_to_permission(&bits_str[3..=3])?;
    let group = digit_to_permission(&bits_str[4..=4])?;
    let other = digit_to_permission(&bits_str[5..=5])?;

    Ok(format!("{type}{user}{group}{other}"))
}
