using NAudio.CoreAudioApi;
using NAudio.Wave;

namespace JUtils.model
{
    public static class MicMute
    {
        private static WaveOutEvent? _outputDevice;
        static AudioFileReader? _audioFile;
        private static bool MicMuted { get; set; }
        private const string SoundMicActivated = "C:\\Users\\Jonas\\Documents\\Github\\JUtils\\JUtils\\mic_activated.wav";
        private const string SoundMicMuted = "C:\\Users\\Jonas\\Documents\\Github\\JUtils\\JUtils\\mic_muted.wav";


        public static void ToggleMic()
        {
            MicMuted = !MicMuted;
            MMDeviceEnumerator enumerator = new MMDeviceEnumerator();
            MMDevice commDevice = enumerator.GetDefaultAudioEndpoint(DataFlow.Capture, Role.Communications);
            commDevice.AudioEndpointVolume.Mute = MicMuted;
            
            if (_outputDevice != null)
            {
                _outputDevice.Dispose();
                _outputDevice = null;
            }
            if(_audioFile != null)
            {
                _audioFile.Dispose();
                _audioFile = null;
            }
            _outputDevice = new WaveOutEvent();
            _outputDevice.PlaybackStopped += OnPlaybackStopped;

            _audioFile = MicMuted ? new AudioFileReader(SoundMicMuted) : new AudioFileReader(SoundMicActivated);
            _outputDevice.Init(_audioFile);
            
            _outputDevice.Play();

        }

        private static void OnPlaybackStopped(object? sender, StoppedEventArgs e)
        {

        }
    }
}
