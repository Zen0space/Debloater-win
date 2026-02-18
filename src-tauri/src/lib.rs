use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebloatItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub safe: bool,
    pub command: String,
    pub rollback_command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PresetsData {
    pub presets: Vec<Preset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

#[tauri::command]
fn get_data_dir() -> Result<String, String> {
    let mut path = if cfg!(debug_assertions) {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
    } else {
        let mut exe_path = std::env::current_exe()
            .map_err(|e| e.to_string())?;
        exe_path.pop();
        exe_path
    };
    
    path.push("data");
    
    if !path.exists() {
        return Err(format!("Data directory not found at: {}", path.to_string_lossy()));
    }
    
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn load_items(category: String) -> Result<Vec<DebloatItem>, String> {
    let data_dir = get_data_dir()?;
    let file_path = format!("{}/{}.json", data_dir, category);
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let items: Vec<DebloatItem> = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(items)
}

#[tauri::command]
fn load_presets() -> Result<Vec<Preset>, String> {
    let data_dir = get_data_dir()?;
    let file_path = format!("{}/presets.json", data_dir);
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read presets file: {}", e))?;
    
    let data: PresetsData = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse presets JSON: {}", e))?;
    
    Ok(data.presets)
}

#[tauri::command]
async fn execute_command(command: String, is_rollback: bool) -> CommandResult {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-Command", &command])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() {
                    CommandResult {
                        success: true,
                        output: stdout,
                        error: None,
                    }
                } else {
                    CommandResult {
                        success: false,
                        output: stdout,
                        error: Some(if stderr.is_empty() {
                            format!("Command failed with exit code: {:?}", output.status.code())
                        } else {
                            stderr
                        }),
                    }
                }
            }
            Err(e) => CommandResult {
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        CommandResult {
            success: false,
            output: String::new(),
            error: Some("This application only runs on Windows".to_string()),
        }
    }
}

#[tauri::command]
async fn execute_commands(commands: Vec<String>, is_rollback: bool) -> Vec<CommandResult> {
    let mut results = Vec::new();
    
    for cmd in commands {
        let result = execute_command(cmd, is_rollback).await;
        results.push(result);
    }
    
    results
}

#[tauri::command]
fn get_system_info() -> Result<serde_json::Value, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
        
        let os_version = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", "[System.Environment]::OSVersion.VersionString"])
            .creation_flags(0x08000000)
            .output();
        
        let os_version_str = match os_version {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        
        let build = Command::new("powershell")
            .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", "(Get-CimInstance Win32_OperatingSystem).BuildNumber"])
            .creation_flags(0x08000000)
            .output();
        
        let build_str = match build {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "Unknown".to_string(),
        };
        
        Ok(serde_json::json!({
            "windowsVersion": os_version_str,
            "buildNumber": build_str,
            "username": username
        }))
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Err("This application only runs on Windows".to_string())
    }
}

#[tauri::command]
fn test_command() -> String {
    "Backend is working!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_data_dir,
            load_items,
            load_presets,
            execute_command,
            execute_commands,
            get_system_info,
            test_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
