use std::fs;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebloatItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub safe: bool,
    pub command: String,
    pub rollback_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_installed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledApp {
    pub name: String,
    pub package_full_name: String,
    pub version: String,
    pub publisher: String,
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
pub struct AppDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub safe: bool,
    pub package_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppsConfig {
    pub apps: Vec<AppDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

#[tauri::command]
fn get_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let resource_path = app.path().resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    
    let data_path = resource_path.join("data");
    
    if !data_path.exists() {
        return Err(format!("Data directory not found at: {}", data_path.to_string_lossy()));
    }
    
    Ok(data_path.to_string_lossy().to_string())
}

#[tauri::command]
fn load_items(category: String, app: tauri::AppHandle) -> Result<Vec<DebloatItem>, String> {
    let data_dir = get_data_dir(app)?;
    let file_path = format!("{}/{}.json", data_dir, category);
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let items: Vec<DebloatItem> = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(items)
}

#[tauri::command]
fn load_presets(app: tauri::AppHandle) -> Result<Vec<Preset>, String> {
    let data_dir = get_data_dir(app)?;
    let file_path = format!("{}/presets.json", data_dir);
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read presets file: {}", e))?;
    
    let data: PresetsData = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse presets JSON: {}", e))?;
    
    Ok(data.presets)
}

#[tauri::command]
async fn get_installed_apps() -> Result<Vec<InstalledApp>, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let ps_script = r#"
            Get-AppxPackage -AllUsers | Select-Object Name, PackageFullName, Version, Publisher | ConvertTo-Json -Depth 3
        "#;
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-Command", ps_script])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                
                if stdout.trim().is_empty() {
                    return Ok(Vec::new());
                }
                
                let apps: Vec<InstalledApp> = if stdout.trim().starts_with('[') {
                    serde_json::from_str(&stdout)
                        .unwrap_or_else(|_| Vec::new())
                } else {
                    let single_app: InstalledApp = serde_json::from_str(&stdout)
                        .unwrap_or_else(|_| InstalledApp {
                            name: String::new(),
                            package_full_name: String::new(),
                            version: String::new(),
                            publisher: String::new(),
                        });
                    if single_app.name.is_empty() {
                        Vec::new()
                    } else {
                        vec![single_app]
                    }
                };
                
                Ok(apps)
            }
            Err(e) => Err(format!("Failed to get installed apps: {}", e))
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

#[tauri::command]
async fn load_apps_with_status(app: tauri::AppHandle) -> Result<Vec<DebloatItem>, String> {
    let data_dir = get_data_dir(app.clone())?;
    let file_path = format!("{}/apps.json", data_dir);
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read apps file: {}", e))?;
    
    let mut items: Vec<DebloatItem> = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse apps JSON: {}", e))?;
    
    #[cfg(target_os = "windows")]
    {
        let installed_apps = get_installed_apps().await?;
        
        for item in &mut items {
            let command = &item.command;
            let is_installed = installed_apps.iter().any(|app| {
                if command.contains('*') {
                    let pattern = command
                        .replace("Get-AppxPackage", "")
                        .replace("| Remove-AppxPackage", "")
                        .replace("*", "")
                        .trim()
                        .to_lowercase();
                    app.name.to_lowercase().contains(&pattern) || 
                    app.package_full_name.to_lowercase().contains(&pattern)
                } else {
                    false
                }
            });
            item.is_installed = Some(is_installed);
        }
    }
    
    Ok(items)
}

#[tauri::command]
async fn execute_command(command: String, _is_rollback: bool) -> CommandResult {
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
            Err(e) => {
                CommandResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!("{}", e)),
                }
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
            get_installed_apps,
            load_apps_with_status,
            execute_command,
            execute_commands,
            get_system_info,
            test_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
