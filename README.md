![Build status](https://github.com/rageagainsthepc/ampsim-launcher/actions/workflows/ci.yml/badge.svg)

# AmpSim Launcher

*AmpSim Launcher* boosts audio performance of standalone amp emulators on *Windows* by switching to the *High Performance* power plan as well as elevating the process priority.
This means that the buffer size for the audio interface can be reduced significantly.
Though this is its primary function, *AmpSim Launcher* can be used to boost other applications (e.g. games) as well.
As soon as the application is closed the original power plan will be restored.

## Usage

- Place the binary somewhere on your system (e.g. `C:\Program Files\AmpSimLauncher`).
- Launch a terminal window (e.g. `powershell.exe`) and navigate to the directory where the *AmpSim Launcher* binary is stored. Alternatively, you can type `powershell` into the address bar of your explorer window which will start a `powershell` instance in the current directory.
- Run `ampsim-launcher.exe link '<path\to\your\ampsim.exe>' '<C:\Users\YOUR USER\Desktop\Shortcut Name>'` for each individual amp emulator. This will create a shortcut which launches *AmpSim Launcher* with the correct parameters.

You could also launch a program directly by running `ampsim-launcher.exe launch <path\to\your\ampsim.exe>`.

## Building

- Install *MSV C++ Build Tools* and *Rust*
- Run `cargo build`

## Caveats

- Since the released binary is not digitally signed *Windows* will display a warning upon executing it for the first time. If you have security concerns you can upload the binary to <https://virustotal.com> which will scan it with a multitude of different
AV products. Of course it is also possible to simply compile the source code manually.
- On termination the power plan that was active when the application was started will be restored. This also means that launching multiple instances of *AmpSim Launcher* simultaneously will probably result in the wrong power plan being restored.
