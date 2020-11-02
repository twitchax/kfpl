# kfpl

[![Actions Status](https://github.com/twitchax/kfpl/workflows/build/badge.svg)](https://github.com/twitchax/kfpl/actions)
[![GitHub Release](https://img.shields.io/github/release/twitchax/kfpl.svg)](https://github.com/twitchax/kfpl/releases)

Automates running KubeFlow Pipelines (KFP) locally.

## Information

### Install

```bash
curl -LO https://github.com/twitchax/kfpl/releases/download/v1.2.0/kfpl
chmod +x ./kfpl
sudo mv ./kfpl /usr/local/bin/kfpl
```

### Test

No.

### Compatibility

Ubuntu.

### Examples

Help output.

```bash
kfpl 1.0
Aaron Roney
Automates running KubeFlow Pipelines (KFP) locally.

USAGE:
    kfpl [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -y, --yes        Answers all of the prompts with 'yes', resulting in a no-touch execution.
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    init       Ensures the dependencies are met (may need to be run as sudo).
    service    Commands to interact with the k3d cluster, and the KFP service.
    ui         Starts the port forwarding to the KFP UI via `kubectl`.
```

Basic usage.

```bash
kfpl -y init
kfpl -y service start
kfpl -y ui
```

## License

```
The MIT License (MIT)

Copyright (c) 2020 Aaron Roney

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
