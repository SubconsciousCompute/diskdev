use std::{env, path::PathBuf};
use sysinfo::Disks;

#[derive(Debug, Default)]
pub struct DiskDevice {
    list: Vec<PathBuf>,
}

pub fn get_disk_devices() -> DiskDevice {
    let mut disks: DiskDevice = DiskDevice::default();
    disks.list = match env::consts::OS {
        "linux" => get_disk_devices_linux(),
        _ => vec![],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_test() {
        let result = get_disk_devices();
        assert!(result.list.len() > 0);
    }
}
