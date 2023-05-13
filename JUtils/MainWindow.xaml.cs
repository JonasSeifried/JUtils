using JUtils.model;
using JUtils.model.hotkeys;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Interop;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        Controller controller;
        public MainWindow()
        {
            InitializeComponent();

            controller = new Controller();
        }


        private void btnSave_Click(object sender, RoutedEventArgs e)
        {
            if (hkp_MicToggle.Valid)
            {
                if(hkp_MicToggle.Keys.Count == 0) 
                {
                    controller.RemoveHotkey(Controller.Hotkeys.ToggleMic);
                    MessageBox.Show("Removed Hotkey MicToggle");
                } else
                {
                    controller.AddHotkey(Controller.Hotkeys.ToggleMic, hkp_MicToggle.Keys.ToArray());
                    MessageBox.Show("Added Hotkey MicToggle: " + controller.HotkeyToString(Controller.Hotkeys.ToggleMic));
                }

            }
        }

        private void MainWindow_Closing(object sender, System.ComponentModel.CancelEventArgs e) 
        {
            controller.ShutdownHotkeySystemHook();
        }
    }
}
