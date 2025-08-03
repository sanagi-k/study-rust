use sysinfo::{Components, Disks, Networks, System};

fn main() {
    // System構造体のインスタンスを作成し、すべての情報を更新します。
    // `new_all()` はCPUとプロセスのリストも取得します。
    let mut sys = System::new_all();

    // 情報を最新の状態に更新する
    sys.refresh_all();

    // --- メモリ情報 ---
    println!("--- Memory Information ---");
    println!("Total memory: {} bytes", sys.total_memory());
    println!("Used memory : {} bytes", sys.used_memory());
    println!("Total swap  : {} bytes", sys.total_swap());
    println!("Used swap   : {} bytes", sys.used_swap());

    // --- CPU情報 ---
    println!("\n--- CPU Information ---");
    // グローバルなCPU使用率 (更新を複数回行い、差分から計算されるため、初回は0%の可能性があります)
    println!("Global CPU usage: {}%", sys.global_cpu_usage());

    // 各CPUコアの使用率
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("CPU {}: {}%", i, cpu.cpu_usage());
    }

    // `refresh_cpu()` を数秒待ってから再度呼び出すと、より正確なCPU使用率が取得できます。
    // 例:
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // sys.refresh_cpu();
    // println!("Global CPU usage (after 1s): {}%", sys.global_cpu_info().cpu_usage());

    // --- ディスク情報 ---
    println!("\n--- Disk Information ---");
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!("Disk name: {:?}", disk.name());
        println!("  Mount point: {:?}", disk.mount_point());
        println!("  Total space: {} bytes", disk.total_space());
        println!("  Available space: {} bytes", disk.available_space());
        println!("  File system: {:?}", disk.file_system());
        println!("  Type: {:?}", disk.kind());
    }

    // --- その他（参考情報） ---
    println!("\n--- System Information ---");
    println!("System name: {:?}", System::name());
    println!("Kernel version: {:?}", System::kernel_version());
    println!("OS version: {:?}", System::os_version());
    println!("Host name: {:?}", System::host_name());
    println!("Boot time: {}", System::boot_time());

    // ネットワーク情報
    let networks = Networks::new_with_refreshed_list();
    println!("\n--- Network Information ---");
    for (interface_name, data) in networks.list() {
        println!("Interface: {}", interface_name);
        println!("  Received: {} bytes", data.received());
        println!("  Transmitted: {} bytes", data.transmitted());
    }

    // コンポーネント情報（温度など）
    let components = Components::new_with_refreshed_list();
    println!("\n--- Component Information ---");
    for component in components.list() {
        println!("Component: {:?}", component.label());
        println!("  Temperature: {:?}°C", component.temperature());
    }
}
