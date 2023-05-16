using System.Windows;
using Forms = System.Windows.Forms;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App
    {
        private readonly Forms.NotifyIcon _notifyIcon;

        public App()
        {
            _notifyIcon = new Forms.NotifyIcon();
        }

        protected override void OnStartup(StartupEventArgs e)
        {
            Controller.Instance.RestoreHotkeys();
            
            MainWindow = new MainWindow();
            MainWindow.Show();

            SetupSystemTray();


            base.OnStartup(e);
        }

        private void SetupSystemTray()
        {
            _notifyIcon.Icon = new System.Drawing.Icon("Resources/SystemTrayIcon.ico");
            _notifyIcon.Text = @"JUtils";
            _notifyIcon.MouseClick += _notifyIcon_MouseClick;
            _notifyIcon.ContextMenuStrip = new Forms.ContextMenuStrip();
            _notifyIcon.ContextMenuStrip.Items.Add("Open", null, (_, _) => OpenMainWindow());
            _notifyIcon.ContextMenuStrip.Items.Add("Exit", null, (_, _) => Shutdown());
            _notifyIcon.Visible = true;
        }

        private void OpenMainWindow()
        {
            MainWindow ??= new MainWindow();
            MainWindow.WindowState = WindowState.Normal;
            MainWindow.Show();
            MainWindow.Activate();
        }

        private void _notifyIcon_MouseClick(object? sender, Forms.MouseEventArgs e)
        {
            if (e.Button == Forms.MouseButtons.Right) return;
            OpenMainWindow();
        }

        protected override void OnExit(ExitEventArgs e)
        {
            Controller.Instance.StoreHotkeys();
            Controller.Instance.ShutdownHotkeySystemHook();
            _notifyIcon.Dispose();
            base.OnExit(e);
        }
    }
}
