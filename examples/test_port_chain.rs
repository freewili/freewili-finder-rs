/**
 * Example demonstrating USB port chain analysis
 *
 * This example shows how to analyze USB port chains and locations:
 * - Location: represents the actual port number on the immediate parent hub/controller
 * - PortChain: represents the full path from root hub to the device
 *
 * This helps understand the physical USB topology and device positioning.
 */
use freewili_finder_rs::{FreeWiliDevice, FreeWiliError};

fn main() -> Result<(), FreeWiliError> {
    println!("Testing Port Chain Analysis");
    println!("============================\n");

    let devices = FreeWiliDevice::find_all()?;

    println!("Found {} FreeWili device(s):\n", devices.len());

    for (i, device) in devices.iter().enumerate() {
        let name = device.name()?;
        let serial = device.serial()?;
        let usb_devices = device.get_usb_devices()?;

        println!("Device {}: {}", i + 1, name);
        println!("  Serial: {}", serial);
        println!("  USB Devices with Port Chain Analysis:");

        for usb_device in &usb_devices {
            println!("    - {}", usb_device.name);
            println!("      Location (port): {}", usb_device.location);

            // Format port chain nicely
            let port_chain_str = usb_device
                .port_chain
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            println!("      Port Chain: [{}]", port_chain_str);

            println!("      Type: {}", usb_device.kind_name);
            println!(
                "      VID:PID: 0x{:04X}:0x{:04X}",
                usb_device.vid, usb_device.pid
            );

            // Show optional path and port information if available
            if let Some(path) = &usb_device.path {
                println!("      System Path: {}", path);
            }
            if let Some(port) = &usb_device.port {
                println!("      Port String: {}", port);
            }

            // Analyze port chain depth and structure
            analyze_port_chain(&usb_device.port_chain, usb_device.location);

            println!();
        }

        // Summary analysis for this device
        analyze_device_topology(&usb_devices);
        println!();
    }

    Ok(())
}

/// Analyze the port chain structure and provide insights
fn analyze_port_chain(port_chain: &[u32], location: u32) {
    let depth = port_chain.len();

    if depth == 0 {
        println!("      Analysis: Direct connection (no hub chain)");
        return;
    }

    println!("      Analysis: {} level(s) deep", depth);

    if depth == 1 {
        println!("        └─ Connected directly to root hub/controller");
    } else {
        println!("        └─ Hub chain depth: {}", depth);
        for (i, &port) in port_chain.iter().enumerate() {
            let indent = "  ".repeat(i + 2);
            if i == 0 {
                println!("        {}└─ Root hub port {}", indent, port);
            } else if i == port_chain.len() - 1 {
                println!(
                    "        {}└─ Final hub port {} (location: {})",
                    indent, port, location
                );
            } else {
                println!("        {}└─ Intermediate hub port {}", indent, port);
            }
        }
    }

    // Check for potential USB topology insights
    if depth > 3 {
        println!("        ⚠️  Deep hub chain detected - may impact performance");
    }

    if location != port_chain.last().copied().unwrap_or(0) && !port_chain.is_empty() {
        println!(
            "        ℹ️  Location ({}) differs from final port chain entry ({})",
            location,
            port_chain.last().unwrap()
        );
    }
}

/// Analyze the overall USB topology for a device
fn analyze_device_topology(usb_devices: &[freewili_finder_rs::USBDevice]) {
    if usb_devices.is_empty() {
        return;
    }

    println!("  Topology Summary:");

    // Find the maximum depth
    let max_depth = usb_devices
        .iter()
        .map(|dev| dev.port_chain.len())
        .max()
        .unwrap_or(0);

    // Count devices at each depth level
    let mut depth_counts = vec![0; max_depth + 1];
    for device in usb_devices {
        let depth = device.port_chain.len();
        if depth < depth_counts.len() {
            depth_counts[depth] += 1;
        }
    }

    println!("    Maximum hub depth: {}", max_depth);

    for (depth, count) in depth_counts.iter().enumerate() {
        if *count > 0 {
            let level_desc = match depth {
                0 => "Direct connections".to_string(),
                1 => "1 hub level".to_string(),
                n => format!("{} hub levels", n),
            };
            println!("    {}: {} device(s)", level_desc, count);
        }
    }

    // Look for potential hub devices
    let potential_hubs: Vec<_> = usb_devices
        .iter()
        .filter(|dev| dev.kind_name.to_lowercase().contains("hub"))
        .collect();

    if !potential_hubs.is_empty() {
        println!("    Hub devices detected: {}", potential_hubs.len());
        for hub in potential_hubs {
            println!(
                "      - {} at chain depth {}",
                hub.name,
                hub.port_chain.len()
            );
        }
    }

    // Check for devices sharing the same port chain (shouldn't happen but useful to detect)
    let mut port_chain_map: std::collections::HashMap<Vec<u32>, Vec<&str>> =
        std::collections::HashMap::new();
    for device in usb_devices {
        port_chain_map
            .entry(device.port_chain.clone())
            .or_default()
            .push(&device.name);
    }

    for (chain, devices) in port_chain_map {
        if devices.len() > 1 {
            println!(
                "    ⚠️  Multiple devices share port chain {:?}: {}",
                chain,
                devices.join(", ")
            );
        }
    }
}
