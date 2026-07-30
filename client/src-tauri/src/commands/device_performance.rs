use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
use sysinfo::{Components, System};

struct CachedPerformanceTier {
    tier: String,
    last_updated: Instant,
}

static CACHED_TIER: LazyLock<Arc<Mutex<CachedPerformanceTier>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(CachedPerformanceTier {
        tier: "medium".to_string(),
        last_updated: Instant::now() - Duration::from_secs(10),
    }))
});

/// Used to get an understanding of device power
///
/// The function gets various system information (see below) and then calculates
/// a score which returns "high", "medium" or "low".
/// This function has many uses but is ideal for tweaking settings or interal things to
/// operate on the user's device more effeciently.
///
/// This is dynamic in a way that, if a "high" rated device has low system resources available (such that a heavy operation could
/// cause the device to freeze or "lag"), this function will not return "high" even though the device is capable of it.
/// This is to ensure that the user has a good experience and does not have to deal with a "laggy" application due to quantity of resouces.
///
/// Below is what system information this function looks at
/// - CPU Core count
/// - CPU frequency in MHz
/// - Total RAM/memory
/// - Used RAM/memory
/// - System load (1-minute average)
///
/// # returns
/// - "high" if the device is capable of running heavy operations without lagging
/// - "medium" if the device is capable of running medium operations without lagging
/// - "low" if the device is not capable of running medium or heavy operations without lagging
///
/// # Example
/// ```rust
/// let performance_tier = get_performance_tier().await.unwrap();
/// assert!(performance_tier == "high" || performance_tier == "medium" || performance_tier == "low");
/// ```
#[tauri::command]
pub async fn get_performance_tier() -> Result<String, String> {
    //Keep a cached result for 1 minute so each call isn't constantly calculating all the time
    let mut cached = CACHED_TIER.lock().unwrap();
    if cached.last_updated.elapsed() < Duration::from_secs(60) {
        return Ok(cached.tier.clone());
    }

    let mut system = System::new_all();
    system.refresh_all();

    // CPU information
    let cpu_cores = system.cpus().len();
    let cpu_speed_mhz = system.cpus()[0].frequency();

    // Memory Information
    let total_memory_mb = system.total_memory() / (1024 * 1024);
    let used_memory_mb = system.used_memory() / (1024 * 1024);

    // Swap Information
    let total_swap_mb = system.total_swap() / (1024 * 1024);
    let used_swap_mb = system.used_swap() / (1024 * 1024);
    let swap_usage_percentage = if total_swap_mb > 0 {
        (used_swap_mb as f64 / total_swap_mb as f64) * 100.0
    } else {
        0.0
    };

    // System load (1-minute average)
    let load_avg = System::load_average().one;

    //Components temperatures
    let components = Components::new_with_refreshed_list();
    let max_cpu_temp = components
        .iter()
        .filter(|c| c.label().to_lowercase().contains("cpu"))
        .filter_map(|c| c.temperature())
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);

    // Calculate a performance score
    let temp_penalty = if max_cpu_temp > 80.0 {
        0.5
    }
    // High temperature
    else if max_cpu_temp > 60.0 {
        0.2
    }
    // Moderate temperature
    else {
        0.0
    }; // No penalty
    let cpu_score = if cpu_cores >= 8 {
        3.0 - temp_penalty
    } else if cpu_cores >= 4 {
        2.0 - temp_penalty
    } else {
        1.0 - temp_penalty
    };
    let cpu_speed_score = if cpu_speed_mhz >= 3000 {
        3.0
    } else if cpu_speed_mhz >= 2000 {
        2.0
    } else {
        1.0
    };
    let available_memory_mb = total_memory_mb - used_memory_mb;
    let swap_penalty = if swap_usage_percentage > 50.0 {
        0.5
    } else if swap_usage_percentage > 20.0 {
        0.2
    } else {
        0.0
    };
    let memory_score = if available_memory_mb >= 12_000 {
        3.0 - swap_penalty
    } else if available_memory_mb >= 6_000 {
        2.0 - swap_penalty
    } else {
        1.0 - swap_penalty
    };
    let load_score = if load_avg < 1.0 {
        3.0
    } else if load_avg < 2.0 {
        2.0
    } else {
        1.0
    };

    // Weighted score
    let performance_score =
        (cpu_score * 0.3) + (cpu_speed_score * 0.2) + (memory_score * 0.3) + (load_score * 0.2);

    // Determine tier
    let tier = if performance_score >= 2.5 {
        "high"
    } else if performance_score >= 1.5 {
        "medium"
    } else {
        "low"
    };

    cached.tier = tier.to_string();
    cached.last_updated = Instant::now();
    Ok(tier.to_string())
}
