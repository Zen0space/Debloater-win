use std::fs;
use std::path::PathBuf;
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatwareApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub safe: bool,
    pub package_pattern: String,
    pub is_installed: bool,
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

fn get_bloatware_definitions() -> Vec<BloatwareApp> {
    vec![
        BloatwareApp { id: "clipchamp", name: "Clipchamp", description: "Video editor from Microsoft", safe: true, package_pattern: "Clipchamp.Clipchamp".to_string(), is_installed: false },
        BloatwareApp { id: "cortana", name: "Cortana", description: "Microsoft Cortana voice assistant (Discontinued)", safe: true, package_pattern: "Microsoft.549981C3F5F10".to_string(), is_installed: false },
        BloatwareApp { id: "copilot", name: "Microsoft Copilot", description: "AI assistant integrated into Windows", safe: true, package_pattern: "Microsoft.Copilot".to_string(), is_installed: false },
        BloatwareApp { id: "bing-weather", name: "Weather App", description: "Weather forecast via Bing", safe: true, package_pattern: "Microsoft.BingWeather".to_string(), is_installed: false },
        BloatwareApp { id: "bing-news", name: "News App", description: "News aggregator via Bing", safe: true, package_pattern: "Microsoft.BingNews".to_string(), is_installed: false },
        BloatwareApp { id: "bing-sports", name: "Bing Sports", description: "Sports news and scores via Bing (Discontinued)", safe: true, package_pattern: "Microsoft.BingSports".to_string(), is_installed: false },
        BloatwareApp { id: "bing-finance", name: "Bing Finance", description: "Finance news and tracking via Bing (Discontinued)", safe: true, package_pattern: "Microsoft.BingFinance".to_string(), is_installed: false },
        BloatwareApp { id: "bing-search", name: "Bing Search", description: "Web Search from Microsoft Bing", safe: true, package_pattern: "Microsoft.BingSearch".to_string(), is_installed: false },
        BloatwareApp { id: "3d-builder", name: "3D Builder", description: "Basic 3D modeling software", safe: true, package_pattern: "Microsoft.3DBuilder".to_string(), is_installed: false },
        BloatwareApp { id: "3d-viewer", name: "3D Viewer", description: "Viewer for 3D models", safe: true, package_pattern: "Microsoft.Microsoft3DViewer".to_string(), is_installed: false },
        BloatwareApp { id: "print-3d", name: "Print 3D", description: "3D printing preparation software", safe: true, package_pattern: "Microsoft.Print3D".to_string(), is_installed: false },
        BloatwareApp { id: "paint-3d", name: "Paint 3D", description: "Modern paint application with 3D features", safe: true, package_pattern: "Microsoft.MSPaint".to_string(), is_installed: false },
        BloatwareApp { id: "office-hub", name: "Office Hub", description: "Hub to access Microsoft Office apps and documents", safe: true, package_pattern: "Microsoft.MicrosoftOfficeHub".to_string(), is_installed: false },
        BloatwareApp { id: "office-sway", name: "Sway", description: "Presentation and storytelling app", safe: true, package_pattern: "Microsoft.Office.Sway".to_string(), is_installed: false },
        BloatwareApp { id: "onenote", name: "OneNote (UWP)", description: "Digital note-taking app (UWP version)", safe: true, package_pattern: "Microsoft.Office.OneNote".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-solitaire", name: "Solitaire Collection", description: "Collection of solitaire card games", safe: true, package_pattern: "Microsoft.MicrosoftSolitaireCollection".to_string(), is_installed: false },
        BloatwareApp { id: "sticky-notes", name: "Sticky Notes", description: "Digital sticky notes app (Deprecated)", safe: true, package_pattern: "Microsoft.MicrosoftStickyNotes".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-todo", name: "Microsoft To Do", description: "To-do list and task management app", safe: true, package_pattern: "Microsoft.Todos".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-journal", name: "Microsoft Journal", description: "Digital note-taking app optimized for pen input", safe: true, package_pattern: "Microsoft.MicrosoftJournal".to_string(), is_installed: false },
        BloatwareApp { id: "power-automate", name: "Power Automate", description: "Desktop automation tool (RPA)", safe: true, package_pattern: "Microsoft.PowerAutomateDesktop".to_string(), is_installed: false },
        BloatwareApp { id: "power-bi", name: "Power BI", description: "Business analytics service client", safe: true, package_pattern: "Microsoft.MicrosoftPowerBIForWindows".to_string(), is_installed: false },
        BloatwareApp { id: "dev-home", name: "Dev Home", description: "Developer dashboard and tool configuration (Discontinued)", safe: true, package_pattern: "Microsoft.Windows.DevHome".to_string(), is_installed: false },
        BloatwareApp { id: "mixed-reality-portal", name: "Mixed Reality Portal", description: "Portal for Windows Mixed Reality headsets", safe: true, package_pattern: "Microsoft.MixedReality.Portal".to_string(), is_installed: false },
        BloatwareApp { id: "network-speed-test", name: "Network Speed Test", description: "Internet connection speed test utility", safe: true, package_pattern: "Microsoft.NetworkSpeedTest".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-news", name: "Microsoft News", description: "News aggregator (now part of Microsoft Start)", safe: true, package_pattern: "Microsoft.News".to_string(), is_installed: false },
        BloatwareApp { id: "messaging", name: "Messaging", description: "Messaging app, often integrates with Skype (Deprecated)", safe: true, package_pattern: "Microsoft.Messaging".to_string(), is_installed: false },
        BloatwareApp { id: "skype-app", name: "Skype (UWP)", description: "Skype communication app, UWP version (Discontinued)", safe: true, package_pattern: "Microsoft.SkypeApp".to_string(), is_installed: false },
        BloatwareApp { id: "oneconnect", name: "One Connect", description: "Mobile Operator management app (Replaced by Mobile Plans)", safe: true, package_pattern: "Microsoft.OneConnect".to_string(), is_installed: false },
        BloatwareApp { id: "windows-maps", name: "Windows Maps", description: "Mapping and navigation app", safe: true, package_pattern: "Microsoft.WindowsMaps".to_string(), is_installed: false },
        BloatwareApp { id: "windows-alarms", name: "Alarms & Clock", description: "Alarms & Clock app", safe: true, package_pattern: "Microsoft.WindowsAlarms".to_string(), is_installed: false },
        BloatwareApp { id: "sound-recorder", name: "Sound Recorder", description: "Basic audio recording app", safe: true, package_pattern: "Microsoft.WindowsSoundRecorder".to_string(), is_installed: false },
        BloatwareApp { id: "windows-feedback-hub", name: "Feedback Hub", description: "App for providing feedback to Microsoft on Windows", safe: true, package_pattern: "Microsoft.WindowsFeedbackHub".to_string(), is_installed: false },
        BloatwareApp { id: "get-help", name: "Get Help", description: "Required for some Windows 11 Troubleshooters and support", safe: true, package_pattern: "Microsoft.GetHelp".to_string(), is_installed: false },
        BloatwareApp { id: "get-started", name: "Get Started", description: "Tips and introductory guide for Windows", safe: true, package_pattern: "Microsoft.Getstarted".to_string(), is_installed: false },
        BloatwareApp { id: "zune-music", name: "Media Player", description: "Modern Media Player (Replaced Groove Music)", safe: true, package_pattern: "Microsoft.ZuneMusic".to_string(), is_installed: false },
        BloatwareApp { id: "zune-video", name: "Movies & TV", description: "Movies & TV app for video content", safe: true, package_pattern: "Microsoft.ZuneVideo".to_string(), is_installed: false },
        BloatwareApp { id: "people-app", name: "People", description: "Contacts management app", safe: true, package_pattern: "Microsoft.People".to_string(), is_installed: false },
        BloatwareApp { id: "mail-calendar", name: "Mail & Calendar", description: "Mail & Calendar app suite (Discontinued)", safe: true, package_pattern: "Microsoft.windowscommunicationsapps".to_string(), is_installed: false },
        BloatwareApp { id: "outlook-new", name: "Outlook for Windows", description: "New Outlook for Windows mail client", safe: true, package_pattern: "Microsoft.OutlookForWindows".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-app", name: "Xbox Console Companion", description: "Old Xbox Console Companion App (Discontinued)", safe: true, package_pattern: "Microsoft.XboxApp".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-tcui", name: "Xbox TCUI Framework", description: "UI framework required for Microsoft Store and certain games", safe: false, package_pattern: "Microsoft.Xbox.TCUI".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-game-overlay", name: "Xbox Game Overlay", description: "Game overlay, part of Xbox Game Bar", safe: false, package_pattern: "Microsoft.XboxGameOverlay".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-gaming-overlay", name: "Xbox Gaming Overlay", description: "Game overlay, required/useful for some games", safe: false, package_pattern: "Microsoft.XboxGamingOverlay".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-identity", name: "Xbox Identity Provider", description: "Xbox sign-in framework, required for some games", safe: false, package_pattern: "Microsoft.XboxIdentityProvider".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-speech", name: "Xbox Speech To Text", description: "Accessibility feature required for some games", safe: false, package_pattern: "Microsoft.XboxSpeechToTextOverlay".to_string(), is_installed: false },
        BloatwareApp { id: "xbox-gaming-app", name: "Xbox Gaming App", description: "Modern Xbox Gaming App, required for installing some PC games", safe: false, package_pattern: "Microsoft.GamingApp".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-teams-old", name: "Microsoft Teams (Old)", description: "Old Microsoft Teams personal (MS Store version)", safe: true, package_pattern: "MicrosoftTeams".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-teams-new", name: "Microsoft Teams (New)", description: "New Microsoft Teams app (Work/School or Personal)", safe: true, package_pattern: "MSTeams".to_string(), is_installed: false },
        BloatwareApp { id: "family-safety", name: "Family Safety", description: "Family Safety App for managing family accounts", safe: true, package_pattern: "MicrosoftCorporationII.MicrosoftFamily".to_string(), is_installed: false },
        BloatwareApp { id: "quick-assist", name: "Quick Assist", description: "Remote assistance tool", safe: true, package_pattern: "MicrosoftCorporationII.QuickAssist".to_string(), is_installed: false },
        BloatwareApp { id: "cross-device", name: "Cross Device Experience", description: "Phone integration within File Explorer and more", safe: true, package_pattern: "MicrosoftWindows.CrossDevice".to_string(), is_installed: false },
        BloatwareApp { id: "phone-link", name: "Phone Link", description: "Phone link (Connects Android/iOS phone to PC)", safe: true, package_pattern: "Microsoft.YourPhone".to_string(), is_installed: false },
        BloatwareApp { id: "whiteboard", name: "Whiteboard", description: "Digital collaborative whiteboard app", safe: true, package_pattern: "Microsoft.Whiteboard".to_string(), is_installed: false },
        BloatwareApp { id: "widgets-experience", name: "Widgets Experience", description: "This app powers Windows Widgets My Feed", safe: true, package_pattern: "Microsoft.StartExperiencesApp".to_string(), is_installed: false },
        BloatwareApp { id: "m365-companions", name: "Microsoft 365 Companions", description: "Microsoft 365 Calendar, Files and People mini-apps", safe: true, package_pattern: "Microsoft.M365Companions".to_string(), is_installed: false },
        BloatwareApp { id: "remote-desktop", name: "Remote Desktop", description: "Remote Desktop client app", safe: true, package_pattern: "Microsoft.RemoteDesktop".to_string(), is_installed: false },
        BloatwareApp { id: "photos", name: "Photos", description: "Default photo viewing and basic editing app", safe: true, package_pattern: "Microsoft.Windows.Photos".to_string(), is_installed: false },
        BloatwareApp { id: "paint", name: "Paint", description: "Classic Paint (Traditional 2D paint application)", safe: true, package_pattern: "Microsoft.Paint".to_string(), is_installed: false },
        BloatwareApp { id: "notepad", name: "Notepad", description: "Notepad text editor app", safe: true, package_pattern: "Microsoft.WindowsNotepad".to_string(), is_installed: false },
        BloatwareApp { id: "calculator", name: "Calculator", description: "Calculator app", safe: true, package_pattern: "Microsoft.WindowsCalculator".to_string(), is_installed: false },
        BloatwareApp { id: "camera", name: "Camera", description: "Camera app for using built-in or connected cameras", safe: false, package_pattern: "Microsoft.WindowsCamera".to_string(), is_installed: false },
        BloatwareApp { id: "snipping-tool", name: "Snipping Tool", description: "Screenshot and annotation tool", safe: true, package_pattern: "Microsoft.ScreenSketch".to_string(), is_installed: false },
        BloatwareApp { id: "terminal", name: "Windows Terminal", description: "Default terminal app in Windows 11", safe: true, package_pattern: "Microsoft.WindowsTerminal".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-store", name: "Microsoft Store", description: "Microsoft Store - WARNING: Cannot be reinstalled easily!", safe: false, package_pattern: "Microsoft.WindowsStore".to_string(), is_installed: false },
        BloatwareApp { id: "microsoft-edge", name: "Microsoft Edge", description: "Edge browser (Can only be uninstalled in EEA)", safe: false, package_pattern: "Microsoft.Edge".to_string(), is_installed: false },
        BloatwareApp { id: "onedrive", name: "OneDrive", description: "OneDrive consumer cloud storage client", safe: false, package_pattern: "Microsoft.OneDrive".to_string(), is_installed: false },
        BloatwareApp { id: "spotify", name: "Spotify", description: "Spotify music streaming app", safe: true, package_pattern: "SpotifyAB.SpotifyMusic".to_string(), is_installed: false },
        BloatwareApp { id: "netflix", name: "Netflix", description: "Netflix streaming service app", safe: true, package_pattern: "4DF9E0F8.Netflix".to_string(), is_installed: false },
        BloatwareApp { id: "prime-video", name: "Prime Video", description: "Amazon Prime Video streaming service app", safe: true, package_pattern: "AmazonVideo.PrimeVideo".to_string(), is_installed: false },
        BloatwareApp { id: "hulu", name: "Hulu", description: "Hulu streaming service app", safe: true, package_pattern: "HULULLC.HULUPLUS".to_string(), is_installed: false },
        BloatwareApp { id: "tiktok", name: "TikTok", description: "TikTok short-form video app", safe: true, package_pattern: "BytedancePte.Ltd.TikTok".to_string(), is_installed: false },
        BloatwareApp { id: "instagram", name: "Instagram", description: "Instagram social media app", safe: true, package_pattern: "Facebook.Instagram".to_string(), is_installed: false },
        BloatwareApp { id: "facebook", name: "Facebook", description: "Facebook social media app", safe: true, package_pattern: "Facebook.Facebook".to_string(), is_installed: false },
        BloatwareApp { id: "twitter", name: "Twitter (X)", description: "Twitter (now X) social media app", safe: true, package_pattern: "9E2F88E3.Twitter".to_string(), is_installed: false },
        BloatwareApp { id: "linkedin", name: "LinkedIn", description: "LinkedIn professional networking app", safe: true, package_pattern: "LinkedInforWindows".to_string(), is_installed: false },
        BloatwareApp { id: "flipboard", name: "Flipboard", description: "News and social network aggregator styled as a magazine", safe: true, package_pattern: "Flipboard.Flipboard".to_string(), is_installed: false },
        BloatwareApp { id: "candy-crush-saga", name: "Candy Crush Saga", description: "Puzzle game from King", safe: true, package_pattern: "king.com.CandyCrushSaga".to_string(), is_installed: false },
        BloatwareApp { id: "candy-crush-soda", name: "Candy Crush Soda", description: "Puzzle game from King", safe: true, package_pattern: "king.com.CandyCrushSodaSaga".to_string(), is_installed: false },
        BloatwareApp { id: "bubble-witch-3", name: "Bubble Witch 3", description: "Puzzle game from King", safe: true, package_pattern: "king.com.BubbleWitch3Saga".to_string(), is_installed: false },
        BloatwareApp { id: "asphalt-8", name: "Asphalt 8", description: "Racing game", safe: true, package_pattern: "GAMELOFTSA.Asphalt8Airborne".to_string(), is_installed: false },
        BloatwareApp { id: "cooking-fever", name: "Cooking Fever", description: "Restaurant simulation game", safe: true, package_pattern: "Nordcurrent.CookingFever".to_string(), is_installed: false },
        BloatwareApp { id: "disney-magic-kingdoms", name: "Disney Magic Kingdoms", description: "Disney theme park building game", safe: true, package_pattern: "A278AB0D.DisneyMagicKingdoms".to_string(), is_installed: false },
        BloatwareApp { id: "march-of-empires", name: "March of Empires", description: "Strategy game", safe: true, package_pattern: "Glu.MarchofEmpires".to_string(), is_installed: false },
        BloatwareApp { id: "royal-revolt", name: "Royal Revolt", description: "Tower defense / strategy game", safe: true, package_pattern: "Microsoft.RoyalRevolt2".to_string(), is_installed: false },
        BloatwareApp { id: "duolingo", name: "Duolingo", description: "Language learning app", safe: true, package_pattern: "DuoLLC.Duolingo-LearnLanguagesforFree".to_string(), is_installed: false },
        BloatwareApp { id: "photoshop-express", name: "Adobe Photoshop Express", description: "Basic photo editing app from Adobe", safe: true, package_pattern: "AdobeSystemsIncorporated.AdobePhotoshopExpress".to_string(), is_installed: false },
        BloatwareApp { id: "picsart", name: "PicsArt", description: "Photo editing and creative app", safe: true, package_pattern: "PicsArt.PicsArt-PhotoStudio".to_string(), is_installed: false },
        BloatwareApp { id: "polarr", name: "Polarr Photo Editor", description: "Photo editing app (Academic Edition)", safe: true, package_pattern: "AcquiredOnline.PolarrPhotoEditorAcademicEdition".to_string(), is_installed: false },
        BloatwareApp { id: "drawboard-pdf", name: "Drawboard PDF", description: "PDF viewing and annotation app", safe: true, package_pattern: "Drawboard.DrawboardPDF".to_string(), is_installed: false },
        BloatwareApp { id: "cyberlink-media-suite", name: "CyberLink Media Suite", description: "Multimedia software suite (often preinstalled by OEMs)", safe: true, package_pattern: "CyberLinkCorp.ac.CyberLinkMediaSuiteEssentials".to_string(), is_installed: false },
        BloatwareApp { id: "plex", name: "Plex", description: "Media server and player app", safe: true, package_pattern: "PlexInc.Plex".to_string(), is_installed: false },
        BloatwareApp { id: "iheartradio", name: "iHeartRadio", description: "Internet radio streaming app", safe: true, package_pattern: "ClearChannel.iHeartRadio".to_string(), is_installed: false },
        BloatwareApp { id: "tunein-radio", name: "TuneIn Radio", description: "Internet radio streaming app", safe: true, package_pattern: "TuneIn.TuneInRadio".to_string(), is_installed: false },
        BloatwareApp { id: "pandora", name: "Pandora", description: "Pandora music streaming app", safe: true, package_pattern: "PandoraMediaInc.29680B314EFC2".to_string(), is_installed: false },
        BloatwareApp { id: "shazam", name: "Shazam", description: "Music identification app", safe: true, package_pattern: "ShazamEntertainmentLtd.Shazam".to_string(), is_installed: false },
        BloatwareApp { id: "amazon", name: "Amazon", description: "Amazon shopping app", safe: true, package_pattern: "Amazon.com.Amazon".to_string(), is_installed: false },
        BloatwareApp { id: "wallet", name: "Wallet", description: "Wallet app", safe: true, package_pattern: "Microsoft.Wallet".to_string(), is_installed: false },
        BloatwareApp { id: "winzip", name: "WinZip", description: "File compression utility (UWP version)", safe: true, package_pattern: "WinZipComputing.WinZipUniversal".to_string(), is_installed: false },
        BloatwareApp { id: "fitbit", name: "Fitbit", description: "Fitbit activity tracker companion app", safe: true, package_pattern: "Fitbit.FitbitCoach".to_string(), is_installed: false },
        BloatwareApp { id: "viber", name: "Viber", description: "Messaging and calling app", safe: true, package_pattern: "2414FC7A.Viber".to_string(), is_installed: false },
        BloatwareApp { id: "sling-tv", name: "Sling TV", description: "Live TV streaming service app", safe: true, package_pattern: "SlingTVLLC.SlingTV".to_string(), is_installed: false },
        BloatwareApp { id: "acg-media-player", name: "ACG Media Player", description: "Media player app", safe: true, package_pattern: "Acgeditor.ACGMediaPlayer".to_string(), is_installed: false },
        BloatwareApp { id: "one-calendar", name: "One Calendar", description: "Calendar aggregation app", safe: true, package_pattern: "johnlangen.OneCalendar".to_string(), is_installed: false },
        BloatwareApp { id: "phototastic", name: "Phototastic Collage", description: "Photo collage creation app", safe: true, package_pattern: "Xerysoft.PhototasticCollage".to_string(), is_installed: false },
        BloatwareApp { id: "actipro", name: "Actipro Software", description: "UI controls or software components, often bundled by OEMs", safe: true, package_pattern: "ActiproSoftwareLLC".to_string(), is_installed: false },
        BloatwareApp { id: "autodesk-sketchbook", name: "Autodesk SketchBook", description: "Digital drawing and sketching app", safe: true, package_pattern: "AutodeskInc.AutodeskSketchBook".to_string(), is_installed: false },
        BloatwareApp { id: "caesars-slots", name: "Caesars Slots", description: "Casino slot machine game", safe: true, package_pattern: "Playtika.CaesarsSlotsFreeCasino".to_string(), is_installed: false },
        BloatwareApp { id: "farmville-2", name: "FarmVille 2", description: "Farming simulation game", safe: true, package_pattern: "Zynga.FarmVille2CountryEscape".to_string(), is_installed: false },
        BloatwareApp { id: "hidden-city", name: "Hidden City", description: "Hidden object puzzle adventure game", safe: true, package_pattern: "G5E.HiddenCityMysteryofShadows".to_string(), is_installed: false },
        BloatwareApp { id: "nyt-crossword", name: "NYT Crossword", description: "New York Times crossword puzzle app", safe: true, package_pattern: "NYTimes.DailyCrossword".to_string(), is_installed: false },
        BloatwareApp { id: "eclipse-manager", name: "Eclipse Manager", description: "OEM software or utilities", safe: true, package_pattern: "EclipseManager".to_string(), is_installed: false },
        BloatwareApp { id: "live-wallpaper", name: "Live Wallpaper", description: "Live wallpaper app", safe: true, package_pattern: "Sidia.LiveWallpaper".to_string(), is_installed: false },
        BloatwareApp { id: "wunderlist", name: "Wunderlist", description: "To-do list app (Acquired by Microsoft, moved to To Do)", safe: true, package_pattern: "6Wunderkinder.Wunderlist".to_string(), is_installed: false },
        BloatwareApp { id: "xing", name: "XING", description: "Professional networking platform", safe: true, package_pattern: "XINGAG.XING".to_string(), is_installed: false },
        BloatwareApp { id: "disney", name: "Disney", description: "General Disney content app", safe: true, package_pattern: "Disney.37853FC22B2CE".to_string(), is_installed: false },
        BloatwareApp { id: "hp-ai-center", name: "HP AI Experience Center", description: "HP OEM software, AI-enhanced features and support", safe: true, package_pattern: "AD2F1837.HPAIExperienceCenter".to_string(), is_installed: false },
        BloatwareApp { id: "hp-connected-music", name: "HP Connected Music", description: "HP OEM software for music", safe: true, package_pattern: "AD2F1837.HPConnectedMusic".to_string(), is_installed: false },
        BloatwareApp { id: "hp-connected-photo", name: "HP Connected Photo", description: "HP OEM software for photos", safe: true, package_pattern: "AD2F1837.HPConnectedPhotopoweredbySnapfish".to_string(), is_installed: false },
        BloatwareApp { id: "hp-desktop-support", name: "HP Desktop Support Utilities", description: "HP OEM software providing desktop support tools", safe: true, package_pattern: "AD2F1837.HPDesktopSupportUtilities".to_string(), is_installed: false },
        BloatwareApp { id: "hp-easy-clean", name: "HP Easy Clean", description: "HP OEM software for system cleaning", safe: true, package_pattern: "AD2F1837.HPEasyClean".to_string(), is_installed: false },
        BloatwareApp { id: "hp-file-viewer", name: "HP File Viewer", description: "HP OEM software for viewing files", safe: true, package_pattern: "AD2F1837.HPFileViewer".to_string(), is_installed: false },
        BloatwareApp { id: "hp-jumpstart", name: "HP JumpStarts", description: "HP OEM software for tutorials and quick access", safe: true, package_pattern: "AD2F1837.HPJumpStarts".to_string(), is_installed: false },
        BloatwareApp { id: "hp-pc-diagnostics", name: "HP PC Hardware Diagnostics", description: "HP OEM software for PC hardware diagnostics", safe: true, package_pattern: "AD2F1837.HPPCHardwareDiagnosticsWindows".to_string(), is_installed: false },
        BloatwareApp { id: "hp-power-manager", name: "HP Power Manager", description: "HP OEM software for managing power settings", safe: true, package_pattern: "AD2F1837.HPPowerManager".to_string(), is_installed: false },
        BloatwareApp { id: "hp-printer-control", name: "HP Printer Control", description: "HP OEM software for managing HP printers", safe: true, package_pattern: "AD2F1837.HPPrinterControl".to_string(), is_installed: false },
        BloatwareApp { id: "hp-privacy-settings", name: "HP Privacy Settings", description: "HP OEM software for managing privacy settings", safe: true, package_pattern: "AD2F1837.HPPrivacySettings".to_string(), is_installed: false },
        BloatwareApp { id: "hp-quickdrop", name: "HP QuickDrop", description: "HP OEM software for quick file transfer", safe: true, package_pattern: "AD2F1837.HPQuickDrop".to_string(), is_installed: false },
        BloatwareApp { id: "hp-quicktouch", name: "HP QuickTouch", description: "HP OEM software for touch-specific shortcuts", safe: true, package_pattern: "AD2F1837.HPQuickTouch".to_string(), is_installed: false },
        BloatwareApp { id: "hp-registration", name: "HP Registration", description: "HP OEM software for product registration", safe: true, package_pattern: "AD2F1837.HPRegistration".to_string(), is_installed: false },
        BloatwareApp { id: "hp-support-assistant", name: "HP Support Assistant", description: "HP OEM software for support and troubleshooting", safe: true, package_pattern: "AD2F1837.HPSupportAssistant".to_string(), is_installed: false },
        BloatwareApp { id: "hp-sure-shield", name: "HP Sure Shield AI", description: "HP OEM security software, AI-based threat protection", safe: true, package_pattern: "AD2F1837.HPSureShieldAI".to_string(), is_installed: false },
        BloatwareApp { id: "hp-system-info", name: "HP System Information", description: "HP OEM software for displaying system information", safe: true, package_pattern: "AD2F1837.HPSystemInformation".to_string(), is_installed: false },
        BloatwareApp { id: "hp-welcome", name: "HP Welcome", description: "HP OEM software providing welcome experience", safe: true, package_pattern: "AD2F1837.HPWelcome".to_string(), is_installed: false },
        BloatwareApp { id: "hp-workwell", name: "HP WorkWell", description: "HP OEM software focused on well-being", safe: true, package_pattern: "AD2F1837.HPWorkWell".to_string(), is_installed: false },
        BloatwareApp { id: "myhp", name: "myHP", description: "HP OEM central hub app for device info and services", safe: true, package_pattern: "AD2F1837.myHP".to_string(), is_installed: false },
    ]
}

fn get_resource_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_path = app.path().resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    Ok(resource_path.join("data"))
}

#[tauri::command]
fn load_items(category: String, app: tauri::AppHandle) -> Result<Vec<DebloatItem>, String> {
    let data_path = get_resource_path(app)?;
    let file_path = data_path.join(format!("{}.json", category));
    
    if !file_path.exists() {
        return Err(format!("File not found: {}", file_path.to_string_lossy()));
    }
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let items: Vec<DebloatItem> = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(items)
}

#[tauri::command]
fn load_presets(app: tauri::AppHandle) -> Result<Vec<Preset>, String> {
    let data_path = get_resource_path(app)?;
    let file_path = data_path.join("presets.json");
    
    if !file_path.exists() {
        return Err(format!("Presets file not found: {}", file_path.to_string_lossy()));
    }
    
    let contents = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read presets file: {}", e))?;
    
    let data: PresetsData = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse presets JSON: {}", e))?;
    
    Ok(data.presets)
}

#[tauri::command]
async fn get_installed_package_names() -> Result<Vec<String>, String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let ps_script = "Get-AppxPackage | Select-Object -ExpandProperty Name";
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-Command", ps_script])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let packages: Vec<String> = stdout
                    .lines()
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect();
                Ok(packages)
            }
            Err(e) => Err(format!("Failed to get installed packages: {}", e))
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

#[tauri::command]
async fn get_all_bloatware_with_status() -> Result<Vec<BloatwareApp>, String> {
    let mut bloatware = get_bloatware_definitions();
    
    #[cfg(target_os = "windows")]
    {
        let installed_packages = get_installed_package_names().await?;
        
        for app in &mut bloatware {
            let pattern_lower = app.package_pattern.to_lowercase();
            app.is_installed = installed_packages.iter().any(|pkg| {
                pkg.contains(&pattern_lower) || pattern_lower.contains(pkg)
            });
        }
    }
    
    Ok(bloatware)
}

#[tauri::command]
async fn remove_app(package_pattern: String) -> CommandResult {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let command = format!(
            "Get-AppxPackage *{}* | Remove-AppxPackage -AllUsers; Get-AppxProvisionedPackage -Online | Where-Object {{ $_.PackageName -like '*{}*' }} | Remove-AppxProvisionedPackage -Online -AllUsers",
            package_pattern, package_pattern
        );
        
        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-WindowStyle", "Hidden", "-Command", &command])
            .creation_flags(0x08000000)
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() || stdout.contains("not found") {
                    CommandResult {
                        success: true,
                        output: if stdout.is_empty() { "App removed successfully".to_string() } else { stdout },
                        error: None,
                    }
                } else {
                    CommandResult {
                        success: false,
                        output: stdout,
                        error: Some(if stderr.is_empty() {
                            "Failed to remove app".to_string()
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
            load_items,
            load_presets,
            get_installed_package_names,
            get_all_bloatware_with_status,
            remove_app,
            execute_command,
            execute_commands,
            get_system_info,
            test_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
