//! Android key codes selectable in automations, sorted by label.
//!
//! Each label ends with the first Android version that knows the code.
//! `input keyevent` resolves the name on the device, so a code newer than
//! the device fails there. Source: the `KeyEvent` reference (API level 37).

pub struct KeyCode {
    pub label: &'static str,
    pub code: &'static str,
}

pub const KEY_CODES: &[KeyCode] = &[
    KeyCode {
        label: "# key (Android 1.0+)",
        code: "KEYCODE_POUND",
    },
    KeyCode {
        label: "' key (Android 1.0+)",
        code: "KEYCODE_APOSTROPHE",
    },
    KeyCode {
        label: "* key (Android 1.0+)",
        code: "KEYCODE_STAR",
    },
    KeyCode {
        label: "+ key (Android 1.0+)",
        code: "KEYCODE_PLUS",
    },
    KeyCode {
        label: ", key (Android 1.0+)",
        code: "KEYCODE_COMMA",
    },
    KeyCode {
        label: "- key (Android 1.0+)",
        code: "KEYCODE_MINUS",
    },
    KeyCode {
        label: ". key (Android 1.0+)",
        code: "KEYCODE_PERIOD",
    },
    KeyCode {
        label: "/ key (Android 1.0+)",
        code: "KEYCODE_SLASH",
    },
    KeyCode {
        label: "0 key (Android 1.0+)",
        code: "KEYCODE_0",
    },
    KeyCode {
        label: "1 key (Android 1.0+)",
        code: "KEYCODE_1",
    },
    KeyCode {
        label: "11 key (Android 5.0+)",
        code: "KEYCODE_11",
    },
    KeyCode {
        label: "12 key (Android 5.0+)",
        code: "KEYCODE_12",
    },
    KeyCode {
        label: "2 key (Android 1.0+)",
        code: "KEYCODE_2",
    },
    KeyCode {
        label: "3 key (Android 1.0+)",
        code: "KEYCODE_3",
    },
    KeyCode {
        label: "3D Mode key (Android 4.0+)",
        code: "KEYCODE_3D_MODE",
    },
    KeyCode {
        label: "4 key (Android 1.0+)",
        code: "KEYCODE_4",
    },
    KeyCode {
        label: "5 key (Android 1.0+)",
        code: "KEYCODE_5",
    },
    KeyCode {
        label: "6 key (Android 1.0+)",
        code: "KEYCODE_6",
    },
    KeyCode {
        label: "7 key (Android 1.0+)",
        code: "KEYCODE_7",
    },
    KeyCode {
        label: "8 key (Android 1.0+)",
        code: "KEYCODE_8",
    },
    KeyCode {
        label: "9 key (Android 1.0+)",
        code: "KEYCODE_9",
    },
    KeyCode {
        label: "; key (Android 1.0+)",
        code: "KEYCODE_SEMICOLON",
    },
    KeyCode {
        label: "= key (Android 1.0+)",
        code: "KEYCODE_EQUALS",
    },
    KeyCode {
        label: "@ key (Android 1.0+)",
        code: "KEYCODE_AT",
    },
    KeyCode {
        label: "A Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_A",
    },
    KeyCode {
        label: "A key (Android 1.0+)",
        code: "KEYCODE_A",
    },
    KeyCode {
        label: "A/V Receiver input key (Android 3.0+)",
        code: "KEYCODE_AVR_INPUT",
    },
    KeyCode {
        label: "A/V Receiver power key (Android 3.0+)",
        code: "KEYCODE_AVR_POWER",
    },
    KeyCode {
        label: "Accessibility key (Android 17+)",
        code: "KEYCODE_ACCESSIBILITY",
    },
    KeyCode {
        label: "Analog Terrestrial key (Android 5.0+)",
        code: "KEYCODE_TV_TERRESTRIAL_ANALOG",
    },
    KeyCode {
        label: "Antenna/Cable key (Android 5.0+)",
        code: "KEYCODE_TV_ANTENNA_CABLE",
    },
    KeyCode {
        label: "App switch key (Android 3.0+)",
        code: "KEYCODE_APP_SWITCH",
    },
    KeyCode {
        label: "Assist key (Android 4.1+)",
        code: "KEYCODE_ASSIST",
    },
    KeyCode {
        label: "Audio Track key (Android 4.4+)",
        code: "KEYCODE_MEDIA_AUDIO_TRACK",
    },
    KeyCode {
        label: "Audio description key (Android 5.0+)",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION",
    },
    KeyCode {
        label: "Audio description mixing volume down key (Android 5.0+)",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN",
    },
    KeyCode {
        label: "Audio description mixing volume up key (Android 5.0+)",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP",
    },
    KeyCode {
        label: "B Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_B",
    },
    KeyCode {
        label: "B key (Android 1.0+)",
        code: "KEYCODE_B",
    },
    KeyCode {
        label: "BS key (Android 5.0+)",
        code: "KEYCODE_TV_SATELLITE_BS",
    },
    KeyCode {
        label: "BS/CS key (Android 5.0+)",
        code: "KEYCODE_TV_SATELLITE_SERVICE",
    },
    KeyCode {
        label: "Back key (Android 1.0+)",
        code: "KEYCODE_BACK",
    },
    KeyCode {
        label: "Backspace key (Android 1.0+)",
        code: "KEYCODE_DEL",
    },
    KeyCode {
        label: "Blue 'programmable' key (Android 3.0+)",
        code: "KEYCODE_PROG_BLUE",
    },
    KeyCode {
        label: "Bookmark key (Android 3.0+)",
        code: "KEYCODE_BOOKMARK",
    },
    KeyCode {
        label: "Break / Pause key (Android 3.0+)",
        code: "KEYCODE_BREAK",
    },
    KeyCode {
        label: "Brightness Down key (Android 4.3+)",
        code: "KEYCODE_BRIGHTNESS_DOWN",
    },
    KeyCode {
        label: "Brightness Up key (Android 4.3+)",
        code: "KEYCODE_BRIGHTNESS_UP",
    },
    KeyCode {
        label: "C Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_C",
    },
    KeyCode {
        label: "C key (Android 1.0+)",
        code: "KEYCODE_C",
    },
    KeyCode {
        label: "CS key (Android 5.0+)",
        code: "KEYCODE_TV_SATELLITE_CS",
    },
    KeyCode {
        label: "Calculator special function key (Android 4.0.3+)",
        code: "KEYCODE_CALCULATOR",
    },
    KeyCode {
        label: "Calendar special function key (Android 4.0.3+)",
        code: "KEYCODE_CALENDAR",
    },
    KeyCode {
        label: "Call key (Android 1.0+)",
        code: "KEYCODE_CALL",
    },
    KeyCode {
        label: "Camera Focus key (Android 1.0+)",
        code: "KEYCODE_FOCUS",
    },
    KeyCode {
        label: "Camera key (Android 1.0+)",
        code: "KEYCODE_CAMERA",
    },
    KeyCode {
        label: "Caps Lock key (Android 3.0+)",
        code: "KEYCODE_CAPS_LOCK",
    },
    KeyCode {
        label: "Channel down key (Android 3.0+)",
        code: "KEYCODE_CHANNEL_DOWN",
    },
    KeyCode {
        label: "Channel up key (Android 3.0+)",
        code: "KEYCODE_CHANNEL_UP",
    },
    KeyCode {
        label: "Clear key (Android 1.0+)",
        code: "KEYCODE_CLEAR",
    },
    KeyCode {
        label: "Close key (Android 16+)",
        code: "KEYCODE_CLOSE",
    },
    KeyCode {
        label: "Close media key (Android 3.0+)",
        code: "KEYCODE_MEDIA_CLOSE",
    },
    KeyCode {
        label: "Component #1 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_COMPONENT_1",
    },
    KeyCode {
        label: "Component #2 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_COMPONENT_2",
    },
    KeyCode {
        label: "Composite #1 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_COMPOSITE_1",
    },
    KeyCode {
        label: "Composite #2 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_COMPOSITE_2",
    },
    KeyCode {
        label: "Contacts special function key (Android 4.0.3+)",
        code: "KEYCODE_CONTACTS",
    },
    KeyCode {
        label: "Contents menu key (Android 5.0+)",
        code: "KEYCODE_TV_CONTENTS_MENU",
    },
    KeyCode {
        label: "Contextual Insert key (Android 17+)",
        code: "KEYCODE_CONTEXTUAL_INSERT",
    },
    KeyCode {
        label: "Contextual Query key (Android 17, SDK 37.1+)",
        code: "KEYCODE_CONTEXTUAL_QUERY",
    },
    KeyCode {
        label: "Contextual Search key (Android 17+)",
        code: "KEYCODE_CONTEXTUAL_SEARCH",
    },
    KeyCode {
        label: "Copy key (Android 7.0+)",
        code: "KEYCODE_COPY",
    },
    KeyCode {
        label: "Cut key (Android 7.0+)",
        code: "KEYCODE_CUT",
    },
    KeyCode {
        label: "D key (Android 1.0+)",
        code: "KEYCODE_D",
    },
    KeyCode {
        label: "DVR key (Android 3.0+)",
        code: "KEYCODE_DVR",
    },
    KeyCode {
        label: "Demo App #1 key (Android 13+)",
        code: "KEYCODE_DEMO_APP_1",
    },
    KeyCode {
        label: "Demo App #2 key (Android 13+)",
        code: "KEYCODE_DEMO_APP_2",
    },
    KeyCode {
        label: "Demo App #3 key (Android 13+)",
        code: "KEYCODE_DEMO_APP_3",
    },
    KeyCode {
        label: "Demo App #4 key (Android 13+)",
        code: "KEYCODE_DEMO_APP_4",
    },
    KeyCode {
        label: "Dictate key (Android 16+)",
        code: "KEYCODE_DICTATE",
    },
    KeyCode {
        label: "Digital Terrestrial key (Android 5.0+)",
        code: "KEYCODE_TV_TERRESTRIAL_DIGITAL",
    },
    KeyCode {
        label: "Directional Pad Center key (Android 1.0+)",
        code: "KEYCODE_DPAD_CENTER",
    },
    KeyCode {
        label: "Directional Pad Down key (Android 1.0+)",
        code: "KEYCODE_DPAD_DOWN",
    },
    KeyCode {
        label: "Directional Pad Down-Left key (Android 7.0+)",
        code: "KEYCODE_DPAD_DOWN_LEFT",
    },
    KeyCode {
        label: "Directional Pad Down-Right key (Android 7.0+)",
        code: "KEYCODE_DPAD_DOWN_RIGHT",
    },
    KeyCode {
        label: "Directional Pad Left key (Android 1.0+)",
        code: "KEYCODE_DPAD_LEFT",
    },
    KeyCode {
        label: "Directional Pad Right key (Android 1.0+)",
        code: "KEYCODE_DPAD_RIGHT",
    },
    KeyCode {
        label: "Directional Pad Up key (Android 1.0+)",
        code: "KEYCODE_DPAD_UP",
    },
    KeyCode {
        label: "Directional Pad Up-Left key (Android 7.0+)",
        code: "KEYCODE_DPAD_UP_LEFT",
    },
    KeyCode {
        label: "Directional Pad Up-Right key (Android 7.0+)",
        code: "KEYCODE_DPAD_UP_RIGHT",
    },
    KeyCode {
        label: "Do Not Disturb key (Android 16+)",
        code: "KEYCODE_DO_NOT_DISTURB",
    },
    KeyCode {
        label: "E key (Android 1.0+)",
        code: "KEYCODE_E",
    },
    KeyCode {
        label: "Eject media key (Android 3.0+)",
        code: "KEYCODE_MEDIA_EJECT",
    },
    KeyCode {
        label: "Emoji Picker key (Android 15+)",
        code: "KEYCODE_EMOJI_PICKER",
    },
    KeyCode {
        label: "End Call key (Android 1.0+)",
        code: "KEYCODE_ENDCALL",
    },
    KeyCode {
        label: "End Movement key (Android 3.0+)",
        code: "KEYCODE_MOVE_END",
    },
    KeyCode {
        label: "Enter key (Android 1.0+)",
        code: "KEYCODE_ENTER",
    },
    KeyCode {
        label: "Envelope special function key (Android 1.0+)",
        code: "KEYCODE_ENVELOPE",
    },
    KeyCode {
        label: "Escape key (Android 3.0+)",
        code: "KEYCODE_ESCAPE",
    },
    KeyCode {
        label: "Explorer special function key (Android 1.0+)",
        code: "KEYCODE_EXPLORER",
    },
    KeyCode {
        label: "F key (Android 1.0+)",
        code: "KEYCODE_F",
    },
    KeyCode {
        label: "F1 key (Android 3.0+)",
        code: "KEYCODE_F1",
    },
    KeyCode {
        label: "F10 key (Android 3.0+)",
        code: "KEYCODE_F10",
    },
    KeyCode {
        label: "F11 key (Android 3.0+)",
        code: "KEYCODE_F11",
    },
    KeyCode {
        label: "F12 key (Android 3.0+)",
        code: "KEYCODE_F12",
    },
    KeyCode {
        label: "F13 key (Android 16+)",
        code: "KEYCODE_F13",
    },
    KeyCode {
        label: "F14 key (Android 16+)",
        code: "KEYCODE_F14",
    },
    KeyCode {
        label: "F15 key (Android 16+)",
        code: "KEYCODE_F15",
    },
    KeyCode {
        label: "F16 key (Android 16+)",
        code: "KEYCODE_F16",
    },
    KeyCode {
        label: "F17 key (Android 16+)",
        code: "KEYCODE_F17",
    },
    KeyCode {
        label: "F18 key (Android 16+)",
        code: "KEYCODE_F18",
    },
    KeyCode {
        label: "F19 key (Android 16+)",
        code: "KEYCODE_F19",
    },
    KeyCode {
        label: "F2 key (Android 3.0+)",
        code: "KEYCODE_F2",
    },
    KeyCode {
        label: "F20 key (Android 16+)",
        code: "KEYCODE_F20",
    },
    KeyCode {
        label: "F21 key (Android 16+)",
        code: "KEYCODE_F21",
    },
    KeyCode {
        label: "F22 key (Android 16+)",
        code: "KEYCODE_F22",
    },
    KeyCode {
        label: "F23 key (Android 16+)",
        code: "KEYCODE_F23",
    },
    KeyCode {
        label: "F24 key (Android 16+)",
        code: "KEYCODE_F24",
    },
    KeyCode {
        label: "F3 key (Android 3.0+)",
        code: "KEYCODE_F3",
    },
    KeyCode {
        label: "F4 key (Android 3.0+)",
        code: "KEYCODE_F4",
    },
    KeyCode {
        label: "F5 key (Android 3.0+)",
        code: "KEYCODE_F5",
    },
    KeyCode {
        label: "F6 key (Android 3.0+)",
        code: "KEYCODE_F6",
    },
    KeyCode {
        label: "F7 key (Android 3.0+)",
        code: "KEYCODE_F7",
    },
    KeyCode {
        label: "F8 key (Android 3.0+)",
        code: "KEYCODE_F8",
    },
    KeyCode {
        label: "F9 key (Android 3.0+)",
        code: "KEYCODE_F9",
    },
    KeyCode {
        label: "Fast Forward media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_FAST_FORWARD",
    },
    KeyCode {
        label: "Featured App #1 key (Android 13+)",
        code: "KEYCODE_FEATURED_APP_1",
    },
    KeyCode {
        label: "Featured App #2 key (Android 13+)",
        code: "KEYCODE_FEATURED_APP_2",
    },
    KeyCode {
        label: "Featured App #3 key (Android 13+)",
        code: "KEYCODE_FEATURED_APP_3",
    },
    KeyCode {
        label: "Featured App #4 key (Android 13+)",
        code: "KEYCODE_FEATURED_APP_4",
    },
    KeyCode {
        label: "Forward Delete key (Android 3.0+)",
        code: "KEYCODE_FORWARD_DEL",
    },
    KeyCode {
        label: "Forward key (Android 3.0+)",
        code: "KEYCODE_FORWARD",
    },
    KeyCode {
        label: "Fullscreen key (Android 16+)",
        code: "KEYCODE_FULLSCREEN",
    },
    KeyCode {
        label: "Function key (Android 3.0+)",
        code: "KEYCODE_FUNCTION",
    },
    KeyCode {
        label: "G key (Android 1.0+)",
        code: "KEYCODE_G",
    },
    KeyCode {
        label: "Generic Game Pad Button #1 (Android 3.1+)",
        code: "KEYCODE_BUTTON_1",
    },
    KeyCode {
        label: "Generic Game Pad Button #10 (Android 3.1+)",
        code: "KEYCODE_BUTTON_10",
    },
    KeyCode {
        label: "Generic Game Pad Button #11 (Android 3.1+)",
        code: "KEYCODE_BUTTON_11",
    },
    KeyCode {
        label: "Generic Game Pad Button #12 (Android 3.1+)",
        code: "KEYCODE_BUTTON_12",
    },
    KeyCode {
        label: "Generic Game Pad Button #13 (Android 3.1+)",
        code: "KEYCODE_BUTTON_13",
    },
    KeyCode {
        label: "Generic Game Pad Button #14 (Android 3.1+)",
        code: "KEYCODE_BUTTON_14",
    },
    KeyCode {
        label: "Generic Game Pad Button #15 (Android 3.1+)",
        code: "KEYCODE_BUTTON_15",
    },
    KeyCode {
        label: "Generic Game Pad Button #16 (Android 3.1+)",
        code: "KEYCODE_BUTTON_16",
    },
    KeyCode {
        label: "Generic Game Pad Button #2 (Android 3.1+)",
        code: "KEYCODE_BUTTON_2",
    },
    KeyCode {
        label: "Generic Game Pad Button #3 (Android 3.1+)",
        code: "KEYCODE_BUTTON_3",
    },
    KeyCode {
        label: "Generic Game Pad Button #4 (Android 3.1+)",
        code: "KEYCODE_BUTTON_4",
    },
    KeyCode {
        label: "Generic Game Pad Button #5 (Android 3.1+)",
        code: "KEYCODE_BUTTON_5",
    },
    KeyCode {
        label: "Generic Game Pad Button #6 (Android 3.1+)",
        code: "KEYCODE_BUTTON_6",
    },
    KeyCode {
        label: "Generic Game Pad Button #7 (Android 3.1+)",
        code: "KEYCODE_BUTTON_7",
    },
    KeyCode {
        label: "Generic Game Pad Button #8 (Android 3.1+)",
        code: "KEYCODE_BUTTON_8",
    },
    KeyCode {
        label: "Generic Game Pad Button #9 (Android 3.1+)",
        code: "KEYCODE_BUTTON_9",
    },
    KeyCode {
        label: "Generic stem key 1 for Wear (Android 7.0+)",
        code: "KEYCODE_STEM_1",
    },
    KeyCode {
        label: "Generic stem key 2 for Wear (Android 7.0+)",
        code: "KEYCODE_STEM_2",
    },
    KeyCode {
        label: "Generic stem key 3 for Wear (Android 7.0+)",
        code: "KEYCODE_STEM_3",
    },
    KeyCode {
        label: "Green 'programmable' key (Android 3.0+)",
        code: "KEYCODE_PROG_GREEN",
    },
    KeyCode {
        label: "Guide key (Android 3.0+)",
        code: "KEYCODE_GUIDE",
    },
    KeyCode {
        label: "H key (Android 1.0+)",
        code: "KEYCODE_H",
    },
    KeyCode {
        label: "HDMI #1 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_HDMI_1",
    },
    KeyCode {
        label: "HDMI #2 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_HDMI_2",
    },
    KeyCode {
        label: "HDMI #3 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_HDMI_3",
    },
    KeyCode {
        label: "HDMI #4 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_HDMI_4",
    },
    KeyCode {
        label: "Headset Hook key (Android 1.0+)",
        code: "KEYCODE_HEADSETHOOK",
    },
    KeyCode {
        label: "Help key (Android 5.0+)",
        code: "KEYCODE_HELP",
    },
    KeyCode {
        label: "Home Movement key (Android 3.0+)",
        code: "KEYCODE_MOVE_HOME",
    },
    KeyCode {
        label: "Home key (Android 1.0+)",
        code: "KEYCODE_HOME",
    },
    KeyCode {
        label: "I key (Android 1.0+)",
        code: "KEYCODE_I",
    },
    KeyCode {
        label: "Info key (Android 3.0+)",
        code: "KEYCODE_INFO",
    },
    KeyCode {
        label: "Insert key (Android 3.0+)",
        code: "KEYCODE_INSERT",
    },
    KeyCode {
        label: "J key (Android 1.0+)",
        code: "KEYCODE_J",
    },
    KeyCode {
        label: "Japanese Ro key (Android 4.1+)",
        code: "KEYCODE_RO",
    },
    KeyCode {
        label: "Japanese Yen key (Android 4.1+)",
        code: "KEYCODE_YEN",
    },
    KeyCode {
        label: "Japanese alphanumeric key (Android 4.1+)",
        code: "KEYCODE_EISU",
    },
    KeyCode {
        label: "Japanese conversion key (Android 4.1+)",
        code: "KEYCODE_HENKAN",
    },
    KeyCode {
        label: "Japanese full-width / half-width key (Android 4.1+)",
        code: "KEYCODE_ZENKAKU_HANKAKU",
    },
    KeyCode {
        label: "Japanese kana key (Android 4.1+)",
        code: "KEYCODE_KANA",
    },
    KeyCode {
        label: "Japanese katakana / hiragana key (Android 4.1+)",
        code: "KEYCODE_KATAKANA_HIRAGANA",
    },
    KeyCode {
        label: "Japanese non-conversion key (Android 4.1+)",
        code: "KEYCODE_MUHENKAN",
    },
    KeyCode {
        label: "K key (Android 1.0+)",
        code: "KEYCODE_K",
    },
    KeyCode {
        label: "Keyboard Backlight Down key (Android 14+)",
        code: "KEYCODE_KEYBOARD_BACKLIGHT_DOWN",
    },
    KeyCode {
        label: "Keyboard Backlight Toggle key (Android 14+)",
        code: "KEYCODE_KEYBOARD_BACKLIGHT_TOGGLE",
    },
    KeyCode {
        label: "Keyboard Backlight Up key (Android 14+)",
        code: "KEYCODE_KEYBOARD_BACKLIGHT_UP",
    },
    KeyCode {
        label: "L key (Android 1.0+)",
        code: "KEYCODE_L",
    },
    KeyCode {
        label: "L1 Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_L1",
    },
    KeyCode {
        label: "L2 Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_L2",
    },
    KeyCode {
        label: "Language Switch key (Android 4.0+)",
        code: "KEYCODE_LANGUAGE_SWITCH",
    },
    KeyCode {
        label: "Last Channel key (Android 5.0+)",
        code: "KEYCODE_LAST_CHANNEL",
    },
    KeyCode {
        label: "Left Alt modifier key (Android 1.0+)",
        code: "KEYCODE_ALT_LEFT",
    },
    KeyCode {
        label: "Left Control modifier key (Android 3.0+)",
        code: "KEYCODE_CTRL_LEFT",
    },
    KeyCode {
        label: "Left Meta modifier key (Android 3.0+)",
        code: "KEYCODE_META_LEFT",
    },
    KeyCode {
        label: "Left Shift modifier key (Android 1.0+)",
        code: "KEYCODE_SHIFT_LEFT",
    },
    KeyCode {
        label: "Left Thumb Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_THUMBL",
    },
    KeyCode {
        label: "Lock Screen key (Android 16+)",
        code: "KEYCODE_LOCK",
    },
    KeyCode {
        label: "M key (Android 1.0+)",
        code: "KEYCODE_M",
    },
    KeyCode {
        label: "Macro #1 key (Android 14+)",
        code: "KEYCODE_MACRO_1",
    },
    KeyCode {
        label: "Macro #2 key (Android 14+)",
        code: "KEYCODE_MACRO_2",
    },
    KeyCode {
        label: "Macro #3 key (Android 14+)",
        code: "KEYCODE_MACRO_3",
    },
    KeyCode {
        label: "Macro #4 key (Android 14+)",
        code: "KEYCODE_MACRO_4",
    },
    KeyCode {
        label: "Manner Mode key (Android 4.0+)",
        code: "KEYCODE_MANNER_MODE",
    },
    KeyCode {
        label: "Media Top Menu key (Android 5.0+)",
        code: "KEYCODE_MEDIA_TOP_MENU",
    },
    KeyCode {
        label: "Media context menu key (Android 5.0+)",
        code: "KEYCODE_TV_MEDIA_CONTEXT_MENU",
    },
    KeyCode {
        label: "Menu key (Android 1.0+)",
        code: "KEYCODE_MENU",
    },
    KeyCode {
        label: "Mode Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_MODE",
    },
    KeyCode {
        label: "Music special function key (Android 4.0.3+)",
        code: "KEYCODE_MUSIC",
    },
    KeyCode {
        label: "Mute key (Android 1.5+)",
        code: "KEYCODE_MUTE",
    },
    KeyCode {
        label: "N key (Android 1.0+)",
        code: "KEYCODE_N",
    },
    KeyCode {
        label: "Navigate in key (Android 6.0+)",
        code: "KEYCODE_NAVIGATE_IN",
    },
    KeyCode {
        label: "Navigate out key (Android 6.0+)",
        code: "KEYCODE_NAVIGATE_OUT",
    },
    KeyCode {
        label: "Navigate to next key (Android 6.0+)",
        code: "KEYCODE_NAVIGATE_NEXT",
    },
    KeyCode {
        label: "Navigate to previous key (Android 6.0+)",
        code: "KEYCODE_NAVIGATE_PREVIOUS",
    },
    KeyCode {
        label: "New key (Android 16+)",
        code: "KEYCODE_NEW",
    },
    KeyCode {
        label: "Notification key (Android 1.0+)",
        code: "KEYCODE_NOTIFICATION",
    },
    KeyCode {
        label: "Num Lock key (Android 3.0+)",
        code: "KEYCODE_NUM_LOCK",
    },
    KeyCode {
        label: "Number entry key (Android 5.0+)",
        code: "KEYCODE_TV_NUMBER_ENTRY",
    },
    KeyCode {
        label: "Number modifier key (Android 1.0+)",
        code: "KEYCODE_NUM",
    },
    KeyCode {
        label: "Numeric keypad ( key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_LEFT_PAREN",
    },
    KeyCode {
        label: "Numeric keypad ) key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_RIGHT_PAREN",
    },
    KeyCode {
        label: "Numeric keypad * key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_MULTIPLY",
    },
    KeyCode {
        label: "Numeric keypad + key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_ADD",
    },
    KeyCode {
        label: "Numeric keypad , key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_COMMA",
    },
    KeyCode {
        label: "Numeric keypad - key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_SUBTRACT",
    },
    KeyCode {
        label: "Numeric keypad . key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_DOT",
    },
    KeyCode {
        label: "Numeric keypad / key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_DIVIDE",
    },
    KeyCode {
        label: "Numeric keypad 0 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_0",
    },
    KeyCode {
        label: "Numeric keypad 1 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_1",
    },
    KeyCode {
        label: "Numeric keypad 2 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_2",
    },
    KeyCode {
        label: "Numeric keypad 3 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_3",
    },
    KeyCode {
        label: "Numeric keypad 4 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_4",
    },
    KeyCode {
        label: "Numeric keypad 5 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_5",
    },
    KeyCode {
        label: "Numeric keypad 6 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_6",
    },
    KeyCode {
        label: "Numeric keypad 7 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_7",
    },
    KeyCode {
        label: "Numeric keypad 8 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_8",
    },
    KeyCode {
        label: "Numeric keypad 9 key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_9",
    },
    KeyCode {
        label: "Numeric keypad = key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_EQUALS",
    },
    KeyCode {
        label: "Numeric keypad Enter key (Android 3.0+)",
        code: "KEYCODE_NUMPAD_ENTER",
    },
    KeyCode {
        label: "O key (Android 1.0+)",
        code: "KEYCODE_O",
    },
    KeyCode {
        label: "P key (Android 1.0+)",
        code: "KEYCODE_P",
    },
    KeyCode {
        label: "Page Down key (Android 2.3+)",
        code: "KEYCODE_PAGE_DOWN",
    },
    KeyCode {
        label: "Page Up key (Android 2.3+)",
        code: "KEYCODE_PAGE_UP",
    },
    KeyCode {
        label: "Pairing key (Android 5.0+)",
        code: "KEYCODE_PAIRING",
    },
    KeyCode {
        label: "Paste key (Android 7.0+)",
        code: "KEYCODE_PASTE",
    },
    KeyCode {
        label: "Pause media key (Android 3.0+)",
        code: "KEYCODE_MEDIA_PAUSE",
    },
    KeyCode {
        label: "Picture Symbols modifier key (Android 2.3+)",
        code: "KEYCODE_PICTSYMBOLS",
    },
    KeyCode {
        label: "Play Next media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_NEXT",
    },
    KeyCode {
        label: "Play Previous media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_PREVIOUS",
    },
    KeyCode {
        label: "Play media key (Android 3.0+)",
        code: "KEYCODE_MEDIA_PLAY",
    },
    KeyCode {
        label: "Play/Pause media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_PLAY_PAUSE",
    },
    KeyCode {
        label: "Power key (Android 1.0+)",
        code: "KEYCODE_POWER",
    },
    KeyCode {
        label: "Primary stem key for Wear (Android 7.0+)",
        code: "KEYCODE_STEM_PRIMARY",
    },
    KeyCode {
        label: "Print key (Android 16+)",
        code: "KEYCODE_PRINT",
    },
    KeyCode {
        label: "Profile Switch key (Android 10+)",
        code: "KEYCODE_PROFILE_SWITCH",
    },
    KeyCode {
        label: "Q key (Android 1.0+)",
        code: "KEYCODE_Q",
    },
    KeyCode {
        label: "R key (Android 1.0+)",
        code: "KEYCODE_R",
    },
    KeyCode {
        label: "R1 Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_R1",
    },
    KeyCode {
        label: "R2 Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_R2",
    },
    KeyCode {
        label: "Radio key (Android 5.0+)",
        code: "KEYCODE_TV_RADIO_SERVICE",
    },
    KeyCode {
        label: "Recent Apps key (Android 14+)",
        code: "KEYCODE_RECENT_APPS",
    },
    KeyCode {
        label: "Record media key (Android 3.0+)",
        code: "KEYCODE_MEDIA_RECORD",
    },
    KeyCode {
        label: "Red 'programmable' key (Android 3.0+)",
        code: "KEYCODE_PROG_RED",
    },
    KeyCode {
        label: "Refresh key (Android 9+)",
        code: "KEYCODE_REFRESH",
    },
    KeyCode {
        label: "Rewind media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_REWIND",
    },
    KeyCode {
        label: "Right Alt modifier key (Android 1.0+)",
        code: "KEYCODE_ALT_RIGHT",
    },
    KeyCode {
        label: "Right Control modifier key (Android 3.0+)",
        code: "KEYCODE_CTRL_RIGHT",
    },
    KeyCode {
        label: "Right Meta modifier key (Android 3.0+)",
        code: "KEYCODE_META_RIGHT",
    },
    KeyCode {
        label: "Right Shift modifier key (Android 1.0+)",
        code: "KEYCODE_SHIFT_RIGHT",
    },
    KeyCode {
        label: "Right Thumb Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_THUMBR",
    },
    KeyCode {
        label: "S key (Android 1.0+)",
        code: "KEYCODE_S",
    },
    KeyCode {
        label: "Satellite key (Android 5.0+)",
        code: "KEYCODE_TV_SATELLITE",
    },
    KeyCode {
        label: "Screenshot key (Android 15+)",
        code: "KEYCODE_SCREENSHOT",
    },
    KeyCode {
        label: "Scroll Lock key (Android 3.0+)",
        code: "KEYCODE_SCROLL_LOCK",
    },
    KeyCode {
        label: "Search key (Android 1.0+)",
        code: "KEYCODE_SEARCH",
    },
    KeyCode {
        label: "Select Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_SELECT",
    },
    KeyCode {
        label: "Set-top-box input key (Android 3.0+)",
        code: "KEYCODE_STB_INPUT",
    },
    KeyCode {
        label: "Set-top-box power key (Android 3.0+)",
        code: "KEYCODE_STB_POWER",
    },
    KeyCode {
        label: "Settings key (Android 3.0+)",
        code: "KEYCODE_SETTINGS",
    },
    KeyCode {
        label: "Show all apps (Android 9+)",
        code: "KEYCODE_ALL_APPS",
    },
    KeyCode {
        label: "Skip backward media key (Android 6.0+)",
        code: "KEYCODE_MEDIA_SKIP_BACKWARD",
    },
    KeyCode {
        label: "Skip forward media key (Android 6.0+)",
        code: "KEYCODE_MEDIA_SKIP_FORWARD",
    },
    KeyCode {
        label: "Sleep key (Android 4.4W+)",
        code: "KEYCODE_SLEEP",
    },
    KeyCode {
        label: "Soft Left key (Android 1.0+)",
        code: "KEYCODE_SOFT_LEFT",
    },
    KeyCode {
        label: "Soft Right key (Android 1.0+)",
        code: "KEYCODE_SOFT_RIGHT",
    },
    KeyCode {
        label: "Soft Sleep key (Android 7.0+)",
        code: "KEYCODE_SOFT_SLEEP",
    },
    KeyCode {
        label: "Space key (Android 1.0+)",
        code: "KEYCODE_SPACE",
    },
    KeyCode {
        label: "Start Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_START",
    },
    KeyCode {
        label: "Step backward media key (Android 6.0+)",
        code: "KEYCODE_MEDIA_STEP_BACKWARD",
    },
    KeyCode {
        label: "Step forward media key (Android 6.0+)",
        code: "KEYCODE_MEDIA_STEP_FORWARD",
    },
    KeyCode {
        label: "Stop media key (Android 1.5+)",
        code: "KEYCODE_MEDIA_STOP",
    },
    KeyCode {
        label: "Stylus Primary Button key (Android 14+)",
        code: "KEYCODE_STYLUS_BUTTON_PRIMARY",
    },
    KeyCode {
        label: "Stylus Secondary Button key (Android 14+)",
        code: "KEYCODE_STYLUS_BUTTON_SECONDARY",
    },
    KeyCode {
        label: "Stylus Tail Button key (Android 14+)",
        code: "KEYCODE_STYLUS_BUTTON_TAIL",
    },
    KeyCode {
        label: "Stylus Tertiary Button key (Android 14+)",
        code: "KEYCODE_STYLUS_BUTTON_TERTIARY",
    },
    KeyCode {
        label: "Switch Charset modifier key (Android 2.3+)",
        code: "KEYCODE_SWITCH_CHARSET",
    },
    KeyCode {
        label: "Symbol modifier key (Android 1.0+)",
        code: "KEYCODE_SYM",
    },
    KeyCode {
        label: "System Navigation Down key (Android 7.1+)",
        code: "KEYCODE_SYSTEM_NAVIGATION_DOWN",
    },
    KeyCode {
        label: "System Navigation Left key (Android 7.1+)",
        code: "KEYCODE_SYSTEM_NAVIGATION_LEFT",
    },
    KeyCode {
        label: "System Navigation Right key (Android 7.1+)",
        code: "KEYCODE_SYSTEM_NAVIGATION_RIGHT",
    },
    KeyCode {
        label: "System Navigation Up key (Android 7.1+)",
        code: "KEYCODE_SYSTEM_NAVIGATION_UP",
    },
    KeyCode {
        label: "System Request / Print Screen key (Android 3.0+)",
        code: "KEYCODE_SYSRQ",
    },
    KeyCode {
        label: "T key (Android 1.0+)",
        code: "KEYCODE_T",
    },
    KeyCode {
        label: "TV data service key (Android 5.0+)",
        code: "KEYCODE_TV_DATA_SERVICE",
    },
    KeyCode {
        label: "TV input key (Android 3.0+)",
        code: "KEYCODE_TV_INPUT",
    },
    KeyCode {
        label: "TV key (Android 3.0+)",
        code: "KEYCODE_TV",
    },
    KeyCode {
        label: "TV power key (Android 3.0+)",
        code: "KEYCODE_TV_POWER",
    },
    KeyCode {
        label: "Tab key (Android 1.0+)",
        code: "KEYCODE_TAB",
    },
    KeyCode {
        label: "Teletext key (Android 5.0+)",
        code: "KEYCODE_TV_TELETEXT",
    },
    KeyCode {
        label: "Thumbs down key (Android 10+)",
        code: "KEYCODE_THUMBS_DOWN",
    },
    KeyCode {
        label: "Thumbs up key (Android 10+)",
        code: "KEYCODE_THUMBS_UP",
    },
    KeyCode {
        label: "Timer programming key (Android 5.0+)",
        code: "KEYCODE_TV_TIMER_PROGRAMMING",
    },
    KeyCode {
        label: "Toggle Network key (Android 5.0+)",
        code: "KEYCODE_TV_NETWORK",
    },
    KeyCode {
        label: "Toggle captions key (Android 3.0+)",
        code: "KEYCODE_CAPTIONS",
    },
    KeyCode {
        label: "U key (Android 1.0+)",
        code: "KEYCODE_U",
    },
    KeyCode {
        label: "Unknown key code (Android 1.0+)",
        code: "KEYCODE_UNKNOWN",
    },
    KeyCode {
        label: "V key (Android 1.0+)",
        code: "KEYCODE_V",
    },
    KeyCode {
        label: "VGA #1 key (Android 5.0+)",
        code: "KEYCODE_TV_INPUT_VGA_1",
    },
    KeyCode {
        label: "Video App #1 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_1",
    },
    KeyCode {
        label: "Video App #2 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_2",
    },
    KeyCode {
        label: "Video App #3 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_3",
    },
    KeyCode {
        label: "Video App #4 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_4",
    },
    KeyCode {
        label: "Video App #5 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_5",
    },
    KeyCode {
        label: "Video App #6 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_6",
    },
    KeyCode {
        label: "Video App #7 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_7",
    },
    KeyCode {
        label: "Video App #8 key (Android 13+)",
        code: "KEYCODE_VIDEO_APP_8",
    },
    KeyCode {
        label: "Voice Assist key (Android 5.0+)",
        code: "KEYCODE_VOICE_ASSIST",
    },
    KeyCode {
        label: "Volume Down key (Android 1.0+)",
        code: "KEYCODE_VOLUME_DOWN",
    },
    KeyCode {
        label: "Volume Mute key (Android 3.0+)",
        code: "KEYCODE_VOLUME_MUTE",
    },
    KeyCode {
        label: "Volume Up key (Android 1.0+)",
        code: "KEYCODE_VOLUME_UP",
    },
    KeyCode {
        label: "W key (Android 1.0+)",
        code: "KEYCODE_W",
    },
    KeyCode {
        label: "Wakeup key (Android 4.4W+)",
        code: "KEYCODE_WAKEUP",
    },
    KeyCode {
        label: "Window key (Android 3.0+)",
        code: "KEYCODE_WINDOW",
    },
    KeyCode {
        label: "X Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_X",
    },
    KeyCode {
        label: "X key (Android 1.0+)",
        code: "KEYCODE_X",
    },
    KeyCode {
        label: "Y Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_Y",
    },
    KeyCode {
        label: "Y key (Android 1.0+)",
        code: "KEYCODE_Y",
    },
    KeyCode {
        label: "Yellow 'programmable' key (Android 3.0+)",
        code: "KEYCODE_PROG_YELLOW",
    },
    KeyCode {
        label: "Z Button key (Android 2.3+)",
        code: "KEYCODE_BUTTON_Z",
    },
    KeyCode {
        label: "Z key (Android 1.0+)",
        code: "KEYCODE_Z",
    },
    KeyCode {
        label: "Zoom in key (Android 3.0+)",
        code: "KEYCODE_ZOOM_IN",
    },
    KeyCode {
        label: "Zoom mode key (Android 5.0+)",
        code: "KEYCODE_TV_ZOOM_MODE",
    },
    KeyCode {
        label: "Zoom out key (Android 3.0+)",
        code: "KEYCODE_ZOOM_OUT",
    },
    KeyCode {
        label: "[ key (Android 1.0+)",
        code: "KEYCODE_LEFT_BRACKET",
    },
    KeyCode {
        label: "\\ key (Android 1.0+)",
        code: "KEYCODE_BACKSLASH",
    },
    KeyCode {
        label: "] key (Android 1.0+)",
        code: "KEYCODE_RIGHT_BRACKET",
    },
    KeyCode {
        label: "` key (Android 1.0+)",
        code: "KEYCODE_GRAVE",
    },
];

#[cfg(test)]
mod tests {
    use super::KEY_CODES;
    use std::collections::HashSet;

    #[test]
    fn sorted_by_label() {
        assert!(KEY_CODES.windows(2).all(|w| w[0].label < w[1].label));
    }

    #[test]
    fn codes_are_unique() {
        let codes: HashSet<_> = KEY_CODES.iter().map(|k| k.code).collect();
        assert_eq!(codes.len(), KEY_CODES.len());
    }
}
