
using System;

using System.Windows;
using System.Windows.Input;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public Controller Controller { get; private set; }
        public MainWindow()
        {
            InitializeComponent();

            Controller = new Controller();
        }


        private void btnSave_Click(object sender, RoutedEventArgs e)
        {
            if (hkp_MicToggle.Valid)
            {
                if(hkp_MicToggle.Keys.Count == 0) 
                {
                    Controller.RemoveHotkey(Controller.Hotkeys.MicToggle);
                    MessageBox.Show("Removed Hotkey MicToggle");
                } else
                {
                    Controller.AddHotkey(Controller.Hotkeys.MicToggle, hkp_MicToggle.Keys.ToArray());
                    MessageBox.Show("Added Hotkey MicToggle: " + Controller.HotkeyToString(Controller.Hotkeys.MicToggle));
                }

            }
        }

        private void MainWindow_Closing(object sender, System.ComponentModel.CancelEventArgs e) 
        {
            Controller.ShutdownHotkeySystemHook();
        }

        // Minimize to system tray when application is minimized.
        protected override void OnStateChanged(EventArgs e)
        {
            if (IsVisible && WindowState == WindowState.Minimized) this.Hide();

            base.OnStateChanged(e);
        }

        // Minimize to system tray when application is closed.
        protected override void OnClosing(System.ComponentModel.CancelEventArgs e)
        {
            // setting cancel to true will cancel the close request
            // so the application is not closed
            e.Cancel = true;

            this.Hide();
            WindowState = WindowState.Minimized;

            base.OnClosing(e);
        }
    }
}
