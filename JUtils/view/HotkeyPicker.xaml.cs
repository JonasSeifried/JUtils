using System;
using System.Collections.Generic;
using System.Diagnostics;
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
        

        private List<Key> keys = new();

        public List<Key> Keys
        {
            get { return keys; }
        }

        private bool valid;

        public bool Valid
        {
            get { return valid; }
        }


        public HotkeyPicker()
        {
            InitializeComponent();
        }

        private void tbHotkeyPicker_PreviewKeyDown(object sender, KeyEventArgs e)
        {

            //Because for leftAlt and f10 e.Key equals Key.System. But e.SystemKey then gives the right Key
            Key key = (e.Key == Key.System ? e.SystemKey : e.Key);
            string keyString = Controller.PrettyKeys[key];

            if (valid)
            {
                valid = false;
                tbHotkeyPicker.Text = string.Empty;
                keys.Clear();
            }
            if (keys.Contains(e.Key))
            {
                e.Handled = true;
                return;
            }      
            if (string.IsNullOrEmpty(tbHotkeyPicker.Text)) tbHotkeyPicker.Text = keyString;
            else tbHotkeyPicker.Text = tbHotkeyPicker.Text + " + " + keyString;
            keys.Add(key);
            
            e.Handled = true;
        }

        private void tbHotkeyPicker_PreviewKeyUp(object sender, KeyEventArgs e)
        {
            valid = true;
        }

        private void btnClear_Click(object sender, RoutedEventArgs e)
        {
            tbHotkeyPicker.Clear();
            valid = true;
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
