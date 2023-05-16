
using System;
using System.Windows;
using System.Windows.Media;

namespace JUtils
{
    public partial class MainWindow
    {
        public MainWindow()
        {
            InitializeComponent();
            TbCurrently.Text = "Currently " + Controller.Instance.HotkeyToString(Controller.Hotkeys.MicToggle);

        }


        private void btnSave_Click(object sender, RoutedEventArgs e)
        {
            if (HkpMicToggle.Valid)
            {
                if(HkpMicToggle.Keys.Count == 0) 
                {
                    Controller.Instance.RemoveHotkey(Controller.Hotkeys.MicToggle);
                    TbMsg.Text = "Removed Hotkey MicToggle";
                    TbMsg.Foreground = Brushes.Red;
                    TbMsg.Visibility = Visibility.Visible;
                    TbCurrently.Text = "Currently None";
                } else
                {
                    Controller.Instance.AddHotkey(Controller.Hotkeys.MicToggle, HkpMicToggle.Keys.ToArray());
                    TbMsg.Text = "Added Hotkey MicToggle: " +
                                 Controller.Instance.HotkeyToString(Controller.Hotkeys.MicToggle);
                    TbMsg.Foreground = Brushes.Green;
                    TbMsg.Visibility = Visibility.Visible;
                    TbCurrently.Text = "Currently " + Controller.Instance.HotkeyToString(Controller.Hotkeys.MicToggle);
                }

            }
        }
        // Minimize to system tray when application is minimized.
        protected override void OnStateChanged(EventArgs e)
        {
            if (IsVisible && WindowState == WindowState.Minimized)
            {
                TbCurrently.Visibility = Visibility.Hidden;
                this.Hide();
            }

            base.OnStateChanged(e);
        }

        // Minimize to system tray when application is closed.
        protected override void OnClosing(System.ComponentModel.CancelEventArgs e)
        {

            this.Hide();
            TbCurrently.Visibility = Visibility.Hidden;
            WindowState = WindowState.Minimized;

            e.Cancel = true;
            base.OnClosing(e);
        }
    }
}
