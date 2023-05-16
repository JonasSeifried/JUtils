using System.Windows;
using System.Windows.Input;
namespace JUtils.view
{
    /// <summary>
    /// Interaction logic for TitleBar.xaml
    /// </summary>
    public partial class TitleBar
    {
        public TitleBar()
        {
            InitializeComponent();
        }

        private void btnMinimize_Click(object sender, RoutedEventArgs e)
        {
            Window.GetWindow(this)?.Close();
        }

        private void UserControl_MouseLeftButtonDown(object sender, MouseButtonEventArgs e)
        {
            Window.GetWindow(this)?.DragMove();
        }

        private void btnMaximize_Click(object sender, RoutedEventArgs e)
        {
            var parentWin = Window.GetWindow(this);
            if (parentWin == null) return;
            parentWin.WindowState = parentWin.WindowState == WindowState.Normal
                ? WindowState.Maximized
                : WindowState.Normal;
        }
    }
}
