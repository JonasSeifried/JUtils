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

        public Dictionary<Hotkeys, string> HotkeyDict { get; private set; } = new Dictionary<Hotkeys, string>();
        public Controller() 
        {
            initHotkeys();
        }
        private void initHotkeys()
        {
            HotkeysManager.SetupSystemHook();
  
            HotkeysManager.AddHotkey(new GlobalHotkey(new Key[2] { Key.F, Key.LeftShift }, () => { MicMute.ToggleMic(); }));
        }

        public List<string> getHotkeysAsStrings()
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
            StringBuilder sb = new StringBuilder();
            foreach(Key key in hotkey.Keys)
            {
                if (sb.Length != 0) sb.Append(" + ");
                sb.Append(key.ToString());
            }
            return sb.ToString();
        }
    }
}
