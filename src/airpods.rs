use serde::Deserialize;
use tokio::process::Command;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct BlueutilDevice {
    name: Option<String>,
    address: Option<String>,
    connected: Option<bool>,
}

pub struct AirPodsInfo {
    pub device_name: String,
    pub left_battery: Option<u8>,
    pub right_battery: Option<u8>,
}

pub async fn get_airpods_info() -> anyhow::Result<AirPodsInfo> {
    // Run system_profiler and parse output
    let output = Command::new("system_profiler")
        .arg("SPBluetoothDataType")
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!("Failed to run system_profiler");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse AirPods block (rudimentary)
    let lines: Vec<&str> = stdout.lines().collect();
    let mut parsing = false;
    let mut data = HashMap::new();

    for line in lines.iter() {
        if line.contains("AirPods") {
            parsing = true;
        }
        if parsing {
            if line.trim().is_empty() {
                break;
            }
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim();
                let value = line[idx + 1..].trim();
                data.insert(key.to_string(), value.to_string());
            }
        }
    }

    let device_name = data
        .get("Device Name")
        .cloned()
        .unwrap_or_else(|| "AirPods".to_string());

    let left_battery = data
        .get("Left Battery Level")
        .and_then(|v| v.trim_end_matches('%').parse::<u8>().ok());

    let right_battery = data
        .get("Right Battery Level")
        .and_then(|v| v.trim_end_matches('%').parse::<u8>().ok());

    Ok(AirPodsInfo {
        device_name,
        left_battery,
        right_battery,
    })
}

pub async fn disconnect_airpods() -> anyhow::Result<String> {
    // Get connected devices from blueutil in JSON format
    let output = Command::new("blueutil")
        .arg("--connected")
        .arg("--format")
        .arg("json")
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!("Failed to list connected devices");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let devices: Vec<BlueutilDevice> = serde_json::from_str(&stdout)?;

    // Find AirPods device
    let airpods = devices
        .iter()
        .find(|d| {
            d.name
                .as_ref()
                .map_or(false, |n| n.to_lowercase().contains("airpods"))
                && d.address.is_some()
        })
        .ok_or_else(|| anyhow::anyhow!("No connected AirPods found"))?;

    // Disconnect AirPods
    let address = airpods.address.as_ref().unwrap();

    let status = Command::new("blueutil")
        .arg("--disconnect")
        .arg(address)
        .status()
        .await?;

    if status.success() {
        Ok(format!("✅ Disconnected {}", airpods.name.as_ref().unwrap()))
    } else {
        anyhow::bail!("Failed to disconnect AirPods");
    }
}
