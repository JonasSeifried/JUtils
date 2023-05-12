using NAudio.CoreAudioApi;
using NAudio.Wave;
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
        static WaveOutEvent? outputDevice;
        static AudioFileReader? audioFile;
        public static bool MicMuted { get; private set; }
        const string MIC_ACTIVATED = "C:\\Users\\Jonas\\Documents\\Github\\JUtils\\JUtils\\mic_activated.wav";
        const string MIC_MUTED = "C:\\Users\\Jonas\\Documents\\Github\\JUtils\\JUtils\\mic_muted.wav";


        public static void ToggleMic()
        {
            MicMuted = !MicMuted;
            MMDeviceEnumerator enumerator = new MMDeviceEnumerator();
            MMDevice commDevice = enumerator.GetDefaultAudioEndpoint(DataFlow.Capture, Role.Communications);
            commDevice.AudioEndpointVolume.Mute = MicMuted;
            
            if (outputDevice != null)
            {
                outputDevice?.Dispose();
                outputDevice = null;
            }
            if(audioFile != null)
            {
                audioFile?.Dispose();
                audioFile = null;
            }
            outputDevice = new WaveOutEvent();
            outputDevice.PlaybackStopped += OnPlaybackStopped;

            if(MicMuted)
            {
            audioFile = new AudioFileReader(MIC_MUTED);
            } else
            {
                audioFile = new AudioFileReader(MIC_ACTIVATED);
            }
            outputDevice.Init(audioFile);
            
            outputDevice.Play();

        }

        private static void OnPlaybackStopped(object? sender, StoppedEventArgs e)
        {

        }
    }
}
