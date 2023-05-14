﻿using System;
using System.Collections.Generic;
using System.Configuration;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Input;

namespace JUtils.model.hotkeys
{
    /// <summary>
    /// A hotkey containing a key combination (a key and modifier key) and a
    /// callback function that gets called if the right keys are down. Also contains a
    /// boolean stating if the callback method can be called or not.
    /// </summary>
    public class GlobalHotkey
    {

        /// <summary>
        /// The keys required to be pressed for the hotkey to be fired
        /// </summary>
        public Key[] Keys { get; set; }

        /// <summary>
        /// hotkey pressed state
        /// </summary>
        public bool pressed { get; set; }

        // You could change this to a list of actions if you want
        // multiple things to be fired when the hotkey fires.
        /// <summary>
        /// The method to be called when the hotkey is pressed
        /// </summary>
        public Action Callback { get; set; }

        /// <summary>
        /// States if the method can be executed.
        /// </summary>
        public bool CanExecute { get; set; }

        /// <summary>
        /// Initiates a new hotkey with the given modifier, key, callback method, 
        /// and also a boolean stating if the callback can be run (can be changed, see <see cref="CanExecute"/>)
        /// </summary>
        /// <param name="modifier">The modifier key required to be pressed</param>
        /// <param name="key">The key required to be pressed</param>
        /// <param name="callbackMethod">The method that gets called when the hotkey is fired</param>
        /// <param name="canExecute">
        /// States whether the callback can be run 
        /// (can be changed, see <see cref="CanExecute"/>)
        /// </param>
        public GlobalHotkey(Key[] keys, Action callbackMethod, bool canExecute = true)
        {
            this.Keys = keys;
            this.Callback = callbackMethod;
            this.CanExecute = canExecute;
        }
    }
}
