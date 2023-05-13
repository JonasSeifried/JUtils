using JUtils.model;
using JUtils.model.hotkeys;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Input;

namespace JUtils
{
    public class Controller
    {
        public enum Hotkeys
        {
            ToggleMic,
        }

        public Dictionary<Hotkeys, GlobalHotkey> HotkeyDict { get; private set; } = new ();
        public Controller() 
        {
            initHotkeys();
        }
        private void initHotkeys()
        {
            HotkeysManager.SetupSystemHook();
        }

        private GlobalHotkey? getHotkey(Hotkeys hotkey)
        {
            if (HotkeyDict.ContainsKey(hotkey)) return HotkeyDict[hotkey];
            return null;
        }

        public void AddHotkey(Hotkeys hotkey, Key[] keys)
        {
            if(getHotkey(hotkey) != null) 
                HotkeysManager.RemoveHotkey(HotkeyDict[hotkey]);


            Action callBack;
            switch (hotkey)
            {
                case Hotkeys.ToggleMic: callBack = () => MicMute.ToggleMic(); break;
                default: return;
            }
            GlobalHotkey ghk = new GlobalHotkey(keys, callBack);
            HotkeysManager.AddHotkey(ghk);
            HotkeyDict[Hotkeys.ToggleMic] = ghk;
        }

        public List<string> GetHotkeysAsStrings()
        {
            List<string> s = new List<string>();
            foreach (GlobalHotkey hotkey in  HotkeysManager.Hotkeys)
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
            foreach(Key key in hotkey.Keys)
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
    }
}
