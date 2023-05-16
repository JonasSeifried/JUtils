using System.Collections.Generic;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;

namespace JUtils.view
{
    /// <summary>
    /// Interaction logic for HotkeyPicker.xaml
    /// </summary>
    public partial class HotkeyPicker
    {
        public List<Key> Keys { get; } = new();

        public bool Valid { get; private set; }


        public HotkeyPicker()
        {
            InitializeComponent();
        }

        private void TbHotkeyPicker_PreviewKeyDown(object sender, KeyEventArgs e)
        {

            //Because for leftAlt and f10 e.Key equals Key.System. But e.SystemKey then gives the right Key
            Key key = (e.Key == Key.System ? e.SystemKey : e.Key);
            string keyString = Controller.PrettyKeys[key];

            if (Valid)
            {
                Valid = false;
                TbHotkeyPicker.Text = string.Empty;
                Keys.Clear();
            }
            if (Keys.Contains(e.Key))
            {
                e.Handled = true;
                return;
            }      
            if (string.IsNullOrEmpty(TbHotkeyPicker.Text)) TbHotkeyPicker.Text = keyString;
            else TbHotkeyPicker.Text = TbHotkeyPicker.Text + " + " + keyString;
            Keys.Add(key);
            
            e.Handled = true;
        }

        private void TbHotkeyPicker_PreviewKeyUp(object sender, KeyEventArgs e) => Valid = true;

        private void BtnClear_Click(object sender, RoutedEventArgs e)
        {
            TbHotkeyPicker.Clear();
            Valid = true;
            Keys.Clear();
            TbHotkeyPicker.Focus();
        }

        private void TbHotkeyPicker_TextChanged(object sender, TextChangedEventArgs e)
        {
            TbPlaceholder.Visibility = string.IsNullOrEmpty(TbHotkeyPicker.Text)
                ? Visibility.Visible
                : Visibility.Hidden;
        }
    }
}
