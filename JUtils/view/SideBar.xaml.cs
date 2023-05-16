using System.Diagnostics;
using System.Windows.Navigation;

namespace JUtils.view;

public partial class SideBar
{
    public SideBar()
    {
        InitializeComponent();
    }

    private void Hyperlink_OnRequestNavigate(object sender, RequestNavigateEventArgs e)
    {
        Process.Start(new ProcessStartInfo("cmd", $"/c start {e.Uri.AbsoluteUri}") { CreateNoWindow = true });
        e.Handled = true;
    }
}