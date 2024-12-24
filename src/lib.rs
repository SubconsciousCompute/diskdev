use std::process::{Command, Stdio};
use std::{env, path::PathBuf, str::FromStr};
use sysinfo::Disks;

#[derive(Debug, Default)]
pub struct DiskDevice {
    pub list: Vec<PathBuf>,
}

pub fn get_disk_devices() -> DiskDevice {
    let mut disks: DiskDevice = DiskDevice::default();
    disks.list = match env::consts::OS {
        "linux" => get_disk_devices_linux(),
        "macos" => get_disk_devices_mac(),
        _ => panic!("Unknown OS"),
    };

    return disks;
}

fn get_disk_devices_linux() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let name = disk.name().to_str().unwrap();
        out.push(PathBuf::from(name));
    }
    return out;
}

fn get_disk_devices_mac() -> Vec<PathBuf> {
    let diskutil_output = Command::new("diskutil")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("diskutil failed to execute");

    let grep_output = Command::new("grep")
        .arg("/dev/disk")
        .stdin(diskutil_output.stdout.unwrap())
        .output()
        .expect("grep failed to execute");

    let out = String::from_utf8_lossy(&grep_output.stdout);
    let lines: Vec<&str> = out.as_ref().split('\n').collect();
    let disks: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let mut paths = Vec::new();
    for disk in disks {
        let path = disk.get(0);
        if !path.is_some() {
            continue;
        }
        paths.push(PathBuf::from_str(path.unwrap()).unwrap());
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn unix_test() {
        let result = get_disk_devices();
        println!("{:?}", result.list);
        assert!(result.list.len() > 0);
    }
}
