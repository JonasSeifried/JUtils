using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace JUtils.view
{
    /// <summary>
    /// Interaction logic for HotkeyPicker.xaml
    /// </summary>
    public partial class HotkeyPicker : UserControl
    {
        bool hotkeySet = false;

        private List<Key> keys = new();

        public List<Key> Keys
        {
            get { return keys; }
            set { keys = value; }
        }

        public HotkeyPicker()
        {
            InitializeComponent();
        }

        private void tbHotkeyPicker_PreviewKeyDown(object sender, KeyEventArgs e)
        {
            if(hotkeySet)
            {
                hotkeySet = false;
                tbHotkeyPicker.Text = string.Empty;
                keys.Clear();
            }
            if (keys.Contains(e.Key))
            {
                e.Handled = true;
                return;
            }
            if (string.IsNullOrEmpty(tbHotkeyPicker.Text)) tbHotkeyPicker.Text = e.Key.ToString();
            else tbHotkeyPicker.Text = tbHotkeyPicker.Text + " + " + e.Key.ToString();
            keys.Add(e.Key);
            
            e.Handled = true;
        }

        private void tbHotkeyPicker_PreviewKeyUp(object sender, KeyEventArgs e)
        {
            hotkeySet = true;
        }

        private void btnClear_Click(object sender, RoutedEventArgs e)
        {
            tbHotkeyPicker.Clear();
            hotkeySet = false;
            keys.Clear();
            tbHotkeyPicker.Focus();
        }

        private void tbHotkeyPicker_TextChanged(object sender, TextChangedEventArgs e)
        {
            if (string.IsNullOrEmpty(tbHotkeyPicker.Text)) tbPlaceholder.Visibility = Visibility.Visible;
            else tbPlaceholder.Visibility = Visibility.Hidden;
        }
    }
}
