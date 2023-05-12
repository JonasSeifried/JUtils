using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Windows.Input;

namespace JUtils.model.hotkeys
{
    /// <summary>
    /// A class for adding/removing global hotkeys to and from your application, 
    /// meaning these hotkeys can be run even if your application isn't focused.
    /// </summary>
    public static class HotkeysManager
    {
        // Events

        public delegate void HotkeyEvent(GlobalHotkey hotkey);

        /// <summary>
        /// Fired when a hotkey is fired (duh lol).
        /// </summary>
        public static event HotkeyEvent HotkeyFired;

        // Callbacks

        private delegate IntPtr LowLevelKeyboardProc(int nCode, IntPtr wParam, IntPtr lParam);
        private static LowLevelKeyboardProc LowLevelProc = HookCallback;

        // All of the Hotkeys
        public static List<GlobalHotkey> Hotkeys { get; private set; }

        // The build in proc ID for telling windows to hook onto the
        // low level keyboard events with the SetWindowsHookEx function
        private const int WH_KEYBOARD_LL = 13;

        // The system hook ID (for storing this application's hook)
        private static IntPtr HookID = IntPtr.Zero;

        /// <summary>
        /// States whether the system keyboard event hook is setup. 
        /// </summary>
        public static bool IsHookSetup { get; private set; }

        /// <summary>
        /// States whether hotkeys require modifier keys to be scanned (and therefore
        /// have a chance for their callback method to be called). If this is disabled,
        /// the hotkeys will be checked in every key stroke/event, so pressing just 'A' could 
        /// fire a hotkey if there is one with no modifiers and just it's key set to 'A'. 
        /// <para>
        /// If enabled, a modifier key is required on hotkeys. if the hotkey
        /// has no modifiers, then it simply wont be scanned at all.
        /// </para>
        /// </summary>
        public static bool RequiresModifierKey { get; set; }

        static HotkeysManager()
        {
            Hotkeys = new List<GlobalHotkey>();
            RequiresModifierKey = true;
        }

        /// <summary>
        /// Hooks/Sets up this application for receiving keydown callbacks
        /// </summary>
        public static void SetupSystemHook(bool requireMod = true)
        {
            HookID = SetHook(LowLevelProc);
            IsHookSetup = true;
            RequiresModifierKey = requireMod;
        }

        /// <summary>
        /// Unhooks this application, stopping it from receiving keydown callbacks
        /// </summary>
        public static void ShutdownSystemHook()
        {
            UnhookWindowsHookEx(HookID);
            IsHookSetup = false;
        }

        /// <summary>
        /// Adds a hotkey to the hotkeys list.
        /// </summary>
        public static void AddHotkey(GlobalHotkey hotkey)
        {
            Hotkeys.Add(hotkey);
        }

        /// <summary>
        /// Removes a hotkey from the hotkeys list.
        /// </summary>
        /// <param name="hotkey"></param>
        public static void RemoveHotkey(GlobalHotkey hotkey)
        {
            Hotkeys.Remove(hotkey);
        }

        /// <summary>
        /// Checks if there are any modifiers are pressed. If so, it checks through every
        /// Hotkey and matches their Modifier/Key. If they both match, and the hotkey allows
        /// the callback method to be called, it is called.
        /// </summary>
        private static void CheckHotkeys()  
        {
            if (RequiresModifierKey && Keyboard.Modifiers == ModifierKeys.None) return;
            foreach (GlobalHotkey hotkey in Hotkeys)
            {
                if (Keyboard.Modifiers == hotkey.Modifier && !hotkey.pressed)
                {
                    foreach (Key key in hotkey.Keys)
                    {
                        if (!Keyboard.IsKeyDown(key)) return;
                    }
                    if (hotkey.CanExecute)
                    {
                        hotkey.pressed = true;
                        hotkey.Callback?.Invoke();
                        HotkeyFired?.Invoke(hotkey);
                        return;
                    }
                } 
                else if (hotkey.pressed)
                {
                    foreach (Key key in hotkey.Keys)
                    {
                        if (Keyboard.IsKeyUp(key))
                        {
                            hotkey.pressed = false;
                            return;
                        }
                    }
                }

            }
        }

        /// <summary>
        /// Finds and returns all hotkeys in the hotkeys list that have matching modifiers and keys given
        /// </summary>
        /// <param name="modifier"></param>
        /// <param name="key"></param>
        /// <param name="callbackMethod">If this is not null, the callback method will be checked</param>
        /// <returns></returns>
        public static List<GlobalHotkey> FindHotkeys(ModifierKeys modifier, Key[] keys)
        {
            List<GlobalHotkey> hotkeys = new List<GlobalHotkey>();
            List<Key> keyList = new List<Key>(keys);
            foreach (GlobalHotkey hotkey in Hotkeys)
                if (hotkey.Modifier == modifier)
                {
                    bool allKeyPressed = true;
                    foreach (Key key in hotkey.Keys) 
                        if (!keyList.Contains(key)) 
                            allKeyPressed = false;

                    if (allKeyPressed) hotkeys.Add(hotkey);
                }
                    

            return hotkeys;
        }

        /// <summary>
        /// Creates and adds a new hotkey to the hotkeys list.
        /// </summary>
        /// <param name="modifier">The modifier key. ALT Does not work.</param>
        /// <param name="key"></param>
        /// <param name="callbackMethod"></param>
        /// <param name="canExecute"></param>
        public static void AddHotkey(ModifierKeys modifier, Key[] keys, Action callbackMethod, bool canExecute = true)
        {
            AddHotkey(new GlobalHotkey(modifier, keys, callbackMethod, canExecute));
        }

        /// <summary>
        /// Removes a or all hotkey from the hotkeys list (depending on 
        /// <paramref name="removeAllOccourances"/>) by going through every hotkey 
        /// and checking it's modifier and key to see if they match. is so, it removes it.
        /// </summary>
        /// <param name="modifier"></param>
        /// <param name="key"></param>
        /// <param name="removeAllOccourances">
        /// If this is false, the first found hotkey will be removed. 
        /// else, every occourance will be removed.
        /// </param>
        public static void RemoveHotkey(ModifierKeys modifier, Key[] keys, bool removeAllOccourances = false)
        {
            List<GlobalHotkey> originalHotkeys = Hotkeys;
            List<GlobalHotkey> toBeRemoved = FindHotkeys(modifier, keys);

            if (toBeRemoved.Count > 0)
            {
                if (removeAllOccourances)
                {
                    foreach (GlobalHotkey hotkey in toBeRemoved)
                    {
                        originalHotkeys.Remove(hotkey);
                    }

                    Hotkeys = originalHotkeys;
                }
                else
                {
                    RemoveHotkey(toBeRemoved[0]);
                }
            }
        }

        /// <summary>
        /// Sets up the Key Up/Down event hooks.
        /// </summary>
        /// <param name="proc">The callback method to be called when a key up/down occours</param>
        /// <returns></returns>
        private static IntPtr SetHook(LowLevelKeyboardProc proc)
        {
            using (Process curProcess = Process.GetCurrentProcess())
            {
                using (ProcessModule curModule = curProcess.MainModule)
                {
                    return SetWindowsHookEx(WH_KEYBOARD_LL, proc, GetModuleHandle(curModule.ModuleName), 0);
                }
            }
        }

        /// <summary>
        /// The method called when a key up/down occours across the system.
        /// </summary>
        /// <param name="nCode">idk tbh</param>
        /// <param name="lParam">LPARAM, contains the key that was pressed. not used atm</param>
        /// <returns>LRESULT</returns>
        private static IntPtr HookCallback(int nCode, IntPtr wParam, IntPtr lParam)
        {
            // Checks if this is called from keydown only because key ups aren't used.
            if (nCode >= 0)
            {
                CheckHotkeys();

                // Cannot use System.Windows' keys because
                // they dont use the same values as windows
                //int vkCode = Marshal.ReadInt32(lParam);
                //System.Windows.Forms.Keys key = (System.Windows.Forms.Keys)vkCode;
                //Debug.WriteLine(key);
            }

            // I think this tells windows that this app has successfully
            // handled the key events and now other apps can handle it next.
            return CallNextHookEx(HookID, nCode, wParam, lParam);
        }

        #region Native Methods

        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr SetWindowsHookEx(int idHook, LowLevelKeyboardProc lpfn, IntPtr hMod, uint dwThreadId);

        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool UnhookWindowsHookEx(IntPtr hhk);

        [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr CallNextHookEx(IntPtr hhk, int nCode, IntPtr wParam, IntPtr lParam);

        [DllImport("kernel32.dll", CharSet = CharSet.Auto, SetLastError = true)]
        private static extern IntPtr GetModuleHandle(string lpModuleName);

        #endregion
    }
}
