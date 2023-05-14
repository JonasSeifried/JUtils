using JUtils.model;
using JUtils.model.hotkeys;
using JUtils.Resources;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Input;

namespace JUtils
{
    public sealed class Controller
    {
        public enum Hotkeys
        {
            MicToggle,
        }

        public static Dictionary<Hotkeys, GlobalHotkey> HotkeyDict { get; private set; } = new();

        public static Dictionary<Key, string> PrettyKeys { get; private set; } = new();

        public Controller()
        {
            InitHotkeys();
            PopulatePrettyKeys();
        }

        private static readonly Lazy<Controller> lazy = new Lazy<Controller>(() => new Controller());
        public static Controller Instance
        {
            get
            {
                return lazy.Value;
            }
        }


        private void InitHotkeys()
        {
            HotkeysManager.SetupSystemHook();
        }

        private GlobalHotkey? getHotkey(Hotkeys hotkey)
        {
            if (HotkeyDict.ContainsKey(hotkey)) return HotkeyDict[hotkey];
            return null;
        }

        public static void PopulatePrettyKeys()
        {
            foreach (Key key in Enum.GetValues(typeof(Key)))
            {
                string? keyName = KeyNamesGER.ResourceManager.GetString(key.ToString());
                keyName = (keyName == null) ? key.ToString() : keyName;
                PrettyKeys[key] = keyName;
            }
        }

        public void AddHotkey(Hotkeys hotkey, Key[] keys)
        {
            if (getHotkey(hotkey) != null)
                HotkeysManager.RemoveHotkey(HotkeyDict[hotkey]);


            Action callBack;
            switch (hotkey)
            {
                case Hotkeys.MicToggle: callBack = () => MicMute.ToggleMic(); break;
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
            if (hotkey == null) return "null";
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
            GlobalHotkey? ghtk = getHotkey(hotkey);
            if (ghtk != null && HotkeyDict.Remove(hotkey))
                return HotkeysManager.RemoveHotkey(ghtk);
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
                if (!HotkeyDict.ContainsKey(hotkey) || HotkeyDict[hotkey] == null) continue;
                Properties.Settings.Default[hotkey.ToString()] = HotkeyDict[hotkey].Keys;
            }
            Properties.Settings.Default.Save();
        }
    }
}
