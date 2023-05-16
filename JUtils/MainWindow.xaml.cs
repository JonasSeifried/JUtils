
using System;

using System.Windows;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow
    {
        public MainWindow()
        {
            InitializeComponent();
            
        }


        private void btnSave_Click(object sender, RoutedEventArgs e)
        {
            if (HkpMicToggle.Valid)
            {
                if(HkpMicToggle.Keys.Count == 0) 
                {
                    Controller.Instance.RemoveHotkey(Controller.Hotkeys.MicToggle);
                    MessageBox.Show("Removed Hotkey MicToggle");
                } else
                {
                    Controller.Instance.AddHotkey(Controller.Hotkeys.MicToggle, HkpMicToggle.Keys.ToArray());
                    MessageBox.Show("Added Hotkey MicToggle: " + Controller.Instance.HotkeyToString(Controller.Hotkeys.MicToggle));
                }

            }
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
