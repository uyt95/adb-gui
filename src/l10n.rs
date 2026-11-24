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

pub fn button_pair() -> &'static str {
    pick("Pair", "Koppelen")
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

pub fn error_missing_pairing_code() -> &'static str {
    pick(
        "Pairing code is a required field",
        "Koppelcode is een verplicht veld",
    )
}

pub fn error_device_not_found() -> &'static str {
    pick(
        "The device was not found on the network. Turn on wireless debugging on the device",
        "Het toestel is niet gevonden op het netwerk. Zet draadloze foutopsporing aan op het toestel",
    )
}

pub fn error_paired_not_connected(name: &str, error: &str) -> String {
    match lang() {
        Lang::En => format!("Paired with {name}, but connecting failed: {error}"),
        Lang::Nl => format!("Gekoppeld met {name}, maar verbinden is mislukt: {error}"),
    }
}

pub fn error_pair_qr_failed(error: &str) -> String {
    match lang() {
        Lang::En => format!("Pairing failed: {error}. Scan the new code to try again."),
        Lang::Nl => {
            format!("Koppelen is mislukt: {error}. Scan de nieuwe code om het opnieuw te proberen.")
        }
    }
}

pub fn error_missing_name() -> &'static str {
    pick("Name is a required field", "Naam is een verplicht veld")
}

pub fn field_address() -> &'static str {
    pick("Address", "Adres")
}

pub fn field_mdns_name() -> &'static str {
    pick("mDNS name", "mDNS-naam")
}

pub fn field_pairing_address() -> &'static str {
    pick("IP address and port", "IP-adres en poort")
}

pub fn field_pairing_code() -> &'static str {
    pick("Pairing code", "Koppelcode")
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

pub fn empty_automations_title() -> &'static str {
    pick("No automations", "Geen automatiseringen")
}

pub fn empty_automations_description() -> &'static str {
    pick(
        "Add an automation with the + button to run a series of commands in one go",
        "Voeg een automatisering toe met de + knop om een reeks commando's in één keer uit te voeren",
    )
}

pub fn empty_connections_title() -> &'static str {
    pick("No connections", "Geen connecties")
}

pub fn empty_connections_description() -> &'static str {
    pick(
        "Add a connection with the + button to pair with a device or connect to it over the network",
        "Voeg een connectie toe met de + knop om met een toestel te koppelen of er via het netwerk mee te verbinden",
    )
}

pub fn empty_emulators_title() -> &'static str {
    pick("No emulators", "Geen emulators")
}

pub fn empty_emulators_description() -> &'static str {
    pick(
        "No Android virtual devices were found. Create one in Android Studio, or check the emulator path in Settings",
        "Er zijn geen Android virtual devices gevonden. Maak er een aan in Android Studio, of controleer het emulatorpad in Instellingen",
    )
}

pub fn empty_proxies_title() -> &'static str {
    pick("No proxies", "Geen proxies")
}

pub fn empty_proxies_description() -> &'static str {
    pick(
        "Add a proxy with the + button to forward a port to or from the device",
        "Voeg een proxy toe met de + knop om een poort van of naar het toestel door te sturen",
    )
}

pub fn add_connection_help() -> &'static str {
    pick(
        "How does the device allow debugging over the network?",
        "Hoe staat het toestel foutopsporing via het netwerk toe?",
    )
}

pub fn add_by_code() -> &'static str {
    pick("Pair with pairing code", "Koppelen met koppelcode")
}

pub fn add_by_code_description() -> &'static str {
    pick(
        "Wireless debugging on Android 11 and newer, for example Google TV",
        "Draadloze foutopsporing op Android 11 en nieuwer, bijvoorbeeld Google TV",
    )
}

pub fn add_by_qr() -> &'static str {
    pick("Pair with QR code", "Koppelen met QR-code")
}

pub fn add_by_qr_description() -> &'static str {
    pick(
        "Wireless debugging on an Android 11+ phone or tablet with a camera",
        "Draadloze foutopsporing op een Android 11+ telefoon of tablet met camera",
    )
}

pub fn add_by_address() -> &'static str {
    pick("Connect by address", "Verbinden via adres")
}

pub fn add_by_address_description() -> &'static str {
    pick(
        "Classic network debugging, for example after adb tcpip 5555",
        "Klassieke netwerk-foutopsporing, bijvoorbeeld na adb tcpip 5555",
    )
}

pub fn qr_code_label() -> &'static str {
    pick("QR code", "QR-code")
}

pub fn pair_code_help() -> &'static str {
    pick(
        "Android 11 and newer. On the device, open Developer options › Wireless debugging › Pair device with pairing code.",
        "Android 11 en nieuwer. Open op het toestel Ontwikkelaarsopties › Draadloze foutopsporing › Apparaat koppelen met koppelingscode.",
    )
}

pub fn pair_code_found() -> &'static str {
    pick("Found on the network", "Gevonden op het netwerk")
}

pub fn pair_qr_help() -> &'static str {
    pick(
        "Android 11 and newer, on a phone or tablet. On the device, open Developer options › Wireless debugging › Pair device with QR code, and scan this code.",
        "Android 11 en nieuwer, op een telefoon of tablet. Open op het toestel Ontwikkelaarsopties › Draadloze foutopsporing › Apparaat koppelen met QR-code, en scan deze code.",
    )
}

pub fn pair_qr_waiting() -> &'static str {
    pick(
        "Waiting for the device to scan the code…",
        "Wachten tot het toestel de code scant…",
    )
}

pub fn pair_in_progress() -> &'static str {
    pick("Pairing…", "Koppelen…")
}

pub fn pair_address_help() -> &'static str {
    pick(
        "For devices with classic network debugging, for example after adb tcpip 5555. Without a port, 5555 is used.",
        "Voor toestellen met klassieke netwerk-foutopsporing, bijvoorbeeld na adb tcpip 5555. Zonder poort wordt 5555 gebruikt.",
    )
}

pub fn paired_with(name: &str) -> String {
    match lang() {
        Lang::En => format!("Paired with {name}"),
        Lang::Nl => format!("Gekoppeld met {name}"),
    }
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

pub fn import_failed(path: &str) -> String {
    format!(
        "{} {path}",
        pick(
            "Data of the previous version could not be imported from",
            "Gegevens van de vorige versie konden niet geïmporteerd worden uit",
        )
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

pub fn console_title() -> &'static str {
    pick("Console", "Console")
}

pub fn console_toggle() -> &'static str {
    pick("Console (Ctrl+`)", "Console (Ctrl+`)")
}

pub fn console_empty() -> &'static str {
    pick(
        "Commands the app sends show up here.",
        "Opdrachten die de app verstuurt, verschijnen hier.",
    )
}

pub fn console_no_failed() -> &'static str {
    pick("No failed commands.", "Geen mislukte opdrachten.")
}

pub fn console_filter_all() -> &'static str {
    pick("All", "Alle")
}

pub fn console_filter_failed() -> &'static str {
    pick("Failed", "Mislukt")
}

pub fn console_count(total: usize, failed: usize) -> String {
    pick(
        "{total} commands · {failed} failed",
        "{total} opdrachten · {failed} mislukt",
    )
    .replace("{total}", &total.to_string())
    .replace("{failed}", &failed.to_string())
}

pub fn button_copy_all() -> &'static str {
    pick("Copy all", "Alles kopiëren")
}

pub fn button_clear() -> &'static str {
    pick("Clear", "Wissen")
}

pub fn button_hide_console() -> &'static str {
    pick("Hide console", "Console verbergen")
}

pub fn button_scroll_to_newest() -> &'static str {
    pick(
        "Scroll to the newest command",
        "Naar de nieuwste opdracht scrollen",
    )
}

pub fn console_running() -> &'static str {
    pick("running…", "bezig…")
}

pub fn console_launched() -> &'static str {
    pick("launched", "gestart")
}

pub fn console_not_started() -> &'static str {
    pick("not started", "niet gestart")
}

pub fn console_signal() -> &'static str {
    pick("stopped", "gestopt")
}

pub fn console_sent(time: &str) -> String {
    format!("{} {time}", pick("Sent", "Verstuurd"))
}

pub fn console_no_output() -> &'static str {
    pick("No output.", "Geen uitvoer.")
}

pub fn console_running_note() -> &'static str {
    pick(
        "The output appears when the command finishes.",
        "De uitvoer verschijnt wanneer de opdracht klaar is.",
    )
}

pub fn console_launched_note() -> &'static str {
    pick(
        "Started in the background. Its output is not captured.",
        "Op de achtergrond gestart. De uitvoer wordt niet bijgehouden.",
    )
}

pub fn console_rejected_note() -> &'static str {
    pick(
        "The command exited with 0, but the app did not accept its output, so it counts as failed.",
        "De opdracht eindigde met 0, maar de app aanvaardde de uitvoer niet, dus ze telt als mislukt.",
    )
}

pub fn console_signal_note() -> &'static str {
    pick(
        "The command was stopped by a signal.",
        "De opdracht werd gestopt door een signaal.",
    )
}

pub fn console_dropped(bytes: usize) -> String {
    pick(
        "{bytes} more bytes were not kept.",
        "{bytes} bytes meer werden niet bijgehouden.",
    )
    .replace("{bytes}", &bytes.to_string())
}
