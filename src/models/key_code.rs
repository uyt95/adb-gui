//! Android key codes selectable in automations, sorted by label.

pub struct KeyCode {
    pub label: &'static str,
    pub code: &'static str,
}

pub const KEY_CODES: &[KeyCode] = &[
    KeyCode {
        label: "# key",
        code: "KEYCODE_POUND",
    },
    KeyCode {
        label: "* key",
        code: "KEYCODE_STAR",
    },
    KeyCode {
        label: "+ key",
        code: "KEYCODE_PLUS",
    },
    KeyCode {
        label: ", key",
        code: "KEYCODE_COMMA",
    },
    KeyCode {
        label: "- key",
        code: "KEYCODE_MINUS",
    },
    KeyCode {
        label: ". key",
        code: "KEYCODE_PERIOD",
    },
    KeyCode {
        label: "/ key",
        code: "KEYCODE_SLASH",
    },
    KeyCode {
        label: "0 key",
        code: "KEYCODE_0",
    },
    KeyCode {
        label: "1 key",
        code: "KEYCODE_1",
    },
    KeyCode {
        label: "2 key",
        code: "KEYCODE_2",
    },
    KeyCode {
        label: "3 key",
        code: "KEYCODE_3",
    },
    KeyCode {
        label: "3D Mode key",
        code: "KEYCODE_3D_MODE",
    },
    KeyCode {
        label: "4 key",
        code: "KEYCODE_4",
    },
    KeyCode {
        label: "5 key",
        code: "KEYCODE_5",
    },
    KeyCode {
        label: "6 key",
        code: "KEYCODE_6",
    },
    KeyCode {
        label: "7 key",
        code: "KEYCODE_7",
    },
    KeyCode {
        label: "8 key",
        code: "KEYCODE_8",
    },
    KeyCode {
        label: "9 key",
        code: "KEYCODE_9",
    },
    KeyCode {
        label: "; key",
        code: "KEYCODE_SEMICOLON",
    },
    KeyCode {
        label: "= key",
        code: "KEYCODE_EQUALS",
    },
    KeyCode {
        label: "@ key",
        code: "KEYCODE_AT",
    },
    KeyCode {
        label: "A Button key",
        code: "KEYCODE_BUTTON_A",
    },
    KeyCode {
        label: "A key",
        code: "KEYCODE_A",
    },
    KeyCode {
        label: "A/V Receiver input key",
        code: "KEYCODE_AVR_INPUT",
    },
    KeyCode {
        label: "A/V Receiver power key",
        code: "KEYCODE_AVR_POWER",
    },
    KeyCode {
        label: "Analog Terrestrial key",
        code: "KEYCODE_TV_TERRESTRIAL_ANALOG",
    },
    KeyCode {
        label: "Antenna/Cable key",
        code: "KEYCODE_TV_ANTENNA_CABLE",
    },
    KeyCode {
        label: "App switch key",
        code: "KEYCODE_APP_SWITCH",
    },
    KeyCode {
        label: "Assist key",
        code: "KEYCODE_ASSIST",
    },
    KeyCode {
        label: "Audio Track key",
        code: "KEYCODE_MEDIA_AUDIO_TRACK",
    },
    KeyCode {
        label: "Audio description key",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION",
    },
    KeyCode {
        label: "Audio description mixing volume down key",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN",
    },
    KeyCode {
        label: "Audio description mixing volume up key",
        code: "KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP",
    },
    KeyCode {
        label: "B Button key",
        code: "KEYCODE_BUTTON_B",
    },
    KeyCode {
        label: "B key",
        code: "KEYCODE_B",
    },
    KeyCode {
        label: "BS key",
        code: "KEYCODE_TV_SATELLITE_BS",
    },
    KeyCode {
        label: "BS/CS key",
        code: "KEYCODE_TV_SATELLITE_SERVICE",
    },
    KeyCode {
        label: "Back key",
        code: "KEYCODE_BACK",
    },
    KeyCode {
        label: "Backspace key",
        code: "KEYCODE_DEL",
    },
    KeyCode {
        label: "Blue 'programmable' key",
        code: "KEYCODE_PROG_BLUE",
    },
    KeyCode {
        label: "Bookmark key",
        code: "KEYCODE_BOOKMARK",
    },
    KeyCode {
        label: "Break / Pause key",
        code: "KEYCODE_BREAK",
    },
    KeyCode {
        label: "Brightness Down key",
        code: "KEYCODE_BRIGHTNESS_DOWN",
    },
    KeyCode {
        label: "Brightness Up key",
        code: "KEYCODE_BRIGHTNESS_UP",
    },
    KeyCode {
        label: "C Button key",
        code: "KEYCODE_BUTTON_C",
    },
    KeyCode {
        label: "C key",
        code: "KEYCODE_C",
    },
    KeyCode {
        label: "CS key",
        code: "KEYCODE_TV_SATELLITE_CS",
    },
    KeyCode {
        label: "Calculator special function key",
        code: "KEYCODE_CALCULATOR",
    },
    KeyCode {
        label: "Calendar special function key",
        code: "KEYCODE_CALENDAR",
    },
    KeyCode {
        label: "Call key",
        code: "KEYCODE_CALL",
    },
    KeyCode {
        label: "Camera Focus key",
        code: "KEYCODE_FOCUS",
    },
    KeyCode {
        label: "Camera key",
        code: "KEYCODE_CAMERA",
    },
    KeyCode {
        label: "Caps Lock key",
        code: "KEYCODE_CAPS_LOCK",
    },
    KeyCode {
        label: "Channel down key",
        code: "KEYCODE_CHANNEL_DOWN",
    },
    KeyCode {
        label: "Channel up key",
        code: "KEYCODE_CHANNEL_UP",
    },
    KeyCode {
        label: "Clear key",
        code: "KEYCODE_CLEAR",
    },
    KeyCode {
        label: "Close media key",
        code: "KEYCODE_MEDIA_CLOSE",
    },
    KeyCode {
        label: "Component #1 key",
        code: "KEYCODE_TV_INPUT_COMPONENT_1",
    },
    KeyCode {
        label: "Component #2 key",
        code: "KEYCODE_TV_INPUT_COMPONENT_2",
    },
    KeyCode {
        label: "Composite #1 key",
        code: "KEYCODE_TV_INPUT_COMPOSITE_1",
    },
    KeyCode {
        label: "Composite #2 key",
        code: "KEYCODE_TV_INPUT_COMPOSITE_2",
    },
    KeyCode {
        label: "Contacts special function key",
        code: "KEYCODE_CONTACTS",
    },
    KeyCode {
        label: "Contents menu key",
        code: "KEYCODE_TV_CONTENTS_MENU",
    },
    KeyCode {
        label: "Copy key",
        code: "KEYCODE_COPY",
    },
    KeyCode {
        label: "Cut key",
        code: "KEYCODE_CUT",
    },
    KeyCode {
        label: "D key",
        code: "KEYCODE_D",
    },
    KeyCode {
        label: "DVR key",
        code: "KEYCODE_DVR",
    },
    KeyCode {
        label: "Digital Terrestrial key",
        code: "KEYCODE_TV_TERRESTRIAL_DIGITAL",
    },
    KeyCode {
        label: "Directional Pad Center key",
        code: "KEYCODE_DPAD_CENTER",
    },
    KeyCode {
        label: "Directional Pad Down key",
        code: "KEYCODE_DPAD_DOWN",
    },
    KeyCode {
        label: "Directional Pad Down-Left key",
        code: "KEYCODE_DPAD_DOWN_LEFT",
    },
    KeyCode {
        label: "Directional Pad Down-Right key",
        code: "KEYCODE_DPAD_DOWN_RIGHT",
    },
    KeyCode {
        label: "Directional Pad Left key",
        code: "KEYCODE_DPAD_LEFT",
    },
    KeyCode {
        label: "Directional Pad Right key",
        code: "KEYCODE_DPAD_RIGHT",
    },
    KeyCode {
        label: "Directional Pad Up key",
        code: "KEYCODE_DPAD_UP",
    },
    KeyCode {
        label: "Directional Pad Up-Left key",
        code: "KEYCODE_DPAD_UP_LEFT",
    },
    KeyCode {
        label: "Directional Pad Up-Right key",
        code: "KEYCODE_DPAD_UP_RIGHT",
    },
    KeyCode {
        label: "E key",
        code: "KEYCODE_E",
    },
    KeyCode {
        label: "Eject media key",
        code: "KEYCODE_MEDIA_EJECT",
    },
    KeyCode {
        label: "End Call key",
        code: "KEYCODE_ENDCALL",
    },
    KeyCode {
        label: "End Movement key",
        code: "KEYCODE_MOVE_END",
    },
    KeyCode {
        label: "Enter key",
        code: "KEYCODE_ENTER",
    },
    KeyCode {
        label: "Envelope special function key",
        code: "KEYCODE_ENVELOPE",
    },
    KeyCode {
        label: "Escape key",
        code: "KEYCODE_ESCAPE",
    },
    KeyCode {
        label: "Explorer special function key",
        code: "KEYCODE_EXPLORER",
    },
    KeyCode {
        label: "F key",
        code: "KEYCODE_F",
    },
    KeyCode {
        label: "F1 key",
        code: "KEYCODE_F1",
    },
    KeyCode {
        label: "F10 key",
        code: "KEYCODE_F10",
    },
    KeyCode {
        label: "F11 key",
        code: "KEYCODE_F11",
    },
    KeyCode {
        label: "F12 key",
        code: "KEYCODE_F12",
    },
    KeyCode {
        label: "F2 key",
        code: "KEYCODE_F2",
    },
    KeyCode {
        label: "F3 key",
        code: "KEYCODE_F3",
    },
    KeyCode {
        label: "F4 key",
        code: "KEYCODE_F4",
    },
    KeyCode {
        label: "F5 key",
        code: "KEYCODE_F5",
    },
    KeyCode {
        label: "F6 key",
        code: "KEYCODE_F6",
    },
    KeyCode {
        label: "F7 key",
        code: "KEYCODE_F7",
    },
    KeyCode {
        label: "F8 key",
        code: "KEYCODE_F8",
    },
    KeyCode {
        label: "F9 key",
        code: "KEYCODE_F9",
    },
    KeyCode {
        label: "Fast Forward media key",
        code: "KEYCODE_MEDIA_FAST_FORWARD",
    },
    KeyCode {
        label: "Forward Delete key",
        code: "KEYCODE_FORWARD_DEL",
    },
    KeyCode {
        label: "Forward key",
        code: "KEYCODE_FORWARD",
    },
    KeyCode {
        label: "Function key",
        code: "KEYCODE_FUNCTION",
    },
    KeyCode {
        label: "G key",
        code: "KEYCODE_G",
    },
    KeyCode {
        label: "Generic Game Pad Button #1",
        code: "KEYCODE_BUTTON_1",
    },
    KeyCode {
        label: "Generic Game Pad Button #10",
        code: "KEYCODE_BUTTON_10",
    },
    KeyCode {
        label: "Generic Game Pad Button #11",
        code: "KEYCODE_BUTTON_11",
    },
    KeyCode {
        label: "Generic Game Pad Button #12",
        code: "KEYCODE_BUTTON_12",
    },
    KeyCode {
        label: "Generic Game Pad Button #13",
        code: "KEYCODE_BUTTON_13",
    },
    KeyCode {
        label: "Generic Game Pad Button #14",
        code: "KEYCODE_BUTTON_14",
    },
    KeyCode {
        label: "Generic Game Pad Button #15",
        code: "KEYCODE_BUTTON_15",
    },
    KeyCode {
        label: "Generic Game Pad Button #16",
        code: "KEYCODE_BUTTON_16",
    },
    KeyCode {
        label: "Generic Game Pad Button #2",
        code: "KEYCODE_BUTTON_2",
    },
    KeyCode {
        label: "Generic Game Pad Button #3",
        code: "KEYCODE_BUTTON_3",
    },
    KeyCode {
        label: "Generic Game Pad Button #4",
        code: "KEYCODE_BUTTON_4",
    },
    KeyCode {
        label: "Generic Game Pad Button #5",
        code: "KEYCODE_BUTTON_5",
    },
    KeyCode {
        label: "Generic Game Pad Button #6",
        code: "KEYCODE_BUTTON_6",
    },
    KeyCode {
        label: "Generic Game Pad Button #7",
        code: "KEYCODE_BUTTON_7",
    },
    KeyCode {
        label: "Generic Game Pad Button #8",
        code: "KEYCODE_BUTTON_8",
    },
    KeyCode {
        label: "Generic Game Pad Button #9",
        code: "KEYCODE_BUTTON_9",
    },
    KeyCode {
        label: "Generic stem key 1 for Wear",
        code: "KEYCODE_STEM_1",
    },
    KeyCode {
        label: "Generic stem key 2 for Wear",
        code: "KEYCODE_STEM_2",
    },
    KeyCode {
        label: "Generic stem key 3 for Wear",
        code: "KEYCODE_STEM_3",
    },
    KeyCode {
        label: "Green 'programmable' key",
        code: "KEYCODE_PROG_GREEN",
    },
    KeyCode {
        label: "Guide key",
        code: "KEYCODE_GUIDE",
    },
    KeyCode {
        label: "H key",
        code: "KEYCODE_H",
    },
    KeyCode {
        label: "HDMI #1 key",
        code: "KEYCODE_TV_INPUT_HDMI_1",
    },
    KeyCode {
        label: "HDMI #2 key",
        code: "KEYCODE_TV_INPUT_HDMI_2",
    },
    KeyCode {
        label: "HDMI #3 key",
        code: "KEYCODE_TV_INPUT_HDMI_3",
    },
    KeyCode {
        label: "HDMI #4 key",
        code: "KEYCODE_TV_INPUT_HDMI_4",
    },
    KeyCode {
        label: "Headset Hook key",
        code: "KEYCODE_HEADSETHOOK",
    },
    KeyCode {
        label: "Help key",
        code: "KEYCODE_HELP",
    },
    KeyCode {
        label: "Home Movement key",
        code: "KEYCODE_MOVE_HOME",
    },
    KeyCode {
        label: "Home key",
        code: "KEYCODE_HOME",
    },
    KeyCode {
        label: "I key",
        code: "KEYCODE_I",
    },
    KeyCode {
        label: "Info key",
        code: "KEYCODE_INFO",
    },
    KeyCode {
        label: "Insert key",
        code: "KEYCODE_INSERT",
    },
    KeyCode {
        label: "J key",
        code: "KEYCODE_J",
    },
    KeyCode {
        label: "Japanese Ro key",
        code: "KEYCODE_RO",
    },
    KeyCode {
        label: "Japanese Yen key",
        code: "KEYCODE_YEN",
    },
    KeyCode {
        label: "Japanese alphanumeric key",
        code: "KEYCODE_EISU",
    },
    KeyCode {
        label: "Japanese conversion key",
        code: "KEYCODE_HENKAN",
    },
    KeyCode {
        label: "Japanese full-width / half-width key",
        code: "KEYCODE_ZENKAKU_HANKAKU",
    },
    KeyCode {
        label: "Japanese kana key",
        code: "KEYCODE_KANA",
    },
    KeyCode {
        label: "Japanese katakana / hiragana key",
        code: "KEYCODE_KATAKANA_HIRAGANA",
    },
    KeyCode {
        label: "Japanese non-conversion key",
        code: "KEYCODE_MUHENKAN",
    },
    KeyCode {
        label: "K key",
        code: "KEYCODE_K",
    },
    KeyCode {
        label: "L key",
        code: "KEYCODE_L",
    },
    KeyCode {
        label: "L1 Button key",
        code: "KEYCODE_BUTTON_L1",
    },
    KeyCode {
        label: "L2 Button key",
        code: "KEYCODE_BUTTON_L2",
    },
    KeyCode {
        label: "Language Switch key",
        code: "KEYCODE_LANGUAGE_SWITCH",
    },
    KeyCode {
        label: "Last Channel key",
        code: "KEYCODE_LAST_CHANNEL",
    },
    KeyCode {
        label: "Left Alt modifier key",
        code: "KEYCODE_ALT_LEFT",
    },
    KeyCode {
        label: "Left Control modifier key",
        code: "KEYCODE_CTRL_LEFT",
    },
    KeyCode {
        label: "Left Meta modifier key",
        code: "KEYCODE_META_LEFT",
    },
    KeyCode {
        label: "Left Shift modifier key",
        code: "KEYCODE_SHIFT_LEFT",
    },
    KeyCode {
        label: "Left Thumb Button key",
        code: "KEYCODE_BUTTON_THUMBL",
    },
    KeyCode {
        label: "M key",
        code: "KEYCODE_M",
    },
    KeyCode {
        label: "Manner Mode key",
        code: "KEYCODE_MANNER_MODE",
    },
    KeyCode {
        label: "Media Top Menu key",
        code: "KEYCODE_MEDIA_TOP_MENU",
    },
    KeyCode {
        label: "Media context menu key",
        code: "KEYCODE_TV_MEDIA_CONTEXT_MENU",
    },
    KeyCode {
        label: "Menu key",
        code: "KEYCODE_MENU",
    },
    KeyCode {
        label: "Mode Button key",
        code: "KEYCODE_BUTTON_MODE",
    },
    KeyCode {
        label: "Music special function key",
        code: "KEYCODE_MUSIC",
    },
    KeyCode {
        label: "Mute key",
        code: "KEYCODE_MUTE",
    },
    KeyCode {
        label: "N key",
        code: "KEYCODE_N",
    },
    KeyCode {
        label: "Navigate in key",
        code: "KEYCODE_NAVIGATE_IN",
    },
    KeyCode {
        label: "Navigate out key",
        code: "KEYCODE_NAVIGATE_OUT",
    },
    KeyCode {
        label: "Navigate to next key",
        code: "KEYCODE_NAVIGATE_NEXT",
    },
    KeyCode {
        label: "Navigate to previous key",
        code: "KEYCODE_NAVIGATE_PREVIOUS",
    },
    KeyCode {
        label: "Notification key",
        code: "KEYCODE_NOTIFICATION",
    },
    KeyCode {
        label: "Num Lock key",
        code: "KEYCODE_NUM_LOCK",
    },
    KeyCode {
        label: "Number entry key",
        code: "KEYCODE_TV_NUMBER_ENTRY",
    },
    KeyCode {
        label: "Number modifier key",
        code: "KEYCODE_NUM",
    },
    KeyCode {
        label: "Numeric keypad ( key",
        code: "KEYCODE_NUMPAD_LEFT_PAREN",
    },
    KeyCode {
        label: "Numeric keypad ) key",
        code: "KEYCODE_NUMPAD_RIGHT_PAREN",
    },
    KeyCode {
        label: "Numeric keypad * key",
        code: "KEYCODE_NUMPAD_MULTIPLY",
    },
    KeyCode {
        label: "Numeric keypad + key",
        code: "KEYCODE_NUMPAD_ADD",
    },
    KeyCode {
        label: "Numeric keypad , key",
        code: "KEYCODE_NUMPAD_COMMA",
    },
    KeyCode {
        label: "Numeric keypad - key",
        code: "KEYCODE_NUMPAD_SUBTRACT",
    },
    KeyCode {
        label: "Numeric keypad . key",
        code: "KEYCODE_NUMPAD_DOT",
    },
    KeyCode {
        label: "Numeric keypad / key",
        code: "KEYCODE_NUMPAD_DIVIDE",
    },
    KeyCode {
        label: "Numeric keypad 0 key",
        code: "KEYCODE_NUMPAD_0",
    },
    KeyCode {
        label: "Numeric keypad 1 key",
        code: "KEYCODE_NUMPAD_1",
    },
    KeyCode {
        label: "Numeric keypad 2 key",
        code: "KEYCODE_NUMPAD_2",
    },
    KeyCode {
        label: "Numeric keypad 3 key",
        code: "KEYCODE_NUMPAD_3",
    },
    KeyCode {
        label: "Numeric keypad 4 key",
        code: "KEYCODE_NUMPAD_4",
    },
    KeyCode {
        label: "Numeric keypad 5 key",
        code: "KEYCODE_NUMPAD_5",
    },
    KeyCode {
        label: "Numeric keypad 6 key",
        code: "KEYCODE_NUMPAD_6",
    },
    KeyCode {
        label: "Numeric keypad 7 key",
        code: "KEYCODE_NUMPAD_7",
    },
    KeyCode {
        label: "Numeric keypad 8 key",
        code: "KEYCODE_NUMPAD_8",
    },
    KeyCode {
        label: "Numeric keypad 9 key",
        code: "KEYCODE_NUMPAD_9",
    },
    KeyCode {
        label: "Numeric keypad = key",
        code: "KEYCODE_NUMPAD_EQUALS",
    },
    KeyCode {
        label: "Numeric keypad Enter key",
        code: "KEYCODE_NUMPAD_ENTER",
    },
    KeyCode {
        label: "O key",
        code: "KEYCODE_O",
    },
    KeyCode {
        label: "P key",
        code: "KEYCODE_P",
    },
    KeyCode {
        label: "Page Down key",
        code: "KEYCODE_PAGE_DOWN",
    },
    KeyCode {
        label: "Page Up key",
        code: "KEYCODE_PAGE_UP",
    },
    KeyCode {
        label: "Pairing key",
        code: "KEYCODE_PAIRING",
    },
    KeyCode {
        label: "Paste key",
        code: "KEYCODE_PASTE",
    },
    KeyCode {
        label: "Pause media key",
        code: "KEYCODE_MEDIA_PAUSE",
    },
    KeyCode {
        label: "Picture Symbols modifier key",
        code: "KEYCODE_PICTSYMBOLS",
    },
    KeyCode {
        label: "Play Next media key",
        code: "KEYCODE_MEDIA_NEXT",
    },
    KeyCode {
        label: "Play Previous media key",
        code: "KEYCODE_MEDIA_PREVIOUS",
    },
    KeyCode {
        label: "Play media key",
        code: "KEYCODE_MEDIA_PLAY",
    },
    KeyCode {
        label: "Play/Pause media key",
        code: "KEYCODE_MEDIA_PLAY_PAUSE",
    },
    KeyCode {
        label: "Power key",
        code: "KEYCODE_POWER",
    },
    KeyCode {
        label: "Primary stem key for Wear",
        code: "KEYCODE_STEM_PRIMARY",
    },
    KeyCode {
        label: "Profile Switch key",
        code: "KEYCODE_PROFILE_SWITCH",
    },
    KeyCode {
        label: "Q key",
        code: "KEYCODE_Q",
    },
    KeyCode {
        label: "R key",
        code: "KEYCODE_R",
    },
    KeyCode {
        label: "R1 Button key",
        code: "KEYCODE_BUTTON_R1",
    },
    KeyCode {
        label: "R2 Button key",
        code: "KEYCODE_BUTTON_R2",
    },
    KeyCode {
        label: "Radio key",
        code: "KEYCODE_TV_RADIO_SERVICE",
    },
    KeyCode {
        label: "Record media key",
        code: "KEYCODE_MEDIA_RECORD",
    },
    KeyCode {
        label: "Red 'programmable' key",
        code: "KEYCODE_PROG_RED",
    },
    KeyCode {
        label: "Refresh key",
        code: "KEYCODE_REFRESH",
    },
    KeyCode {
        label: "Rewind media key",
        code: "KEYCODE_MEDIA_REWIND",
    },
    KeyCode {
        label: "Right Alt modifier key",
        code: "KEYCODE_ALT_RIGHT",
    },
    KeyCode {
        label: "Right Control modifier key",
        code: "KEYCODE_CTRL_RIGHT",
    },
    KeyCode {
        label: "Right Meta modifier key",
        code: "KEYCODE_META_RIGHT",
    },
    KeyCode {
        label: "Right Shift modifier key",
        code: "KEYCODE_SHIFT_RIGHT",
    },
    KeyCode {
        label: "Right Thumb Button key",
        code: "KEYCODE_BUTTON_THUMBR",
    },
    KeyCode {
        label: "S key",
        code: "KEYCODE_S",
    },
    KeyCode {
        label: "Satellite key",
        code: "KEYCODE_TV_SATELLITE",
    },
    KeyCode {
        label: "Scroll Lock key",
        code: "KEYCODE_SCROLL_LOCK",
    },
    KeyCode {
        label: "Search key",
        code: "KEYCODE_SEARCH",
    },
    KeyCode {
        label: "Select Button key",
        code: "KEYCODE_BUTTON_SELECT",
    },
    KeyCode {
        label: "Set-top-box input key",
        code: "KEYCODE_STB_INPUT",
    },
    KeyCode {
        label: "Set-top-box power key",
        code: "KEYCODE_STB_POWER",
    },
    KeyCode {
        label: "Settings key",
        code: "KEYCODE_SETTINGS",
    },
    KeyCode {
        label: "Show all apps",
        code: "KEYCODE_ALL_APPS",
    },
    KeyCode {
        label: "Skip backward media key",
        code: "KEYCODE_MEDIA_SKIP_BACKWARD",
    },
    KeyCode {
        label: "Skip forward media key",
        code: "KEYCODE_MEDIA_SKIP_FORWARD",
    },
    KeyCode {
        label: "Sleep key",
        code: "KEYCODE_SLEEP",
    },
    KeyCode {
        label: "Soft Left key",
        code: "KEYCODE_SOFT_LEFT",
    },
    KeyCode {
        label: "Soft Right key",
        code: "KEYCODE_SOFT_RIGHT",
    },
    KeyCode {
        label: "Soft Sleep key",
        code: "KEYCODE_SOFT_SLEEP",
    },
    KeyCode {
        label: "Space key",
        code: "KEYCODE_SPACE",
    },
    KeyCode {
        label: "Start Button key",
        code: "KEYCODE_BUTTON_START",
    },
    KeyCode {
        label: "Step backward media key",
        code: "KEYCODE_MEDIA_STEP_BACKWARD",
    },
    KeyCode {
        label: "Step forward media key",
        code: "KEYCODE_MEDIA_STEP_FORWARD",
    },
    KeyCode {
        label: "Stop media key",
        code: "KEYCODE_MEDIA_STOP",
    },
    KeyCode {
        label: "Switch Charset modifier key",
        code: "KEYCODE_SWITCH_CHARSET",
    },
    KeyCode {
        label: "Symbol modifier key",
        code: "KEYCODE_SYM",
    },
    KeyCode {
        label: "System Navigation Down key",
        code: "KEYCODE_SYSTEM_NAVIGATION_DOWN",
    },
    KeyCode {
        label: "System Navigation Left key",
        code: "KEYCODE_SYSTEM_NAVIGATION_LEFT",
    },
    KeyCode {
        label: "System Navigation Right key",
        code: "KEYCODE_SYSTEM_NAVIGATION_RIGHT",
    },
    KeyCode {
        label: "System Navigation Up key",
        code: "KEYCODE_SYSTEM_NAVIGATION_UP",
    },
    KeyCode {
        label: "System Request / Print Screen key",
        code: "KEYCODE_SYSRQ",
    },
    KeyCode {
        label: "T key",
        code: "KEYCODE_T",
    },
    KeyCode {
        label: "TV data service key",
        code: "KEYCODE_TV_DATA_SERVICE",
    },
    KeyCode {
        label: "TV input key",
        code: "KEYCODE_TV_INPUT",
    },
    KeyCode {
        label: "TV key",
        code: "KEYCODE_TV",
    },
    KeyCode {
        label: "TV power key",
        code: "KEYCODE_TV_POWER",
    },
    KeyCode {
        label: "Tab key",
        code: "KEYCODE_TAB",
    },
    KeyCode {
        label: "Teletext key",
        code: "KEYCODE_TV_TELETEXT",
    },
    KeyCode {
        label: "Thumbs down key",
        code: "KEYCODE_THUMBS_DOWN",
    },
    KeyCode {
        label: "Thumbs up key",
        code: "KEYCODE_THUMBS_UP",
    },
    KeyCode {
        label: "Timer programming key",
        code: "KEYCODE_TV_TIMER_PROGRAMMING",
    },
    KeyCode {
        label: "Toggle Network key",
        code: "KEYCODE_TV_NETWORK",
    },
    KeyCode {
        label: "Toggle captions key",
        code: "KEYCODE_CAPTIONS",
    },
    KeyCode {
        label: "U key",
        code: "KEYCODE_U",
    },
    KeyCode {
        label: "Unknown key code",
        code: "KEYCODE_UNKNOWN",
    },
    KeyCode {
        label: "V key",
        code: "KEYCODE_V",
    },
    KeyCode {
        label: "VGA #1 key",
        code: "KEYCODE_TV_INPUT_VGA_1",
    },
    KeyCode {
        label: "Voice Assist key",
        code: "KEYCODE_VOICE_ASSIST",
    },
    KeyCode {
        label: "Volume Down key",
        code: "KEYCODE_VOLUME_DOWN",
    },
    KeyCode {
        label: "Volume Mute key",
        code: "KEYCODE_VOLUME_MUTE",
    },
    KeyCode {
        label: "Volume Up key",
        code: "KEYCODE_VOLUME_UP",
    },
    KeyCode {
        label: "W key",
        code: "KEYCODE_W",
    },
    KeyCode {
        label: "Wakeup key",
        code: "KEYCODE_WAKEUP",
    },
    KeyCode {
        label: "Window key",
        code: "KEYCODE_WINDOW",
    },
    KeyCode {
        label: "X Button key",
        code: "KEYCODE_BUTTON_X",
    },
    KeyCode {
        label: "X key",
        code: "KEYCODE_X",
    },
    KeyCode {
        label: "Y Button key",
        code: "KEYCODE_BUTTON_Y",
    },
    KeyCode {
        label: "Y key",
        code: "KEYCODE_Y",
    },
    KeyCode {
        label: "Yellow 'programmable' key",
        code: "KEYCODE_PROG_YELLOW",
    },
    KeyCode {
        label: "Z Button key",
        code: "KEYCODE_BUTTON_Z",
    },
    KeyCode {
        label: "Z key",
        code: "KEYCODE_Z",
    },
    KeyCode {
        label: "Zoom in key",
        code: "KEYCODE_ZOOM_IN",
    },
    KeyCode {
        label: "Zoom mode key",
        code: "KEYCODE_TV_ZOOM_MODE",
    },
    KeyCode {
        label: "Zoom out key",
        code: "KEYCODE_ZOOM_OUT",
    },
    KeyCode {
        label: "[ key",
        code: "KEYCODE_LEFT_BRACKET",
    },
    KeyCode {
        label: "' key",
        code: "KEYCODE_APOSTROPHE",
    },
    KeyCode {
        label: "\\ key",
        code: "KEYCODE_BACKSLASH",
    },
    KeyCode {
        label: "] key",
        code: "KEYCODE_RIGHT_BRACKET",
    },
    KeyCode {
        label: "` key",
        code: "KEYCODE_GRAVE",
    },
];
