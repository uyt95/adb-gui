//! User-facing strings in English and Dutch, selected from the locale.

use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    En,
    Nl,
}

fn lang() -> Lang {
    static LANG: OnceLock<Lang> = OnceLock::new();
    *LANG.get_or_init(|| {
        for var in ["LC_ALL", "LC_MESSAGES", "LANG"] {
            if let Ok(value) = std::env::var(var)
                && !value.is_empty()
            {
                return if value.starts_with("nl") {
                    Lang::Nl
                } else {
                    Lang::En
                };
            }
        }
        Lang::En
    })
}

fn pick(en: &'static str, nl: &'static str) -> &'static str {
    match lang() {
        Lang::En => en,
        Lang::Nl => nl,
    }
}

pub fn app_name() -> &'static str {
    pick("ADB GUI", "ADB GUI")
}

pub fn button_activate() -> &'static str {
    pick("Activate", "Activateren")
}

pub fn button_add() -> &'static str {
    pick("Add", "Toevoegen")
}

pub fn button_backspace() -> &'static str {
    pick("Backspace", "Backspace")
}

pub fn button_browse() -> &'static str {
    pick("Browse", "Bladeren")
}

pub fn button_cancel() -> &'static str {
    pick("Cancel", "Annuleren")
}

pub fn button_capture_screenshot() -> &'static str {
    pick("Capture screenshot", "Maak screenshot")
}

pub fn button_capture() -> &'static str {
    pick("Capture", "Vastleggen")
}

pub fn button_connect() -> &'static str {
    pick("Connect", "Verbinden")
}

pub fn button_deactivate() -> &'static str {
    pick("Deactivate", "Deactivateren")
}

pub fn button_delete() -> &'static str {
    pick("Delete", "Verwijderen")
}

pub fn button_disconnect() -> &'static str {
    pick("Disconnect", "Verbinding verbreken")
}

pub fn button_duplicate() -> &'static str {
    pick("Duplicate", "Dupliceren")
}

pub fn button_edit() -> &'static str {
    pick("Edit", "Bewerken")
}

pub fn button_edit_parameters() -> &'static str {
    pick("Edit parameters", "Parameters bewerken")
}

pub fn button_install() -> &'static str {
    pick("Install", "Installeren")
}

pub fn button_move_down() -> &'static str {
    pick("Move down", "Verplaats naar beneden")
}

pub fn button_move_up() -> &'static str {
    pick("Move up", "Verplaats naar boven")
}

pub fn button_refresh() -> &'static str {
    pick("Refresh", "Verversen")
}

pub fn button_run() -> &'static str {
    pick("Run", "Uitvoeren")
}

pub fn button_save() -> &'static str {
    pick("Save", "Opslaan")
}

pub fn button_select() -> &'static str {
    pick("Select", "Selecteren")
}

pub fn button_send() -> &'static str {
    pick("Send", "Versturen")
}

pub fn button_tab() -> &'static str {
    pick("Tab", "Tab")
}

pub fn copy_name_postfix() -> &'static str {
    pick("copy", "kopie")
}

pub fn dialog_message_confirm_delete(name: &str) -> String {
    pick(
        "Are you sure you want to delete {name}?",
        "Ben je zeker dat je {name} wil verwijderen?",
    )
    .replace("{name}", name)
}

pub fn dialog_title_automation_add() -> &'static str {
    pick("Add automation", "Automation toevoegen")
}

pub fn dialog_title_automation_delete() -> &'static str {
    pick("Delete automation", "Automation verwijderen")
}

pub fn dialog_title_automation_edit() -> &'static str {
    pick("Edit automation", "Automation bewerken")
}

pub fn dialog_title_connection_add() -> &'static str {
    pick("Add connection", "Connectie toevoegen")
}

pub fn dialog_title_connection_delete() -> &'static str {
    pick("Delete connection", "Connectie verwijderen")
}

pub fn dialog_title_connection_edit() -> &'static str {
    pick("Edit connection", "Connectie bewerken")
}

pub fn dialog_title_emulator_params_edit() -> &'static str {
    pick("Edit emulator parameters", "Emulator parameters bewerken")
}

pub fn dialog_title_proxy_add() -> &'static str {
    pick("Add proxy", "Proxy toevoegen")
}

pub fn dialog_title_proxy_delete() -> &'static str {
    pick("Delete proxy", "Proxy verwijderen")
}

pub fn dialog_title_proxy_edit() -> &'static str {
    pick("Edit proxy", "Proxy bewerken")
}

pub fn error_missing_address() -> &'static str {
    pick("Address is a required field", "Adres is een verplicht veld")
}

pub fn error_missing_name() -> &'static str {
    pick("Name is a required field", "Naam is een verplicht veld")
}

pub fn field_address() -> &'static str {
    pick("Address", "Adres")
}

pub fn field_command() -> &'static str {
    pick("Command", "Commando")
}

pub fn field_device() -> &'static str {
    pick("Device", "Toestel")
}

pub fn field_from() -> &'static str {
    pick("From", "Van")
}

pub fn field_keyboard_input() -> &'static str {
    pick("Keyboard input", "Toetsenbord invoer")
}

pub fn field_mode() -> &'static str {
    pick("Mode", "Modus")
}

pub fn field_name() -> &'static str {
    pick("Name", "Naam")
}

pub fn field_no_snapshot_load() -> &'static str {
    pick("No snapshot load", "Geen snapshot start")
}

pub fn field_operation() -> &'static str {
    pick("Operation", "Bewerking")
}

pub fn field_package_path() -> &'static str {
    pick("Package path", "Pad naar pakket")
}

pub fn field_screenshot_directory() -> &'static str {
    pick("Screenshot directory", "Map voor schermafbeeldingen")
}

pub fn field_setting_adb_path() -> &'static str {
    pick("ADB path", "Pad naar ADB")
}

pub fn field_setting_emulator_path() -> &'static str {
    pick("Emulator path", "Pad naar emulator")
}

pub fn field_setting_screenshot_directory() -> &'static str {
    pick(
        "Default screenshot directory",
        "Standaard map voor schermafbeeldingen",
    )
}

pub fn field_to() -> &'static str {
    pick("To", "Naar")
}

pub fn field_type() -> &'static str {
    pick("Type", "Type")
}

pub fn field_writeable_system() -> &'static str {
    pick("Writable system", "Schrijfbaar systeem")
}

pub fn page_title_automations() -> &'static str {
    pick("Automations", "Automatiseringen")
}

pub fn page_title_connections() -> &'static str {
    pick("Connections", "Connecties")
}

pub fn page_title_emulators() -> &'static str {
    pick("Emulators", "Emulators")
}

pub fn page_title_install() -> &'static str {
    pick("Install", "Installeren")
}

pub fn page_title_keyboard() -> &'static str {
    pick("Keyboard", "Toetsenbord")
}

pub fn page_title_proxies() -> &'static str {
    pick("Proxies", "Proxies")
}

pub fn page_title_remote_control() -> &'static str {
    pick("Remote control", "Afstandsbediening")
}

pub fn page_title_screenshot() -> &'static str {
    pick("Screenshot", "Schermafbeelding")
}

pub fn page_title_settings() -> &'static str {
    pick("Settings", "Instellingen")
}

pub fn table_empty() -> &'static str {
    pick("No data", "Geen data")
}

pub fn value_active() -> &'static str {
    pick("Active", "Actief")
}

pub fn value_connected() -> &'static str {
    pick("Connected", "Verbonden")
}

pub fn value_disconnected() -> &'static str {
    pick("Disconnected", "Niet verbonden")
}

pub fn value_inactive() -> &'static str {
    pick("Inactive", "Inactief")
}

pub fn warning_priv_app() -> &'static str {
    pick(
        "Warning: root access is required to install as priv app",
        "Waarschuwing: root toegang is vereist om te installeren als priv app",
    )
}

pub fn data_backed_up(backup: &str) -> String {
    format!(
        "{} {backup}",
        pick(
            "Saved data could not be read and was moved to",
            "Opgeslagen gegevens konden niet gelezen worden en zijn verplaatst naar",
        )
    )
}

pub fn data_not_saving() -> &'static str {
    pick(
        "Saved data could not be read; changes will not be saved",
        "Opgeslagen gegevens konden niet gelezen worden; wijzigingen worden niet opgeslagen",
    )
}

pub fn error_message(message: &str) -> String {
    format!("{}: {message}", pick("Error", "Fout"))
}

pub fn command_number(number: usize) -> String {
    format!("{} #{number}", pick("Command", "Commando"))
}

pub fn value_none() -> &'static str {
    pick("No device", "Geen toestel")
}

pub fn keyboard_input_placeholder() -> &'static str {
    pick("Type text to send", "Typ tekst om te versturen")
}
