![Build status](https://github.com/rageagainsthepc/ampsim-launcher/actions/workflows/ci.yml/badge.svg)

# AmpSim Launcher

*AmpSim Launcher* boosts audio performance of standalone amp emulators on *Windows* by switching to the *High Performance* power plan as well as elevating the process priority.
This means that the buffer size for the audio interface can be reduced significantly.
Though this is its primary function, *AmpSim Launcher* can be used to boost other applications (e.g. games) as well.
As soon as the application is closed the original power plan will be restored.

## Usage

This tool offers an interactive as well as a CLI mode. If you are not familiar using consoles (e.g. `powershell.exe`, `cmd.exe`)
you might be better off using the interactive mode.

### Interactive

- Place the binary somewhere on your system (e.g. `C:\Program Files\AmpSimLauncher\ampsim-launcher.exe`).
- Start the binary via double click. A console window will open in order to guide you through the process of creating a shortcut.
The shortcut will be configured to execute *AmpSim Launcher* with the correct parameters for your specific target program.
- Repeat the shortcut creation step for each program that should be launched via *AmpSim Launcher*.

#### Tips

- Dragging and dropping binaries into the console window will automatically enter their respective file path.
- If you decide to relocate the *AmpSim Launcher* binary you will have to re-generate all shortcuts.

### CLI

Advanced users may also use this tool from a console window. Currently, there are two subcommands available:

- `launch` for executing a target process
- `link` for creating a shortcut

You can get a more detailed description by running `ampsim-launcher.exe --help`.

## Building

- Install *MSV C++ Build Tools* and *Rust*
- Run `cargo build`

## Caveats

- Since the released binary is not digitally signed *Windows* will display a warning upon executing it for the first time. If you have security concerns you can upload the binary to <https://virustotal.com> which will scan it with a multitude of different
AV products. Of course it is also possible to simply compile the source code manually.
- On termination the power plan that was active when the application was started will be restored. This also means that launching multiple instances of *AmpSim Launcher* simultaneously will probably result in the wrong power plan being restored.
