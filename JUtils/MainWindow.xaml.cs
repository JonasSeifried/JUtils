using JUtils.model;
using JUtils.model.hotkeys;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Interop;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace JUtils
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        Controller controller;
        public MainWindow()
        {
            InitializeComponent();

            controller = new Controller();
        }


        private void btnRun_Click(object sender, RoutedEventArgs e)
        {
            LVhotkeys.Items.Clear();
            controller.getHotkeysAsStrings().ForEach(s => { LVhotkeys.Items.Add(s); });
            hkp_MicToggle.Keys.ForEach(s =>  LVhotkeys.Items.Add(s));
        }

        private void MainWindow_Closing(object sender, System.ComponentModel.CancelEventArgs e) 
        {
            controller.ShutdownHotkeySystemHook();
        }
    }
}
