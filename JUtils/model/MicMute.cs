using NAudio.CoreAudioApi;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Interop;

namespace JUtils.model
{
    internal class MicMute
    {


        public static void ToggleMic()
        {
            MMDeviceEnumerator enumerator = new MMDeviceEnumerator();
            MMDevice commDevice = enumerator.GetDefaultAudioEndpoint(DataFlow.Capture, Role.Communications);
            commDevice.AudioEndpointVolume.Mute = !commDevice.AudioEndpointVolume.Mute;

        }
    }
}
