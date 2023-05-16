using JUtils.model;
using JUtils.model.hotkeys;
using JUtils.Resources;
using System;
using System.Collections.Generic;
using System.Text;
using System.Windows.Input;

namespace JUtils
{
    public sealed class Controller
    {
        public enum Hotkeys
        {
            MicToggle,
        }

        private static Dictionary<Hotkeys, GlobalHotkey> HotkeyDict { get; set; } = new();

        public static Dictionary<Key, string> PrettyKeys { get; private set; } = new();

        public Controller()
        {
            InitHotkeys();
            PopulatePrettyKeys();
        }

        private static readonly Lazy<Controller> Lazy = new Lazy<Controller>(() => new Controller());
        public static Controller Instance => Lazy.Value;


        private void InitHotkeys()
        {
            HotkeysManager.SetupSystemHook();
        }

        private static GlobalHotkey? GetHotkey(Hotkeys hotkey)
        {
            return HotkeyDict.TryGetValue(hotkey, out var hotkey1) ? hotkey1 : null;
        }

        public static void PopulatePrettyKeys()
        {
            foreach (Key key in Enum.GetValues(typeof(Key)))
            {
                string? keyName = KeyNamesGER.ResourceManager.GetString(key.ToString());
                keyName = keyName ?? key.ToString();
                PrettyKeys[key] = keyName;
            }
        }

        public void AddHotkey(Hotkeys hotkey, Key[] keys)
        {
            if (GetHotkey(hotkey) != null)
                HotkeysManager.RemoveHotkey(HotkeyDict[hotkey]);


            Action callBack;
            switch (hotkey)
            {
                case Hotkeys.MicToggle: callBack = MicMute.ToggleMic; break;
                default: return;
            }
            GlobalHotkey ghk = new GlobalHotkey(keys, callBack);
            HotkeysManager.AddHotkey(ghk);
            HotkeyDict[Hotkeys.MicToggle] = ghk;
        }

        public List<string> GetHotkeysAsStrings()
        {
            List<string> s = new List<string>();
            foreach (GlobalHotkey hotkey in HotkeysManager.Hotkeys)
            {
                s.Add(HotkeyToString(hotkey));
            }
            return s;
        }


        public void ShutdownHotkeySystemHook()
        {
            HotkeysManager.ShutdownSystemHook();
        }

        public string HotkeyToString(GlobalHotkey hotkey)
        {
            StringBuilder sb = new StringBuilder();
            foreach (Key key in hotkey.Keys)
            {
                if (sb.Length != 0) sb.Append(" + ");
                sb.Append(key.ToString());
            }
            return sb.ToString();
        }

        public string HotkeyToString(Hotkeys hotkey) => HotkeyToString(HotkeyDict[hotkey]);


        public bool RemoveHotkey(Hotkeys hotkey)
        {
            GlobalHotkey? gHk = GetHotkey(hotkey);
            if (gHk != null && HotkeyDict.Remove(hotkey))
                return HotkeysManager.RemoveHotkey(gHk);
            return false;
        }

        public void RestoreHotkeys()
        {
            foreach (Hotkeys hotkey in Enum.GetValues(typeof(Hotkeys)))
            {

                object keysAsObject = Properties.Settings.Default[hotkey.ToString()];
                Key[] keys = (Key[]) keysAsObject;
                if(keys == null) continue;
                AddHotkey(hotkey, keys);
            }
        }

        public void StoreHotkeys()
        {
            foreach (Hotkeys hotkey in Enum.GetValues(typeof(Hotkeys)))
            {
                if (!HotkeyDict.ContainsKey(hotkey)) continue;
                Properties.Settings.Default[hotkey.ToString()] = HotkeyDict[hotkey].Keys;
            }
            Properties.Settings.Default.Save();
        }
    }
}
