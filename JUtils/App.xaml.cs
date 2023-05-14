using System;
using System.Collections.Generic;
using System.Configuration;
using System.Data;
using System.Linq;
using System.Threading.Tasks;
using System.Windows;
using Forms = System.Windows.Forms;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : Application
    {
        private readonly Forms.NotifyIcon _notifyIcon;

        public App()
        {
            _notifyIcon = new Forms.NotifyIcon();
        }

        protected override void OnStartup(StartupEventArgs e)
        {
            MainWindow = new MainWindow();
            MainWindow.Show();

            setupSystemTray();

            Controller.Instance.RestoreHotkeys();

            base.OnStartup(e);
        }

        private void setupSystemTray()
        {
            _notifyIcon.Icon = new System.Drawing.Icon("Resources/SystemTrayIcon.ico");
            _notifyIcon.Text = "JUtils";
            _notifyIcon.MouseClick += _notifyIcon_MouseClick;
            _notifyIcon.ContextMenuStrip = new Forms.ContextMenuStrip();
            _notifyIcon.ContextMenuStrip.Items.Add("Open", null, (object? sender, EventArgs e) => openMainWindow());
            _notifyIcon.ContextMenuStrip.Items.Add("Exit", null, (object? sender, EventArgs e) => Shutdown());
            _notifyIcon.Visible = true;
        }

        private void openMainWindow()
        {
            MainWindow.WindowState = WindowState.Normal;
            MainWindow.Show();
            MainWindow.Activate();
        }

        private void _notifyIcon_MouseClick(object? sender, Forms.MouseEventArgs e)
        {
            if (e.Button == Forms.MouseButtons.Right) return;
            openMainWindow();
        }

        protected override void OnExit(ExitEventArgs e)
        {
            Controller.Instance.StoreHotkeys();
            _notifyIcon.Dispose();
            base.OnExit(e);
        }
    }
}
